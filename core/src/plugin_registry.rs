use std::collections::HashMap;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use tokio::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub wasm_path: PathBuf,
    pub installed_at: chrono::DateTime<chrono::Utc>,
    pub enabled: bool,
}

/// Registry for managing installed plugins
pub struct PluginRegistry {
    registry_path: PathBuf,
    plugins: HashMap<String, PluginMetadata>,
}

impl PluginRegistry {
    pub fn new(registry_path: impl AsRef<Path>) -> Self {
        Self {
            registry_path: registry_path.as_ref().to_path_buf(),
            plugins: HashMap::new(),
        }
    }
    
    /// Load registry from disk
    pub async fn load(&mut self) -> Result<()> {
        let registry_file = self.registry_path.join("registry.json");
        
        if !registry_file.exists() {
            // Create empty registry
            self.plugins.clear();
            return Ok(());
        }
        
        let content = fs::read_to_string(&registry_file).await?;
        let plugins: Vec<PluginMetadata> = serde_json::from_str(&content)?;
        
        self.plugins.clear();
        for plugin in plugins {
            self.plugins.insert(plugin.id.clone(), plugin);
        }
        
        Ok(())
    }
    
    /// Save registry to disk
    pub async fn save(&self) -> Result<()> {
        // Ensure directory exists
        fs::create_dir_all(&self.registry_path).await?;
        
        let registry_file = self.registry_path.join("registry.json");
        let plugins: Vec<_> = self.plugins.values().cloned().collect();
        let content = serde_json::to_string_pretty(&plugins)?;
        
        fs::write(&registry_file, content).await?;
        Ok(())
    }
    
    /// Register a new plugin
    pub async fn register(&mut self, metadata: PluginMetadata) -> Result<()> {
        if self.plugins.contains_key(&metadata.id) {
            return Err(anyhow!("Plugin {} is already registered", metadata.id));
        }
        
        self.plugins.insert(metadata.id.clone(), metadata);
        self.save().await?;
        Ok(())
    }
    
    /// Unregister a plugin
    pub async fn unregister(&mut self, plugin_id: &str) -> Result<PluginMetadata> {
        let metadata = self.plugins.remove(plugin_id)
            .ok_or_else(|| anyhow!("Plugin {} not found", plugin_id))?;
        
        // Delete the WASM file
        if metadata.wasm_path.exists() {
            fs::remove_file(&metadata.wasm_path).await?;
        }
        
        self.save().await?;
        Ok(metadata)
    }
    
    /// Get plugin metadata
    pub fn get(&self, plugin_id: &str) -> Option<&PluginMetadata> {
        self.plugins.get(plugin_id)
    }
    
    /// List all plugins
    pub fn list(&self) -> Vec<PluginMetadata> {
        self.plugins.values().cloned().collect()
    }
    
    /// Enable/disable a plugin
    pub async fn set_enabled(&mut self, plugin_id: &str, enabled: bool) -> Result<()> {
        let plugin = self.plugins.get_mut(plugin_id)
            .ok_or_else(|| anyhow!("Plugin {} not found", plugin_id))?;
        
        plugin.enabled = enabled;
        self.save().await?;
        Ok(())
    }
    
    /// Get path to plugin storage directory
    pub fn plugin_dir(&self) -> PathBuf {
        self.registry_path.join("plugins")
    }
    
    /// Store plugin WASM file
    pub async fn store_plugin_wasm(&self, plugin_id: &str, wasm_bytes: &[u8]) -> Result<PathBuf> {
        let plugin_dir = self.plugin_dir();
        fs::create_dir_all(&plugin_dir).await?;
        
        let wasm_path = plugin_dir.join(format!("{plugin_id}.wasm"));
        fs::write(&wasm_path, wasm_bytes).await?;
        
        Ok(wasm_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_registry_operations() {
        let temp_dir = tempdir().unwrap();
        let mut registry = PluginRegistry::new(temp_dir.path());
        
        // Load empty registry
        registry.load().await.unwrap();
        assert_eq!(registry.list().len(), 0);
        
        // Register a plugin
        let metadata = PluginMetadata {
            id: "test-plugin".to_string(),
            name: "Test Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "A test plugin".to_string(),
            wasm_path: temp_dir.path().join("test.wasm"),
            installed_at: chrono::Utc::now(),
            enabled: true,
        };
        
        registry.register(metadata.clone()).await.unwrap();
        assert_eq!(registry.list().len(), 1);
        
        // Get plugin
        let plugin = registry.get("test-plugin").unwrap();
        assert_eq!(plugin.name, "Test Plugin");
        
        // Disable plugin
        registry.set_enabled("test-plugin", false).await.unwrap();
        let plugin = registry.get("test-plugin").unwrap();
        assert!(!plugin.enabled);
        
        // Unregister plugin
        registry.unregister("test-plugin").await.unwrap();
        assert_eq!(registry.list().len(), 0);
    }
}