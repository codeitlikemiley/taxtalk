use anyhow::Error;
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

use crate::token::{ActionToken, Token};

/// Base trait that all plugins must implement
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Unique identifier for this plugin
    fn id(&self) -> &str;

    /// Version of this plugin
    fn version(&self) -> &str;

    /// Commands this plugin can handle
    fn commands(&self) -> Vec<CommandRegistration>;

    /// Check if this plugin can handle the given command
    fn can_handle(&self, command: &ActionToken, tokens: &[Box<dyn Token>]) -> CanHandleResult;

    /// Execute the command with the given tokens
    async fn execute(
        &self,
        command: &ActionToken,
        tokens: Vec<Box<dyn Token>>,
    ) -> Result<ExecutionResult, Error>;

    /// Export the current state of the plugin
    async fn export_state(&self) -> Result<Value, Error>;

    /// Import state into the plugin
    async fn import_state(&mut self, state: Value) -> Result<(), Error>;

    /// Initialize the plugin with configuration
    async fn initialize(&mut self, config: Value) -> Result<(), Error> {
        Ok(())
    }

    /// Cleanup before unloading
    async fn cleanup(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

/// Result of can_handle check
#[derive(Debug, Clone)]
pub enum CanHandleResult {
    /// Plugin can definitely handle this command
    Yes { confidence: f32 },

    /// Plugin cannot handle this command
    No { reason: String },

    /// Plugin could handle if missing tokens were provided
    Partial { missing: Vec<String> },
}

/// Command registration information
pub struct CommandRegistration {
    pub command: String,
    pub aliases: Vec<String>,
    pub requirements: CommandRequirements,
    pub priority: i32,
    pub description: String,
    pub examples: Vec<String>,
}

/// Requirements for a command
pub struct CommandRequirements {
    pub command: String,
    pub required: Vec<TokenRequirement>,
    pub optional: Vec<TokenRequirement>,
    pub defaults: Vec<TokenDefault>,
    pub validators: Vec<Arc<dyn CommandValidator>>,
    pub allow_partial: bool,
}

/// Token requirement for a command
#[derive(Clone)]
pub struct TokenRequirement {
    pub token_type: String,
    pub description: String,
    pub validator: Option<Arc<dyn Fn(&dyn Token) -> bool + Send + Sync>>,
    pub multiple: bool,
}

/// Default token to apply if missing
pub struct TokenDefault {
    pub token_type: String,
    pub default_value: Box<dyn Token>,
    pub condition: Option<Arc<dyn Fn(&[Box<dyn Token>]) -> bool + Send + Sync>>,
}

/// Command validator trait
pub trait CommandValidator: Send + Sync {
    fn validate(&self, tokens: &[Box<dyn Token>]) -> Result<(), String>;
}

/// Result of executing a command
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub plugin_id: String,
    pub command: String,
    pub journal_entries: Vec<JournalEntry>,
    pub metadata: Value,
    pub artifacts: Vec<Artifact>,
}

/// Journal entry for double-entry bookkeeping
#[derive(Debug, Clone)]
pub struct JournalEntry {
    pub date: chrono::NaiveDate,
    pub description: String,
    pub lines: Vec<JournalLine>,
    pub reference: Option<String>,
    pub tags: Vec<String>,
}

/// Line item in a journal entry
#[derive(Debug, Clone)]
pub struct JournalLine {
    pub account_code: String,
    pub account_name: Option<String>,
    pub debit: Option<rust_decimal::Decimal>,
    pub credit: Option<rust_decimal::Decimal>,
    pub reference: Option<String>,
    pub memo: Option<String>,
}

/// Artifact produced by command execution
#[derive(Debug, Clone)]
pub struct Artifact {
    pub artifact_type: String,
    pub data: Value,
    pub metadata: Option<Value>,
}

/// Plugin lifecycle trait
#[async_trait]
pub trait PluginLifecycle: Plugin {
    /// Called when plugin is loaded
    async fn on_load(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /// Called when plugin is unloaded
    async fn on_unload(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /// Called when plugin is enabled
    async fn on_enable(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /// Called when plugin is disabled
    async fn on_disable(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
