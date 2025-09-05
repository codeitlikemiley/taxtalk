use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Plugin manifest handling and validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Unique plugin identifier
    pub id: String,

    /// Human-readable name
    pub name: String,

    /// Semantic version
    pub version: String,

    /// Plugin type (native or WASM)
    pub plugin_type: PluginType,

    /// Entry point file
    pub entry_point: String,

    /// Commands this plugin provides
    pub commands: Vec<CommandManifest>,

    /// Token types this plugin defines
    pub token_types: Vec<TokenTypeManifest>,

    /// Plugin dependencies
    pub dependencies: Vec<PluginDependency>,

    /// Configuration schema
    pub config_schema: Option<Value>,

    /// Metadata
    pub metadata: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginType {
    #[serde(rename = "native")]
    Native,
    #[serde(rename = "wasm")]
    Wasm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandManifest {
    /// Command name
    pub name: String,

    /// Alternative command names
    pub aliases: Vec<String>,

    /// Human-readable description
    pub description: String,

    /// Requirements for this command
    pub requirements: CommandRequirements,

    /// Examples of usage
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRequirements {
    /// Required token types
    pub required: Vec<TokenRequirement>,

    /// Optional token types
    pub optional: Vec<TokenRequirement>,

    /// Default values for tokens
    pub defaults: Vec<TokenDefault>,

    /// Validation rules
    pub validators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRequirement {
    /// Token type name
    pub token_type: String,

    /// Human-readable description
    pub description: String,

    /// Whether multiple tokens of this type are allowed
    pub multiple: bool,

    /// Validation rules
    pub validation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenDefault {
    /// Token type to provide default for
    pub token_type: String,

    /// Default value
    pub default_value: Value,

    /// Condition when to apply this default
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenTypeManifest {
    /// Token type name
    pub type_name: String,

    /// Regex patterns for recognition
    pub patterns: Vec<String>,

    /// Creator class name
    pub creator_class: String,

    /// Schema for token validation
    pub schema: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    /// Plugin ID
    pub plugin_id: String,

    /// Version requirement
    pub version: String,

    /// Whether this dependency is required
    pub required: bool,
}

#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid version format: {0}")]
    InvalidVersion(String),
}

pub struct ManifestValidator;

impl ManifestValidator {
    /// Validate a plugin manifest
    pub fn validate(manifest: &PluginManifest) -> Result<(), ManifestError> {
        // Validate required fields
        if manifest.id.is_empty() {
            return Err(ManifestError::MissingField("id".to_string()));
        }

        if manifest.name.is_empty() {
            return Err(ManifestError::MissingField("name".to_string()));
        }

        if manifest.version.is_empty() {
            return Err(ManifestError::MissingField("version".to_string()));
        }

        if manifest.entry_point.is_empty() {
            return Err(ManifestError::MissingField("entry_point".to_string()));
        }

        // Validate version format (basic semantic versioning)
        Self::validate_version(&manifest.version)?;

        // Validate commands
        for command in &manifest.commands {
            Self::validate_command(command)?;
        }

        // Validate token types
        for token_type in &manifest.token_types {
            Self::validate_token_type(token_type)?;
        }

        // Validate dependencies
        for dependency in &manifest.dependencies {
            Self::validate_dependency(dependency)?;
        }

        Ok(())
    }

    fn validate_version(version: &str) -> Result<(), ManifestError> {
        // Basic semantic version validation (x.y.z format)
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return Err(ManifestError::InvalidVersion(version.to_string()));
        }

        for part in parts {
            if part.parse::<u32>().is_err() {
                return Err(ManifestError::InvalidVersion(version.to_string()));
            }
        }

        Ok(())
    }

    fn validate_command(command: &CommandManifest) -> Result<(), ManifestError> {
        if command.name.is_empty() {
            return Err(ManifestError::Validation("Command name cannot be empty".to_string()));
        }

        if command.description.is_empty() {
            return Err(ManifestError::Validation("Command description cannot be empty".to_string()));
        }

        // Validate requirements
        for req in &command.requirements.required {
            if req.token_type.is_empty() {
                return Err(ManifestError::Validation("Required token type cannot be empty".to_string()));
            }
        }

        Ok(())
    }

    fn validate_token_type(token_type: &TokenTypeManifest) -> Result<(), ManifestError> {
        if token_type.type_name.is_empty() {
            return Err(ManifestError::Validation("Token type name cannot be empty".to_string()));
        }

        if token_type.creator_class.is_empty() {
            return Err(ManifestError::Validation("Creator class cannot be empty".to_string()));
        }

        Ok(())
    }

    fn validate_dependency(dependency: &PluginDependency) -> Result<(), ManifestError> {
        if dependency.plugin_id.is_empty() {
            return Err(ManifestError::Validation("Dependency plugin ID cannot be empty".to_string()));
        }

        Self::validate_version(&dependency.version)?;

        Ok(())
    }
}

pub struct ManifestLoader;

impl ManifestLoader {
    /// Load and validate a plugin manifest from file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<PluginManifest, ManifestError> {
        let content = fs::read_to_string(path)?;
        let manifest: PluginManifest = serde_json::from_str(&content)?;
        ManifestValidator::validate(&manifest)?;
        Ok(manifest)
    }

    /// Load and validate a plugin manifest from string
    pub fn load_from_string(content: &str) -> Result<PluginManifest, ManifestError> {
        let manifest: PluginManifest = serde_json::from_str(content)?;
        ManifestValidator::validate(&manifest)?;
        Ok(manifest)
    }

    /// Create a default manifest template
    pub fn create_template(plugin_id: &str, plugin_name: &str) -> PluginManifest {
        PluginManifest {
            id: plugin_id.to_string(),
            name: plugin_name.to_string(),
            version: "0.1.0".to_string(),
            plugin_type: PluginType::Wasm,
            entry_point: "lib.wasm".to_string(),
            commands: vec![
                CommandManifest {
                    name: "example_command".to_string(),
                    aliases: vec!["@example".to_string()],
                    description: "Example command".to_string(),
                    requirements: CommandRequirements {
                        required: vec![],
                        optional: vec![],
                        defaults: vec![],
                        validators: vec![],
                    },
                    examples: vec!["@example".to_string()],
                }
            ],
            token_types: vec![],
            dependencies: vec![],
            config_schema: None,
            metadata: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_manifest() {
        let manifest = ManifestLoader::create_template("test-plugin", "Test Plugin");
        assert!(ManifestValidator::validate(&manifest).is_ok());
    }

    #[test]
    fn test_invalid_manifest_missing_id() {
        let mut manifest = ManifestLoader::create_template("test-plugin", "Test Plugin");
        manifest.id = "".to_string();
        assert!(ManifestValidator::validate(&manifest).is_err());
    }

    #[test]
    fn test_invalid_version() {
        let mut manifest = ManifestLoader::create_template("test-plugin", "Test Plugin");
        manifest.version = "invalid".to_string();
        assert!(ManifestValidator::validate(&manifest).is_err());
    }

    #[test]
    fn test_valid_version() {
        assert!(ManifestValidator::validate_version("1.2.3").is_ok());
        assert!(ManifestValidator::validate_version("0.1.0").is_ok());
        assert!(ManifestValidator::validate_version("10.0.0").is_ok());
    }

    #[test]
    fn test_invalid_versions() {
        assert!(ManifestValidator::validate_version("1.2").is_err());
        assert!(ManifestValidator::validate_version("1.2.3.4").is_err());
        assert!(ManifestValidator::validate_version("abc").is_err());
        assert!(ManifestValidator::validate_version("1.2.def").is_err());
    }
}