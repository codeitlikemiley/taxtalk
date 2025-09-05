use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::path::PathBuf;

/// Plugin manifest definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub plugin_type: PluginType,
    pub entry_point: String,
    
    /// Commands this plugin provides
    pub commands: Vec<CommandManifest>,
    
    /// Token types this plugin creates
    pub token_types: Vec<TokenTypeManifest>,
    
    /// Dependencies on other plugins
    pub dependencies: Vec<PluginDependency>,
    
    /// Configuration schema
    pub config_schema: Option<Value>,
    
    /// Required capabilities
    pub required_capabilities: Vec<String>,
    
    /// Plugin metadata
    pub metadata: PluginMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginType {
    Native,
    Wasm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandManifest {
    pub command: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub requirements: RequirementsManifest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementsManifest {
    pub required: Vec<TokenRequirementManifest>,
    pub optional: Vec<TokenRequirementManifest>,
    pub defaults: Vec<TokenDefaultManifest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRequirementManifest {
    pub token_type: String,
    pub description: String,
    pub multiple: bool,
    pub validation: Option<String>, // Script or rule name
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenDefaultManifest {
    pub token_type: String,
    pub value: Value,
    pub condition: Option<String>, // Script or rule name
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenTypeManifest {
    pub type_name: String,
    pub patterns: Vec<String>,
    pub creator_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    pub plugin_id: String,
    pub version: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub author: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub license: Option<String>,
    pub tags: Vec<String>,
}

impl PluginManifest {
    /// Load manifest from JSON file
    pub fn from_file(path: &PathBuf) -> Result<Self, anyhow::Error> {
        let content = std::fs::read_to_string(path)?;
        let manifest: Self = serde_json::from_str(&content)?;
        manifest.validate()?;
        Ok(manifest)
    }
    
    /// Validate the manifest
    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if self.id.is_empty() {
            return Err(anyhow::anyhow!("Plugin ID cannot be empty"));
        }
        
        if self.name.is_empty() {
            return Err(anyhow::anyhow!("Plugin name cannot be empty"));
        }
        
        if self.version.is_empty() {
            return Err(anyhow::anyhow!("Plugin version cannot be empty"));
        }
        
        // Validate version format (simple semver check)
        if !self.version.chars().filter(|c| *c == '.').count() == 2 {
            return Err(anyhow::anyhow!("Invalid version format: {}", self.version));
        }
        
        Ok(())
    }
    
    /// Check if this plugin is compatible with another version
    pub fn is_compatible_with(&self, required_version: &str) -> bool {
        // Simple version compatibility check
        // In production, use a proper semver library
        let self_parts: Vec<u32> = self.version
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
            
        let required_parts: Vec<u32> = required_version
            .trim_start_matches(">=")
            .trim_start_matches('^')
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        
        if self_parts.len() != 3 || required_parts.len() != 3 {
            return false;
        }
        
        // Major version must match
        if self_parts[0] != required_parts[0] {
            return false;
        }
        
        // Minor version must be >= required
        if self_parts[1] < required_parts[1] {
            return false;
        }
        
        // If minor versions match, patch must be >= required
        if self_parts[1] == required_parts[1] && self_parts[2] < required_parts[2] {
            return false;
        }
        
        true
    }
}