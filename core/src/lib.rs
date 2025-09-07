// New simplified core without Crux
pub mod plugin_host;
pub mod plugin_registry;
pub mod router;
pub mod plugin;
pub mod token;
pub mod tokenizer;
pub mod database;
pub mod repositories;
pub mod validation;
pub mod nlp;

pub use plugin_host::PluginHost;
pub use plugin_registry::PluginRegistry;
pub use router::CommandRouter;
pub use database::{DbPool, init_database};
pub use validation::{TokenValidator, ValidationResult, GuidedSession, SessionState};

// Re-export common types
pub use anyhow::{Result, Error};
pub use serde_json::Value;