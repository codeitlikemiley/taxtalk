use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::{Error, anyhow};

use crate::token::{Token, ActionToken};
use crate::plugin::traits::{CanHandleResult, ExecutionResult};
use crate::plugin::types::LoadedPlugin;

/// Routes commands to appropriate plugins
pub struct CommandRouter {
    /// Command to plugin mapping
    command_map: HashMap<String, Vec<PluginRoute>>,
    
    /// Loaded plugins
    plugins: Arc<RwLock<HashMap<String, Arc<LoadedPlugin>>>>,
}

#[derive(Clone)]
struct PluginRoute {
    plugin_id: String,
    priority: i32,
    aliases: Vec<String>,
}

impl CommandRouter {
    pub fn new() -> Self {
        Self {
            command_map: HashMap::new(),
            plugins: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register a plugin with the router
    pub async fn register_plugin(&mut self, plugin: Arc<LoadedPlugin>) -> Result<(), Error> {
        let plugin_id = plugin.id.clone();
        
        // Register all commands from the plugin
        for command_manifest in &plugin.manifest.commands {
            let route = PluginRoute {
                plugin_id: plugin_id.clone(),
                priority: 100, // Default priority, could be from manifest
                aliases: command_manifest.aliases.clone(),
            };
            
            // Register main command
            self.command_map
                .entry(command_manifest.command.clone())
                .or_insert_with(Vec::new)
                .push(route.clone());
            
            // Register aliases
            for alias in &command_manifest.aliases {
                self.command_map
                    .entry(alias.clone())
                    .or_insert_with(Vec::new)
                    .push(route.clone());
            }
        }
        
        // Store the plugin
        self.plugins.write().await.insert(plugin_id, plugin);
        
        Ok(())
    }
    
    /// Unregister a plugin
    pub async fn unregister_plugin(&mut self, plugin_id: &str) -> Result<(), Error> {
        // Remove from command map
        self.command_map.retain(|_, routes| {
            routes.retain(|route| route.plugin_id != plugin_id);
            !routes.is_empty()
        });
        
        // Remove from plugins
        self.plugins.write().await.remove(plugin_id);
        
        Ok(())
    }
    
    /// Route a command to the appropriate plugin
    pub async fn route_command(&self, tokens: Vec<Box<dyn Token>>) -> Result<ExecutionResult, Error> {
        // Find the action token
        let action = tokens.iter()
            .find_map(|t| t.as_any().downcast_ref::<ActionToken>())
            .ok_or_else(|| anyhow!("No action token found in command"))?
            .clone();
        
        // Find plugins that can handle this command
        let routes = self.command_map
            .get(&action.verb)
            .ok_or_else(|| anyhow!("Unknown command: {}", action.verb))?;
        
        // Sort by priority (create a vector of references)
        let mut sorted_routes: Vec<&PluginRoute> = routes.iter().collect();
        sorted_routes.sort_by_key(|r| -r.priority);
        
        // Try each plugin in priority order
        let plugins = self.plugins.read().await;
        
        for route in sorted_routes {
            let plugin = plugins.get(&route.plugin_id)
                .ok_or_else(|| anyhow!("Plugin not found: {}", route.plugin_id))?;
            
            // Check if plugin can handle
            let can_handle = match &plugin.instance {
                crate::plugin::types::PluginInstance::Native(p) => {
                    p.can_handle(&action, &tokens)
                }
                crate::plugin::types::PluginInstance::Wasm(_) => {
                    // For WASM, we'd need to call through the WASM boundary
                    CanHandleResult::No { reason: "WASM plugin support not yet implemented".to_string() }
                }
            };
            
            match can_handle {
                CanHandleResult::Yes { .. } => {
                    // Execute the command
                    return self.execute_plugin_command(plugin, &action, tokens).await;
                }
                CanHandleResult::Partial { missing } => {
                    // Could prompt for missing tokens
                    return Err(anyhow!("Missing required tokens: {:?}", missing));
                }
                CanHandleResult::No { .. } => {
                    // Try next plugin
                    continue;
                }
            }
        }
        
        Err(anyhow!("No plugin could handle command: {}", action.verb))
    }
    
    /// Execute a command on a specific plugin
    async fn execute_plugin_command(
        &self,
        plugin: &Arc<LoadedPlugin>,
        action: &ActionToken,
        tokens: Vec<Box<dyn Token>>,
    ) -> Result<ExecutionResult, Error> {
        match &plugin.instance {
            crate::plugin::types::PluginInstance::Native(p) => {
                p.execute(action, tokens).await
            }
            crate::plugin::types::PluginInstance::Wasm(_) => {
                Err(anyhow!("WASM plugin execution not yet implemented"))
            }
        }
    }
    
    /// Hot swap a plugin
    pub async fn hot_swap_plugin(
        &mut self,
        old_plugin_id: &str,
        new_plugin: Arc<LoadedPlugin>,
    ) -> Result<(), Error> {
        // First unregister the old plugin
        self.unregister_plugin(old_plugin_id).await?;
        
        // Then register the new one
        self.register_plugin(new_plugin).await?;
        
        Ok(())
    }
    
    /// Get suggestions for command completion
    pub fn suggest_completions(&self, partial_command: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for (command, _) in &self.command_map {
            if command.starts_with(partial_command) {
                suggestions.push(command.clone());
            }
        }
        
        suggestions.sort();
        suggestions.dedup();
        suggestions
    }
    
    /// List all available commands
    pub fn list_commands(&self) -> Vec<String> {
        let mut commands: Vec<String> = self.command_map.keys().cloned().collect();
        commands.sort();
        commands.dedup();
        commands
    }
}