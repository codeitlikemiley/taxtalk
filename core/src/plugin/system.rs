use std::sync::Arc;
use std::path::Path;
use std::collections::HashMap;
use tokio::sync::RwLock;
use anyhow::{Error, anyhow};
use serde_json::Value;

use crate::token::{Token, TokenFactory, TokenValidator, ActionToken};
use crate::plugin::router::CommandRouter;
use crate::plugin::loader::PluginLoader;
use crate::plugin::types::{LoadedPlugin, PluginInstance};
use crate::plugin::traits::ExecutionResult;

/// Main plugin system that coordinates everything
pub struct PluginSystem {
    /// Command router for plugin dispatch
    router: Arc<RwLock<CommandRouter>>,
    
    /// Token factory for creating tokens
    token_factory: Arc<TokenFactory>,
    
    /// Token validator
    validator: Arc<TokenValidator>,
    
    /// Plugin loader
    loader: Arc<PluginLoader>,
    
    /// Loaded plugins
    plugins: Arc<RwLock<HashMap<String, Arc<LoadedPlugin>>>>,
}

impl PluginSystem {
    pub async fn new() -> Result<Self, Error> {
        let router = Arc::new(RwLock::new(CommandRouter::new()));
        let token_factory = Arc::new(TokenFactory::new());
        let validator = Arc::new(TokenValidator::new());
        let loader = Arc::new(PluginLoader::new()?);
        
        Ok(Self {
            router,
            token_factory,
            validator,
            loader,
            plugins: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Load a plugin from file or registry
    pub async fn load_plugin(&self, path: &Path) -> Result<(), Error> {
        // Load the plugin
        let plugin = self.loader.load_plugin(path).await?;
        let plugin_id = plugin.id.clone();
        
        // Register with router
        let mut router = self.router.write().await;
        router.register_plugin(plugin.clone()).await?;
        
        // Store in plugins map
        self.plugins.write().await.insert(plugin_id.clone(), plugin.clone());
        
        // Initialize the plugin
        if let PluginInstance::Native(ref _p) = &plugin.instance {
            // Initialize with config
            // Note: This would require the plugin to be mutable
            // In practice, you might handle this differently
        }
        
        println!("Loaded plugin: {plugin_id}");
        
        Ok(())
    }
    
    /// Unload a plugin
    pub async fn unload_plugin(&self, plugin_id: &str) -> Result<(), Error> {
        // Get the plugin
        let plugin = self.plugins.write().await.remove(plugin_id)
            .ok_or_else(|| anyhow!("Plugin not found: {}", plugin_id))?;
        
        // Cleanup
        if let PluginInstance::Native(ref _p) = &plugin.instance {
            // Call cleanup method
        }
        
        // Unregister from router
        let mut router = self.router.write().await;
        router.unregister_plugin(plugin_id).await?;
        
        println!("Unloaded plugin: {plugin_id}");
        
        Ok(())
    }
    
    /// Process a natural language command
    pub async fn process_command(&self, input: &str) -> Result<ExecutionResult, Error> {
        // Step 1: Tokenize the input
        let tokens = self.tokenize(input).await?;
        
        // Step 2: Validate tokens
        self.validator.validate_composition(&tokens)
            .map_err(|errors| anyhow!("Validation failed: {:?}", errors))?;
        
        // Step 3: Route to appropriate plugin
        let router = self.router.read().await;
        router.route_command(tokens).await
    }
    
    /// Tokenize input into tokens
    async fn tokenize(&self, input: &str) -> Result<Vec<Box<dyn Token>>, Error> {
        // This is a simplified tokenizer
        // In practice, you'd use a proper NLP tokenizer
        
        let mut tokens: Vec<Box<dyn Token>> = Vec::new();
        let words: Vec<&str> = input.split_whitespace().collect();
        
        let mut i = 0;
        while i < words.len() {
            let word = words[i];
            
            // Look for patterns
            if word.starts_with('@') {
                // Entity reference
                let entity_type = word.trim_start_matches('@');
                let entity_id = if i + 1 < words.len() {
                    i += 1;
                    words[i]
                } else {
                    ""
                };
                
                let position = crate::token::TokenPosition {
                    start: 0,
                    end: word.len() + entity_id.len() + 1,
                    line: 1,
                    column: 1,
                };
                
                let entity_ref = crate::token::EntityRef {
                    base: crate::token::BaseTokenFields::new(
                        format!("{word} {entity_id}"),
                        position,
                    ),
                    plugin: "core".to_string(),
                    entity_type: entity_type.to_string(),
                    entity_id: entity_id.to_string(),
                    display_name: Some(entity_id.to_string()),
                    resolver: None,
                };
                
                tokens.push(Box::new(entity_ref));
            } else if word.parse::<f64>().is_ok() {
                // Amount token
                let position = crate::token::TokenPosition {
                    start: 0,
                    end: word.len(),
                    line: 1,
                    column: 1,
                };
                
                let amount = crate::token::Amount {
                    base: crate::token::BaseTokenFields::new(word.to_string(), position),
                    value: word.parse::<rust_decimal::Decimal>().unwrap(),
                    currency: Some("PHP".to_string()),
                    format: crate::token::AmountFormat::Currency,
                };
                
                tokens.push(Box::new(amount));
            } else {
                // Check if it's a command/action
                let action_words = ["invoice", "bill", "pay", "receive", "buy", "sell", "charge"];
                if action_words.contains(&word) {
                    let position = crate::token::TokenPosition {
                        start: 0,
                        end: word.len(),
                        line: 1,
                        column: 1,
                    };
                    
                    let action = ActionToken {
                        base: crate::token::BaseTokenFields::new(word.to_string(), position),
                        verb: word.to_string(),
                        tense: crate::token::Tense::Present,
                        variations: std::collections::HashSet::new(),
                    };
                    
                    tokens.push(Box::new(action));
                }
            }
            
            i += 1;
        }
        
        Ok(tokens)
    }
    
    /// Hot-swap a plugin
    pub async fn hot_swap(&self, old_id: &str, new_path: &Path) -> Result<(), Error> {
        // Export state from old plugin
        let _old_state = {
            let plugins = self.plugins.read().await;
            if let Some(old_plugin) = plugins.get(old_id) {
                self.loader.export_plugin_state(old_plugin).await?
            } else {
                serde_json::json!({})
            }
        };
        
        // Unload old plugin
        self.unload_plugin(old_id).await?;
        
        // Load new plugin
        let new_plugin = self.loader.load_plugin(new_path).await?;
        let new_plugin_id = new_plugin.id.clone();
        
        // Import state to new plugin
        // This would require mutable access to the plugin
        // In practice, you'd handle this through a different mechanism
        
        // Register with router
        let mut router = self.router.write().await;
        router.register_plugin(new_plugin.clone()).await?;
        
        // Store in plugins map
        self.plugins.write().await.insert(new_plugin_id.clone(), new_plugin);
        
        println!("Hot-swapped {old_id} with plugin from {new_path:?}");
        
        Ok(())
    }
    
    /// Get command suggestions
    pub async fn suggest_completions(&self, partial_command: &str) -> Vec<String> {
        let router = self.router.read().await;
        router.suggest_completions(partial_command)
    }
    
    /// List all available commands
    pub async fn list_commands(&self) -> Vec<String> {
        let router = self.router.read().await;
        router.list_commands()
    }
    
    /// Get plugin information
    pub async fn get_plugin_info(&self, plugin_id: &str) -> Option<Value> {
        let plugins = self.plugins.read().await;
        plugins.get(plugin_id).map(|p| {
            serde_json::json!({
                "id": p.id,
                "name": p.manifest.name,
                "version": p.manifest.version,
                "commands": p.manifest.commands.iter()
                    .map(|c| &c.command)
                    .collect::<Vec<_>>(),
                "state": p.state,
            })
        })
    }
    
    /// List all loaded plugins
    pub async fn list_plugins(&self) -> Vec<String> {
        let plugins = self.plugins.read().await;
        plugins.keys().cloned().collect()
    }
}