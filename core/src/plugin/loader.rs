use std::path::Path;
use std::sync::Arc;
use anyhow::{Error, anyhow};
use wasmtime::{Engine, Module, Store, Linker};
use serde_json::Value;

use crate::plugin::manifest::{PluginManifest, PluginType};
use crate::plugin::types::{LoadedPlugin, PluginInstance, PluginState};
use crate::plugin::traits::Plugin;

/// Loads plugins from various sources
pub struct PluginLoader {
    engine: Engine,
    native_plugins: Vec<Box<dyn Fn() -> Box<dyn Plugin> + Send + Sync>>,
}

impl PluginLoader {
    pub fn new() -> Result<Self, Error> {
        let engine = Engine::default();
        
        Ok(Self {
            engine,
            native_plugins: Vec::new(),
        })
    }
    
    /// Register a native plugin factory
    pub fn register_native_factory<F>(&mut self, factory: F)
    where
        F: Fn() -> Box<dyn Plugin> + Send + Sync + 'static,
    {
        self.native_plugins.push(Box::new(factory));
    }
    
    /// Load a plugin from a directory
    pub async fn load_plugin(&self, path: &Path) -> Result<Arc<LoadedPlugin>, Error> {
        // Load manifest
        let manifest_path = path.join("manifest.json");
        let manifest = PluginManifest::from_file(&manifest_path)?;
        
        // Validate dependencies
        self.validate_dependencies(&manifest)?;
        
        // Load configuration
        let config = self.load_config(path)?;
        
        // Create plugin instance based on type
        let instance = match manifest.plugin_type {
            PluginType::Native => {
                self.load_native_plugin(path, &manifest).await?
            }
            PluginType::Wasm => {
                self.load_wasm_plugin(path, &manifest).await?
            }
        };
        
        // Create loaded plugin
        let loaded = Arc::new(LoadedPlugin {
            id: manifest.id.clone(),
            instance,
            manifest,
            state: PluginState::new(),
            config,
        });
        
        Ok(loaded)
    }
    
    /// Load a native Rust plugin
    async fn load_native_plugin(
        &self,
        path: &Path,
        manifest: &PluginManifest,
    ) -> Result<PluginInstance, Error> {
        // For compiled-in plugins, we'd use the registered factories
        // For dynamic loading, we'd use libloading or similar
        
        // Try to find a registered factory for this plugin ID
        // This is a simplified approach - in production you'd have better plugin discovery
        
        // For now, return an error as we need actual plugin implementations
        Err(anyhow!("Native plugin loading requires registered plugin factory for: {}", manifest.id))
    }
    
    /// Load a WASM plugin
    async fn load_wasm_plugin(
        &self,
        path: &Path,
        manifest: &PluginManifest,
    ) -> Result<PluginInstance, Error> {
        let wasm_path = path.join(&manifest.entry_point);
        
        // Load WASM module
        let module = Module::from_file(&self.engine, wasm_path)?;
        
        // Create store
        let mut store = Store::new(&self.engine, ());
        
        // Create linker with host functions
        let linker = self.create_wasm_linker()?;
        
        // Instantiate module
        let instance = linker.instantiate(&mut store, &module)?;
        
        Ok(PluginInstance::Wasm(instance))
    }
    
    /// Create WASM linker with host functions
    fn create_wasm_linker(&self) -> Result<Linker<()>, Error> {
        let mut linker = Linker::new(&self.engine);
        
        // Add host functions that plugins can call
        // These would include functions for:
        // - Token creation
        // - Database access
        // - HTTP requests
        // - Logging
        // etc.
        
        // Example host function
        linker.func_wrap("host", "log", |message: i32| {
            println!("WASM plugin log: {}", message);
        })?;
        
        Ok(linker)
    }
    
    /// Load plugin configuration
    fn load_config(&self, path: &Path) -> Result<Value, Error> {
        let config_path = path.join("config.json");
        
        if config_path.exists() {
            let content = std::fs::read_to_string(config_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(serde_json::json!({}))
        }
    }
    
    /// Validate plugin dependencies
    fn validate_dependencies(&self, manifest: &PluginManifest) -> Result<(), Error> {
        for dependency in &manifest.dependencies {
            if dependency.required {
                // In a real system, we'd check if the dependency is loaded
                // For now, just log
                println!("Checking dependency: {} version {}", 
                    dependency.plugin_id, 
                    dependency.version
                );
            }
        }
        
        Ok(())
    }
    
    /// Export plugin state
    pub async fn export_plugin_state(
        &self,
        plugin: &LoadedPlugin,
    ) -> Result<Value, Error> {
        match &plugin.instance {
            PluginInstance::Native(p) => {
                p.export_state().await
            }
            PluginInstance::Wasm(_instance) => {
                // For WASM, we'd call an export function through the WASM boundary
                Ok(serde_json::json!({}))
            }
        }
    }
    
    /// Import plugin state
    pub async fn import_plugin_state(
        &self,
        plugin: &mut LoadedPlugin,
        state: Value,
    ) -> Result<(), Error> {
        match &mut plugin.instance {
            PluginInstance::Native(p) => {
                p.import_state(state).await
            }
            PluginInstance::Wasm(_instance) => {
                // For WASM, we'd call an import function through the WASM boundary
                Ok(())
            }
        }
    }
}