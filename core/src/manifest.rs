// Plugin manifest parsing and validation
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub version: String,
    pub entry: String,
    pub capabilities: Vec<String>,
    pub commands: Vec<PluginCommand>,
    pub schemas: HashMap<String, String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCommand {
    pub name: String,
    pub schema: String,
    pub aliases: Vec<String>,
}

impl PluginManifest {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let manifest: PluginManifest = serde_json::from_str(&content)?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate ID format
        if self.id.is_empty() {
            errors.push("Plugin ID cannot be empty".to_string());
        }

        // Validate version format
        if self.version.is_empty() {
            errors.push("Plugin version cannot be empty".to_string());
        }

        // Validate entry point
        if self.entry.is_empty() {
            errors.push("Plugin entry point cannot be empty".to_string());
        }

        // Validate capabilities
        let valid_capabilities = ["http", "kv", "render", "pubsub"];
        for cap in &self.capabilities {
            if !valid_capabilities.contains(&cap.as_str()) {
                errors.push(format!("Invalid capability: {}", cap));
            }
        }

        // Validate commands
        for cmd in &self.commands {
            if cmd.name.is_empty() {
                errors.push("Command name cannot be empty".to_string());
            }
            if cmd.schema.is_empty() {
                errors.push(format!("Command '{}' schema cannot be empty", cmd.name));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn get_command_by_alias(&self, alias: &str) -> Option<&PluginCommand> {
        self.commands.iter().find(|cmd| {
            cmd.name == alias || cmd.aliases.contains(&alias.to_string())
        })
    }
}

pub struct ManifestManager {
    manifests: HashMap<String, PluginManifest>,
}

impl ManifestManager {
    pub fn new() -> Self {
        Self {
            manifests: HashMap::new(),
        }
    }

    pub fn load_manifest<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let manifest = PluginManifest::from_file(path)?;
        manifest.validate().map_err(|errors| errors.join(", "))?;
        self.manifests.insert(manifest.id.clone(), manifest);
        Ok(())
    }

    pub fn get_manifest(&self, plugin_id: &str) -> Option<&PluginManifest> {
        self.manifests.get(plugin_id)
    }

    pub fn get_all_manifests(&self) -> Vec<&PluginManifest> {
        self.manifests.values().collect()
    }

    pub fn find_command(&self, alias: &str) -> Option<(&PluginManifest, &PluginCommand)> {
        for manifest in self.manifests.values() {
            if let Some(cmd) = manifest.get_command_by_alias(alias) {
                return Some((manifest, cmd));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_manifest_validation() {
        let manifest = PluginManifest {
            id: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            entry: "lib.wasm".to_string(),
            capabilities: vec!["http".to_string(), "kv".to_string()],
            commands: vec![PluginCommand {
                name: "create".to_string(),
                schema: "create_schema.json".to_string(),
                aliases: vec!["@create".to_string()],
            }],
            schemas: HashMap::new(),
            dependencies: vec![],
        };

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_invalid_manifest() {
        let manifest = PluginManifest {
            id: "".to_string(), // Invalid: empty ID
            version: "1.0.0".to_string(),
            entry: "lib.wasm".to_string(),
            capabilities: vec!["invalid_cap".to_string()], // Invalid capability
            commands: vec![],
            schemas: HashMap::new(),
            dependencies: vec![],
        };

        let errors = manifest.validate().unwrap_err();
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_command_lookup() {
        let manifest = PluginManifest {
            id: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            entry: "lib.wasm".to_string(),
            capabilities: vec![],
            commands: vec![PluginCommand {
                name: "create".to_string(),
                schema: "create_schema.json".to_string(),
                aliases: vec!["@create".to_string(), "@new".to_string()],
            }],
            schemas: HashMap::new(),
            dependencies: vec![],
        };

        assert!(manifest.get_command_by_alias("create").is_some());
        assert!(manifest.get_command_by_alias("@create").is_some());
        assert!(manifest.get_command_by_alias("@new").is_some());
        assert!(manifest.get_command_by_alias("nonexistent").is_none());
    }
}