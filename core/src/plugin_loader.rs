// Plugin loader for Wasmtime WASM execution
use std::collections::HashMap;
use std::path::Path;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::{WasiCtxBuilder, preview1::WasiP1Ctx};

use crate::manifest::PluginManifest;

pub struct PluginLoader {
    engine: Engine,
    plugins: HashMap<String, PluginInstance>,
}

pub struct PluginInstance {
    pub store: Store<WasiP1Ctx>,
    pub instance: wasmtime::Instance,
    pub manifest: PluginManifest,
}

impl PluginLoader {
    pub fn new() -> Self {
        let engine = Engine::default();
        Self {
            engine,
            plugins: HashMap::new(),
        }
    }

    pub fn load_plugin<P: AsRef<Path>>(
        &mut self,
        wasm_path: P,
        manifest: PluginManifest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create WASI context
        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdio()
            .build_p1();

        // Create store
        let mut store = Store::new(&self.engine, wasi_ctx);

        // Load WASM module
        let module = Module::from_file(&self.engine, wasm_path)?;

        // Create linker
        let mut linker = Linker::new(&self.engine);
        wasmtime_wasi::preview1::add_to_linker_sync(&mut linker, |state| state)?;

        // Instantiate the module
        let instance = linker.instantiate(&mut store, &module)?;

        // Store the plugin instance
        let plugin_instance = PluginInstance {
            store,
            instance,
            manifest: manifest.clone(),
        };

        self.plugins.insert(manifest.id.clone(), plugin_instance);

        tracing::info!("Loaded plugin: {}", manifest.id);
        Ok(())
    }

    pub fn get_plugin(&self, plugin_id: &str) -> Option<&PluginInstance> {
        self.plugins.get(plugin_id)
    }

    pub fn get_plugin_mut(&mut self, plugin_id: &str) -> Option<&mut PluginInstance> {
        self.plugins.get_mut(plugin_id)
    }

    pub fn unload_plugin(&mut self, plugin_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        if self.plugins.remove(plugin_id).is_some() {
            tracing::info!("Unloaded plugin: {}", plugin_id);
            Ok(())
        } else {
            Err(format!("Plugin {} not found", plugin_id).into())
        }
    }

    pub fn list_plugins(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }

    pub fn call_plugin_function(
        &mut self,
        plugin_id: &str,
        function_name: &str,
        params: &[wasmtime::Val],
    ) -> Result<Vec<wasmtime::Val>, Box<dyn std::error::Error>> {
        let plugin = self
            .plugins
            .get_mut(plugin_id)
            .ok_or_else(|| format!("Plugin {} not found", plugin_id))?;

        let func = plugin
            .instance
            .get_func(&mut plugin.store, function_name)
            .ok_or_else(|| {
                format!(
                    "Function {} not found in plugin {}",
                    function_name, plugin_id
                )
            })?;

        let mut results = vec![wasmtime::Val::I32(0); func.ty(&plugin.store).results().len()];
        func.call(&mut plugin.store, params, &mut results)?;

        Ok(results)
    }
}

impl PluginInstance {
    pub fn call_update(&mut self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let func = self
            .instance
            .get_func(&mut self.store, "update")
            .ok_or("update function not found")?;

        // For simplicity, we'll assume the function takes a pointer and length
        // In a real implementation, you'd need proper memory management
        let ptr = 0; // This would need proper allocation
        let len = data.len() as i32;

        let params = [wasmtime::Val::I32(ptr), wasmtime::Val::I32(len)];
        let mut results = vec![wasmtime::Val::I32(0)];

        func.call(&mut self.store, &params, &mut results)?;

        // Return dummy result for now
        Ok(vec![])
    }

    pub fn call_view(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let func = self
            .instance
            .get_func(&mut self.store, "view")
            .ok_or("view function not found")?;

        let params = [];
        let mut results = vec![wasmtime::Val::I32(0)];

        func.call(&mut self.store, &params, &mut results)?;

        // Return dummy result for now
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_plugin_loader_creation() {
        let loader = PluginLoader::new();
        assert!(loader.list_plugins().is_empty());
    }

    #[test]
    fn test_plugin_listing() {
        let mut loader = PluginLoader::new();

        // Create a dummy manifest
        let manifest = PluginManifest {
            id: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            entry: "test.wasm".to_string(),
            capabilities: vec![],
            commands: vec![],
            schemas: HashMap::new(),
            dependencies: vec![],
        };

        // Note: This test would need a real WASM file to fully work
        // For now, just test the manifest handling
        assert!(loader.get_plugin("test-plugin").is_none());
    }
}
