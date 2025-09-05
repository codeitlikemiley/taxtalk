use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use wasmtime::Instance;

use crate::plugin::traits::Plugin;
use crate::plugin::manifest::PluginManifest;

/// Loaded plugin instance
pub struct LoadedPlugin {
    pub id: String,
    pub instance: PluginInstance,
    pub manifest: PluginManifest,
    pub state: PluginState,
    pub config: Value,
}

/// Plugin instance types
pub enum PluginInstance {
    /// Native Rust plugin
    Native(Box<dyn Plugin>),
    
    /// WASM plugin instance
    Wasm(Instance),
}

/// Plugin state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginState {
    pub enabled: bool,
    pub loaded_at: chrono::DateTime<chrono::Utc>,
    pub last_executed: Option<chrono::DateTime<chrono::Utc>>,
    pub execution_count: u64,
    pub error_count: u64,
    pub custom_state: Value,
}

impl PluginState {
    pub fn new() -> Self {
        Self {
            enabled: true,
            loaded_at: chrono::Utc::now(),
            last_executed: None,
            execution_count: 0,
            error_count: 0,
            custom_state: serde_json::json!({}),
        }
    }
}

/// Command request from client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRequest {
    pub command: String,
    pub session_id: Option<String>,
    pub context: Option<Value>,
}

/// Command response to client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResponse {
    pub success: bool,
    pub result: Option<Value>,
    pub error: Option<String>,
    pub suggestions: Vec<String>,
    pub metadata: Option<Value>,
}

/// Plugin error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginError {
    NotFound { plugin_id: String },
    LoadFailed { plugin_id: String, reason: String },
    ExecutionFailed { plugin_id: String, reason: String },
    InvalidManifest { reason: String },
    DependencyMissing { dependency: String },
    VersionConflict { plugin_id: String, required: String, found: String },
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginError::NotFound { plugin_id } => write!(f, "Plugin not found: {}", plugin_id),
            PluginError::LoadFailed { plugin_id, reason } => write!(f, "Failed to load plugin {}: {}", plugin_id, reason),
            PluginError::ExecutionFailed { plugin_id, reason } => write!(f, "Plugin {} execution failed: {}", plugin_id, reason),
            PluginError::InvalidManifest { reason } => write!(f, "Invalid manifest: {}", reason),
            PluginError::DependencyMissing { dependency } => write!(f, "Missing dependency: {}", dependency),
            PluginError::VersionConflict { plugin_id, required, found } => {
                write!(f, "Version conflict for {}: required {}, found {}", plugin_id, required, found)
            }
        }
    }
}

impl std::error::Error for PluginError {}

/// Plugin capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCapabilities {
    pub commands: Vec<String>,
    pub token_types: Vec<String>,
    pub features: HashMap<String, bool>,
}

/// Plugin dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    pub plugin_id: String,
    pub version: String,
    pub required: bool,
}

/// Terms token for payment terms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermsToken {
    pub base: crate::token::BaseTokenFields,
    pub days: u32,
    pub term_type: TermType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TermType {
    Net,
    Discount { percentage: rust_decimal::Decimal, days: u32 },
    Immediate,
}

impl crate::token::Token for TermsToken {
    fn token_type(&self) -> &str {
        "terms"
    }
    
    fn uuid(&self) -> uuid::Uuid {
        self.base.uuid
    }
    
    fn raw_text(&self) -> &str {
        &self.base.raw_text
    }
    
    fn position(&self) -> &crate::token::TokenPosition {
        &self.base.position
    }
    
    fn to_json(&self) -> Value {
        serde_json::json!({
            "type": "terms",
            "uuid": self.uuid().to_string(),
            "days": self.days,
            "term_type": self.term_type,
            "raw_text": self.raw_text(),
        })
    }
    
    fn clone_box(&self) -> Box<dyn crate::token::Token> {
        Box::new(self.clone())
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    
    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }
    
    fn matches(&self, pattern: &dyn crate::token::TokenPattern) -> bool {
        pattern.matches(self)
    }
    
    fn validate(&self) -> Result<(), crate::token::ValidationError> {
        if self.days == 0 && !matches!(self.term_type, TermType::Immediate) {
            return Err(crate::token::ValidationError::InvalidToken {
                reason: "Terms must have days > 0 unless immediate".to_string(),
            });
        }
        Ok(())
    }
    
    fn metadata(&self) -> crate::token::TokenMetadata {
        self.base.metadata.clone()
    }
    
    fn can_compose_with(&self, other: &dyn crate::token::Token) -> bool {
        matches!(other.token_type(), "invoice" | "payment" | "amount")
    }
}