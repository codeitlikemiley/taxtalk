use std::sync::Arc;
use tokio::sync::RwLock;
use wasmtime::{Engine, Module, Store, Instance, Linker};
use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::tokens::*;
use crate::event_bus;

/// Plugin system that manages hot-swappable plugins
pub struct PluginSystem {
    /// Command router for plugin dispatch
    router: Arc<RwLock<CommandRouter>>,

    /// Token factory for creating tokens
    token_factory: Arc<TokenFactory>,

    /// Token validator
    validator: Arc<TokenValidator>,

    /// Wasmtime engine for WASM plugins
    engine: Engine,

    /// Loaded plugins
    plugins: Arc<RwLock<HashMap<String, LoadedPlugin>>>,
}

#[derive(Debug)]
pub struct LoadedPlugin {
    pub id: String,
    pub instance: PluginInstance,
    pub manifest: PluginManifest,
    pub state: PluginState,
}

#[derive(Debug)]
pub enum PluginInstance {
    Native(Box<dyn Plugin>),
    Wasm(Instance),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginState {
    Loading,
    Active,
    Error(String),
    Unloaded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub plugin_type: PluginType,
    pub entry_point: String,
    pub commands: Vec<CommandManifest>,
    pub token_types: Vec<TokenTypeManifest>,
    pub dependencies: Vec<PluginDependency>,
    pub config_schema: Option<Value>,
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
    pub validation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenDefaultManifest {
    pub token_type: String,
    pub default_value: Value,
    pub condition: Option<String>,
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

#[async_trait]
pub trait Plugin: Send + Sync {
    fn id(&self) -> &str;
    fn version(&self) -> &str;
    fn commands(&self) -> Vec<CommandRegistration>;
    fn can_handle(&self, command: &CommandToken, tokens: &[Box<dyn Token>]) -> CanHandleResult;
    async fn execute(&self, command: &CommandToken, tokens: Vec<Box<dyn Token>>) -> Result<ExecutionResult, Error>;
    async fn export_state(&self) -> Result<Value, Error>;
    async fn import_state(&mut self, state: Value) -> Result<(), Error>;
}

#[derive(Debug)]
pub enum CanHandleResult {
    Yes { confidence: f32 },
    No { reason: String },
    Partial { missing: Vec<String> },
}

#[derive(Debug, Clone)]
pub struct CommandRegistration {
    pub command: String,
    pub aliases: Vec<String>,
    pub requirements: CommandRequirements,
    pub priority: i32,
    pub description: String,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CommandRequirements {
    pub required: Vec<TokenRequirement>,
    pub optional: Vec<TokenRequirement>,
    pub defaults: Vec<TokenDefault>,
    pub validators: Vec<Box<dyn Fn(&[Box<dyn Token>]) -> Result<(), Error> + Send + Sync>>,
    pub allow_partial: bool,
}

#[derive(Debug, Clone)]
pub struct TokenRequirement {
    pub token_type: String,
    pub description: String,
    pub validator: Option<Arc<dyn Fn(&dyn Token) -> bool + Send + Sync>>,
    pub multiple: bool,
}

#[derive(Debug, Clone)]
pub struct TokenDefault {
    pub token_type: String,
    pub default_value: Box<dyn Token>,
    pub condition: Option<Arc<dyn Fn(&[Box<dyn Token>]) -> bool + Send + Sync>>,
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub plugin_id: String,
    pub command: String,
    pub journal_entries: Vec<JournalEntry>,
    pub metadata: Value,
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub date: DateTime<Utc>,
    pub description: String,
    pub lines: Vec<JournalLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalLine {
    pub account_code: String,
    pub debit: Option<rust_decimal::Decimal>,
    pub credit: Option<rust_decimal::Decimal>,
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub artifact_type: String,
    pub data: Value,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Plugin not found: {0}")]
    PluginNotFound(String),

    #[error("Plugin load failed: {0}")]
    PluginLoadFailed(String),

    #[error("Command not supported: {0}")]
    CommandNotSupported(String),

    #[error("Token validation failed: {0}")]
    TokenValidationFailed(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("WASM error: {0}")]
    Wasm(String),
}

pub struct CommandRouter {
    routes: HashMap<String, Vec<RouteEntry>>,
}

#[derive(Debug)]
struct RouteEntry {
    plugin_id: String,
    priority: i32,
    handler: Arc<dyn Fn(&CommandToken, Vec<Box<dyn Token>>) -> Result<ExecutionResult, Error> + Send + Sync>,
}

impl PluginSystem {
    pub async fn new() -> Result<Self, Error> {
        let engine = Engine::default();
        let router = Arc::new(RwLock::new(CommandRouter::new()));
        let token_factory = Arc::new(TokenFactory::new());
        let validator = Arc::new(TokenValidator::new());

        Ok(Self {
            router,
            token_factory,
            validator,
            engine,
            plugins: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Load a plugin from file or registry
    pub async fn load_plugin(&self, path: &Path) -> Result<(), Error> {
        let manifest = self.load_manifest(path)?;

        match manifest.plugin_type {
            PluginType::Native => {
                self.load_native_plugin(path, manifest).await
            }
            PluginType::Wasm => {
                self.load_wasm_plugin(path, manifest).await
            }
        }
    }

    /// Process a natural language command
    pub async fn process_command(&self, input: &str) -> Result<ExecutionResult, Error> {
        // Step 1: Tokenize the input
        let tokens = self.tokenize(input).await?;

        // Step 2: Validate tokens
        self.validator.validate_composition(&tokens)?;

        // Step 3: Route to appropriate plugin
        let router = self.router.read().await;
        router.route_command(tokens).await
    }

    /// Hot-swap a plugin
    pub async fn hot_swap(&self, old_id: &str, new_path: &Path) -> Result<(), Error> {
        // Load new plugin
        let new_manifest = self.load_manifest(new_path)?;
        let new_plugin = self.create_plugin_instance(new_path, &new_manifest).await?;

        // Export state from old plugin
        let mut plugins = self.plugins.write().await;
        let old_state = if let Some(old) = plugins.get(old_id) {
            match &old.instance {
                PluginInstance::Native(plugin) => plugin.export_state().await?,
                PluginInstance::Wasm(instance) => self.export_wasm_state(instance).await?,
            }
        } else {
            json!({})
        };

        // Import state to new plugin
        match &new_plugin {
            PluginInstance::Native(plugin) => {
                plugin.import_state(old_state).await?;
            }
            PluginInstance::Wasm(instance) => {
                self.import_wasm_state(instance, old_state).await?;
            }
        }

        // Update router
        let mut router = self.router.write().await;
        router.hot_swap_plugin(old_id, new_plugin).await?;

        Ok(())
    }

    fn load_manifest(&self, path: &Path) -> Result<PluginManifest, Error> {
        let manifest_path = path.join("plugin.json");
        let content = std::fs::read_to_string(manifest_path)?;
        let manifest: PluginManifest = serde_json::from_str(&content)?;
        Ok(manifest)
    }

    async fn load_native_plugin(&self, path: &Path, manifest: PluginManifest) -> Result<(), Error> {
        // For native plugins, we would dynamically load the library
        // This is a simplified version
        let plugin = self.create_native_plugin(&manifest)?;
        let loaded_plugin = LoadedPlugin {
            id: manifest.id.clone(),
            instance: PluginInstance::Native(plugin),
            manifest,
            state: PluginState::Active,
        };

        let mut plugins = self.plugins.write().await;
        plugins.insert(loaded_plugin.id.clone(), loaded_plugin);

        Ok(())
    }

    async fn load_wasm_plugin(&self, path: &Path, manifest: PluginManifest) -> Result<(), Error> {
        let wasm_path = path.join(&manifest.entry_point);
        let module = Module::from_file(&self.engine, wasm_path)?;

        let mut store = Store::new(&self.engine, ());
        let instance = Linker::new(&self.engine)
            .instantiate(&mut store, &module)?;

        let loaded_plugin = LoadedPlugin {
            id: manifest.id.clone(),
            instance: PluginInstance::Wasm(instance),
            manifest,
            state: PluginState::Active,
        };

        let mut plugins = self.plugins.write().await;
        plugins.insert(loaded_plugin.id.clone(), loaded_plugin);

        Ok(())
    }

    fn create_native_plugin(&self, manifest: &PluginManifest) -> Result<Box<dyn Plugin>, Error> {
        // This would dynamically load the plugin
        // For now, return a placeholder
        Err(Error::PluginLoadFailed("Native plugin loading not implemented".to_string()))
    }

    async fn create_plugin_instance(&self, path: &Path, manifest: &PluginManifest) -> Result<PluginInstance, Error> {
        match manifest.plugin_type {
            PluginType::Native => {
                let plugin = self.create_native_plugin(manifest)?;
                Ok(PluginInstance::Native(plugin))
            }
            PluginType::Wasm => {
                let wasm_path = path.join(&manifest.entry_point);
                let module = Module::from_file(&self.engine, wasm_path)?;
                let mut store = Store::new(&self.engine, ());
                let instance = Linker::new(&self.engine)
                    .instantiate(&mut store, &module)?;
                Ok(PluginInstance::Wasm(instance))
            }
        }
    }

    async fn tokenize(&self, input: &str) -> Result<Vec<Box<dyn Token>>, Error> {
        // Use the tokenizer to parse input into tokens
        // This is a simplified version
        Ok(vec![])
    }

    async fn export_wasm_state(&self, instance: &Instance) -> Result<Value, Error> {
        // Export state from WASM plugin
        // This would call a WASM export function
        Ok(json!({}))
    }

    async fn import_wasm_state(&self, instance: &Instance, state: Value) -> Result<(), Error> {
        // Import state to WASM plugin
        // This would call a WASM export function
        Ok(())
    }
}

impl CommandRouter {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub async fn route_command(&self, tokens: Vec<Box<dyn Token>>) -> Result<ExecutionResult, Error> {
        // Find the best plugin to handle this command
        // This is a simplified version
        Err(Error::CommandNotSupported("No plugin found for command".to_string()))
    }

    pub async fn hot_swap_plugin(&self, old_id: &str, new_plugin: PluginInstance) -> Result<(), Error> {
        // Update routing for the new plugin
        Ok(())
    }
}

impl Default for PluginSystem {
    fn default() -> Self {
        tokio::runtime::Handle::current().block_on(Self::new()).unwrap()
    }
}