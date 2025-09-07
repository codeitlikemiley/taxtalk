use std::collections::{HashMap, HashSet};
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Represents a command from a plugin manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandDefinition {
    pub command: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub plugin_id: String,
    pub requirements: CommandRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRequirements {
    pub required: Vec<TokenRequirement>,
    pub optional: Vec<TokenRequirement>,
    #[serde(default)]
    pub defaults: Vec<TokenDefault>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRequirement {
    pub token_type: String,
    pub description: String,
    pub multiple: bool,
    #[serde(default)]
    pub entity_type: Option<String>, // For entity_ref tokens
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenDefault {
    pub token_type: String,
    pub value: serde_json::Value,
}

/// Plugin manifest structure (partial - just what we need)
#[derive(Debug, Deserialize)]
struct PluginManifest {
    id: String,
    name: String,
    commands: Vec<CommandManifestEntry>,
}

#[derive(Debug, Deserialize)]
struct CommandManifestEntry {
    command: String,
    #[serde(default)]
    aliases: Vec<String>,
    description: String,
    requirements: CommandRequirements,
}

/// Registry for all available commands from plugins
pub struct CommandRegistry {
    /// Map from command (including aliases) to command definition
    commands: HashMap<String, CommandDefinition>,
    /// Map from plugin ID to its commands
    plugin_commands: HashMap<String, Vec<String>>,
    /// Set of all valid command prefixes for quick lookup
    valid_prefixes: HashSet<String>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            plugin_commands: HashMap::new(),
            valid_prefixes: HashSet::new(),
        }
    }
    
    /// Load commands from a plugin manifest file
    pub async fn load_plugin_manifest(&mut self, manifest_path: impl AsRef<Path>) -> Result<()> {
        let content = tokio::fs::read_to_string(manifest_path.as_ref()).await?;
        let manifest: PluginManifest = serde_json::from_str(&content)?;
        
        let mut plugin_commands = Vec::new();
        
        for cmd_entry in manifest.commands {
            let command_def = CommandDefinition {
                command: cmd_entry.command.clone(),
                aliases: cmd_entry.aliases.clone(),
                description: cmd_entry.description,
                plugin_id: manifest.id.clone(),
                requirements: cmd_entry.requirements,
            };
            
            // Register main command
            self.commands.insert(cmd_entry.command.clone(), command_def.clone());
            plugin_commands.push(cmd_entry.command.clone());
            
            // Register command with common prefixes
            self.register_command_variations(&cmd_entry.command, &command_def);
            
            // Register aliases
            for alias in &cmd_entry.aliases {
                self.commands.insert(alias.clone(), command_def.clone());
                self.register_command_variations(alias, &command_def);
            }
        }
        
        self.plugin_commands.insert(manifest.id.clone(), plugin_commands);
        
        Ok(())
    }
    
    /// Register common command variations (with action verbs)
    fn register_command_variations(&mut self, base_command: &str, definition: &CommandDefinition) {
        // Common action verbs that might precede commands
        let action_verbs = ["create", "add", "new", "record", "generate", "make", "process"];
        
        for verb in &action_verbs {
            let full_command = format!("{verb} {base_command}");
            self.commands.insert(full_command.clone(), definition.clone());
            self.valid_prefixes.insert(full_command);
        }
        
        // Also add the base command as a valid prefix
        self.valid_prefixes.insert(base_command.to_string());
    }
    
    /// Load all plugin manifests from a directory
    pub async fn load_all_from_directory(&mut self, plugins_dir: impl AsRef<Path>) -> Result<()> {
        let plugins_dir = plugins_dir.as_ref();
        
        // Look for manifest.json files in subdirectories
        let mut entries = tokio::fs::read_dir(plugins_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join("manifest.json");
                if manifest_path.exists() {
                    // Load this manifest, but don't fail if one plugin has issues
                    if let Err(e) = self.load_plugin_manifest(&manifest_path).await {
                        eprintln!("Failed to load manifest from {manifest_path:?}: {e}");
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Check if a text contains a valid command
    pub fn has_valid_command(&self, text: &str) -> bool {
        let text_lower = text.to_lowercase();
        
        // Check if any valid command prefix is in the text
        for prefix in &self.valid_prefixes {
            if text_lower.contains(&prefix.to_lowercase()) {
                return true;
            }
        }
        
        false
    }
    
    /// Get command definition for a text
    pub fn get_command_for_text(&self, text: &str) -> Option<&CommandDefinition> {
        let text_lower = text.to_lowercase();
        
        // Try to find the most specific matching command
        let mut best_match: Option<(&String, &CommandDefinition)> = None;
        let mut best_length = 0;
        
        for (cmd_key, cmd_def) in &self.commands {
            if text_lower.contains(&cmd_key.to_lowercase())
                && cmd_key.len() > best_length {
                    best_match = Some((cmd_key, cmd_def));
                    best_length = cmd_key.len();
                }
        }
        
        best_match.map(|(_, def)| def)
    }
    
    /// Get entity types required for a command
    pub fn get_required_entities(&self, text: &str) -> Vec<String> {
        if let Some(command) = self.get_command_for_text(text) {
            let mut entity_types = Vec::new();
            
            for req in &command.requirements.required {
                if req.token_type == "entity_ref" {
                    // Determine entity type from context or default to "client"
                    let entity_type = req.entity_type.clone()
                        .or_else(|| {
                            // Infer from description
                            if req.description.to_lowercase().contains("customer") || 
                               req.description.to_lowercase().contains("client") {
                                Some("client".to_string())
                            } else if req.description.to_lowercase().contains("supplier") ||
                                      req.description.to_lowercase().contains("vendor") {
                                Some("supplier".to_string())
                            } else if req.description.to_lowercase().contains("product") {
                                Some("product".to_string())
                            } else if req.description.to_lowercase().contains("employee") {
                                Some("employee".to_string())
                            } else {
                                Some("entity".to_string())
                            }
                        })
                        .unwrap_or_else(|| "entity".to_string());
                    
                    entity_types.push(entity_type);
                }
            }
            
            entity_types
        } else {
            Vec::new()
        }
    }
    
    /// Get all registered commands
    pub fn get_all_commands(&self) -> Vec<String> {
        self.valid_prefixes.iter().cloned().collect()
    }
    
    /// Get commands for a specific plugin
    pub fn get_plugin_commands(&self, plugin_id: &str) -> Vec<String> {
        self.plugin_commands.get(plugin_id)
            .cloned()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;
    
    #[tokio::test]
    async fn test_command_registry() {
        let temp_dir = tempdir().unwrap();
        let manifest_path = temp_dir.path().join("test_plugin").join("manifest.json");
        
        // Create test manifest
        fs::create_dir_all(manifest_path.parent().unwrap()).await.unwrap();
        
        let manifest_content = r#"{
            "id": "test",
            "name": "Test Plugin",
            "commands": [
                {
                    "command": "invoice",
                    "aliases": ["bill"],
                    "description": "Create an invoice",
                    "requirements": {
                        "required": [
                            {
                                "token_type": "entity_ref",
                                "description": "Customer to invoice",
                                "multiple": false
                            }
                        ],
                        "optional": [],
                        "defaults": []
                    }
                }
            ]
        }"#;
        
        fs::write(&manifest_path, manifest_content).await.unwrap();
        
        let mut registry = CommandRegistry::new();
        registry.load_plugin_manifest(&manifest_path).await.unwrap();
        
        // Test command detection
        assert!(registry.has_valid_command("create invoice"));
        assert!(registry.has_valid_command("invoice"));
        assert!(registry.has_valid_command("bill"));
        assert!(!registry.has_valid_command("random text"));
        
        // Test command retrieval
        let cmd = registry.get_command_for_text("create invoice for client").unwrap();
        assert_eq!(cmd.plugin_id, "test");
        
        // Test entity type detection
        let entities = registry.get_required_entities("create invoice");
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0], "client");
    }
}