use anyhow::{Result, Context};
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};
use std::path::Path;
use std::sync::Arc;
use std::collections::HashMap;
use parking_lot::RwLock;
use serde_json::json;

// Generate bindings from WIT
wasmtime::component::bindgen!({
    world: "plugin-world",
    path: "../wit",
    async: false,
});

// Import the generated types using the correct module path
use self::exports::taxtalk::plugin::plugin::Guest;

/// Host state that implements the store and http capabilities
pub struct HostState {
    pub store: Arc<RwLock<HashMap<String, String>>>,
    pub plugin_id: String,
    pub wasi_ctx: WasiCtx,
    pub table: wasmtime_wasi::ResourceTable,
}

impl HostState {
    pub fn new(plugin_id: String) -> Self {
        // Create a minimal WASI context (no filesystem access, no environment vars)
        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdio()  // Allow basic stdio for debugging
            .build();
            
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            plugin_id,
            wasi_ctx,
            table: wasmtime_wasi::ResourceTable::new(),
        }
    }
}

// Implement WasiView for HostState
impl WasiView for HostState {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        &mut self.table
    }
    
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

// Implement the store host interface
impl self::taxtalk::plugin::store::Host for HostState {
    fn get(&mut self, key: String) -> Option<String> {
        let full_key = format!("{}:{}", self.plugin_id, key);
        self.store.read().get(&full_key).cloned()
    }

    fn set(&mut self, key: String, value: String) -> Result<(), String> {
        let full_key = format!("{}:{}", self.plugin_id, key);
        self.store.write().insert(full_key, value);
        Ok(())
    }

    fn delete(&mut self, key: String) -> Result<(), String> {
        let full_key = format!("{}:{}", self.plugin_id, key);
        self.store.write().remove(&full_key);
        Ok(())
    }

    fn list_keys(&mut self, prefix: String) -> Vec<String> {
        let full_prefix = format!("{}:{}", self.plugin_id, prefix);
        self.store
            .read()
            .keys()
            .filter(|k| k.starts_with(&full_prefix))
            .map(|k| k.strip_prefix(&format!("{}:", self.plugin_id)).unwrap_or(k).to_string())
            .collect()
    }
}

// Implement the http host interface
impl self::taxtalk::plugin::http::Host for HostState {
    fn fetch(&mut self, req: self::taxtalk::plugin::http::Request) -> Result<self::taxtalk::plugin::http::Response, String> {
        // For now, return a stub response
        // In production, this would make actual HTTP requests
        println!("[Plugin {}] HTTP request: {} {}", self.plugin_id, req.method, req.url);
        
        Ok(self::taxtalk::plugin::http::Response {
            status: 200,
            headers: vec![],
            body: json!({
                "message": "HTTP not implemented in sandbox",
                "request_url": req.url
            }).to_string(),
        })
    }
}

/// Plugin instance that wraps a WASM component
pub struct WasmPlugin {
    pub id: String,
    store: Store<HostState>,
    instance: PluginWorld,
    _component: Component,
}

impl WasmPlugin {
    /// Load a plugin from a WASM file
    pub fn load(engine: &Engine, plugin_id: &str, wasm_path: &Path) -> Result<Self> {
        // Load the component
        let component = Component::from_file(&engine, wasm_path)
            .with_context(|| format!("Failed to load WASM component from {:?}", wasm_path))?;
        
        // Create store with host state
        let mut store = Store::new(&engine, HostState::new(plugin_id.to_string()));
        
        // Create linker and add host implementations
        let mut linker = Linker::new(&engine);
        
        // Add WASI support
        wasmtime_wasi::add_to_linker_sync(&mut linker)?;
        
        // Add the store interface
        self::taxtalk::plugin::store::add_to_linker(&mut linker, |state: &mut HostState| state)?;
        
        // Add the http interface  
        self::taxtalk::plugin::http::add_to_linker(&mut linker, |state: &mut HostState| state)?;
        
        // Instantiate the component
        let instance = PluginWorld::instantiate(&mut store, &component, &linker)
            .with_context(|| format!("Failed to instantiate plugin {}", plugin_id))?;
        
        Ok(Self {
            id: plugin_id.to_string(),
            store,
            instance,
            _component: component,
        })
    }
    
    /// Get plugin manifest
    pub fn get_manifest(&mut self) -> Result<self::taxtalk::plugin::types::PluginManifest> {
        let exports = self.instance.taxtalk_plugin_plugin();
        exports.call_get_manifest(&mut self.store)
            .with_context(|| format!("Failed to get manifest for plugin {}", self.id))
    }
    
    /// Execute a command
    pub fn execute(&mut self, command: &str, tokens: Vec<self::taxtalk::plugin::types::Token>) -> Result<self::taxtalk::plugin::types::CommandResult> {
        let exports = self.instance.taxtalk_plugin_plugin();
        exports.call_execute(&mut self.store, command, &tokens)
            .with_context(|| format!("Failed to execute command on plugin {}", self.id))
    }
    
    /// Get plugin schema
    pub fn get_schema(&mut self) -> Result<String> {
        let exports = self.instance.taxtalk_plugin_plugin();
        exports.call_get_schema(&mut self.store)
            .with_context(|| format!("Failed to get schema for plugin {}", self.id))
    }
    
    /// Get plugin state (if stateful)
    pub fn get_state(&mut self) -> Result<Option<String>> {
        let exports = self.instance.taxtalk_plugin_plugin();
        exports.call_get_state(&mut self.store)
            .with_context(|| format!("Failed to get state for plugin {}", self.id))
    }
    
    /// Set plugin state (if stateful)
    pub fn set_state(&mut self, state: &str) -> Result<()> {
        let exports = self.instance.taxtalk_plugin_plugin();
        exports.call_set_state(&mut self.store, state)
            .map_err(|e| anyhow::anyhow!("Failed to set state: {}", e))?;
        Ok(())
    }
}

/// Plugin manager that loads and manages multiple plugins
pub struct PluginManager {
    engine: Engine,
    plugins: HashMap<String, WasmPlugin>,
    shared_store: Arc<RwLock<HashMap<String, String>>>,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.async_support(false);
        
        let engine = Engine::new(&config)?;
        
        Ok(Self {
            engine,
            plugins: HashMap::new(),
            shared_store: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Load a plugin
    pub fn load_plugin(&mut self, plugin_id: &str, wasm_path: &Path) -> Result<()> {
        println!("Loading plugin '{}' from {:?}", plugin_id, wasm_path);
        
        let mut plugin = WasmPlugin::load(&self.engine, plugin_id, wasm_path)?;
        
        // Get and display manifest
        let manifest = plugin.get_manifest()?;
        println!("  Plugin loaded: {} v{}", manifest.name, manifest.version);
        println!("  Description: {}", manifest.description);
        println!("  Commands: {}", manifest.commands.len());
        
        self.plugins.insert(plugin_id.to_string(), plugin);
        
        Ok(())
    }
    
    /// Execute a command on a specific plugin
    pub fn execute_on_plugin(
        &mut self, 
        plugin_id: &str, 
        command: &str, 
        tokens: Vec<self::taxtalk::plugin::types::Token>
    ) -> Result<self::taxtalk::plugin::types::CommandResult> {
        let plugin = self.plugins.get_mut(plugin_id)
            .ok_or_else(|| anyhow::anyhow!("Plugin '{}' not found", plugin_id))?;
        
        plugin.execute(command, tokens)
    }
    
    /// Get a plugin's manifest
    pub fn get_plugin_manifest(&mut self, plugin_id: &str) -> Result<self::taxtalk::plugin::types::PluginManifest> {
        let plugin = self.plugins.get_mut(plugin_id)
            .ok_or_else(|| anyhow::anyhow!("Plugin '{}' not found", plugin_id))?;
        
        plugin.get_manifest()
    }
    
    /// List all loaded plugins
    pub fn list_plugins(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }
    
    /// Get shared store for debugging
    pub fn get_store_contents(&self) -> HashMap<String, String> {
        self.shared_store.read().clone()
    }
}

// Re-export the generated types for easier use
pub use self::taxtalk::plugin::types::{Token, CommandResult, PluginManifest, CommandDef};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert!(manager.is_ok());
    }
    
    #[test]
    fn test_host_state() {
        let mut state = HostState::new("test".to_string());
        
        // Test store operations
        assert!(state.set("key1".to_string(), "value1".to_string()).is_ok());
        assert_eq!(state.get("key1".to_string()), Some("value1".to_string()));
        
        assert!(state.delete("key1".to_string()).is_ok());
        assert_eq!(state.get("key1".to_string()), None);
        
        // Test list_keys
        let _ = state.set("test:1".to_string(), "a".to_string());
        let _ = state.set("test:2".to_string(), "b".to_string());
        let _ = state.set("other:1".to_string(), "c".to_string());
        
        let keys = state.list_keys("test:".to_string());
        assert_eq!(keys.len(), 2);
    }
}