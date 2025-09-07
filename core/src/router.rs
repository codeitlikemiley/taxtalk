use std::collections::HashMap;
use anyhow::{Result, anyhow};

// Define simplified Token enum for routing
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Text(String),
    Verb(String),
    Amount { value: f64, currency: String },
    EntityRef { entity_type: String, identifier: String },
    Date(String),
    Identifier(String),
}

#[derive(Debug, Clone)]
struct Route {
    plugin_id: String,
    command: String,
    aliases: Vec<String>,
}

pub struct CommandRouter {
    routes: HashMap<String, Route>,  // command/alias -> route
    plugin_routes: HashMap<String, Vec<String>>,  // plugin_id -> commands
}

impl Default for CommandRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandRouter {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            plugin_routes: HashMap::new(),
        }
    }
    
    /// Register a command with its aliases for a plugin
    pub fn register_command(&mut self, plugin_id: &str, command: &str, aliases: &[String]) {
        let route = Route {
            plugin_id: plugin_id.to_string(),
            command: command.to_string(),
            aliases: aliases.to_vec(),
        };
        
        // Register main command
        self.routes.insert(command.to_string(), route.clone());
        
        // Register aliases
        for alias in aliases {
            self.routes.insert(alias.clone(), route.clone());
        }
        
        // Track plugin's commands
        self.plugin_routes
            .entry(plugin_id.to_string())
            .or_default()
            .push(command.to_string());
    }
    
    /// Remove all routes for a plugin
    pub fn unregister_plugin(&mut self, plugin_id: &str) {
        // Find all routes for this plugin
        let routes_to_remove: Vec<String> = self.routes
            .iter()
            .filter(|(_, route)| route.plugin_id == plugin_id)
            .map(|(key, _)| key.clone())
            .collect();
        
        // Remove them
        for key in routes_to_remove {
            self.routes.remove(&key);
        }
        
        // Remove from plugin routes
        self.plugin_routes.remove(plugin_id);
    }
    
    /// Route tokens to the appropriate plugin
    pub fn route(&self, tokens: &[Token]) -> Result<String> {
        // Look for command tokens (verbs, actions)
        for token in tokens {
            // Check if this token matches any registered command
            if let Token::Verb(verb) = token {
                if let Some(route) = self.routes.get(verb) {
                    return Ok(route.plugin_id.clone());
                }
            }
            
            // Also check text tokens that might be commands
            if let Token::Text(text) = token {
                if let Some(route) = self.routes.get(text) {
                    return Ok(route.plugin_id.clone());
                }
            }
        }
        
        // Try to match based on entity references
        for token in tokens {
            if let Token::EntityRef { entity_type, .. } = token {
                // Route based on entity type
                match entity_type.as_str() {
                    "invoice" => return Ok("invoice".to_string()),
                    "payment" => return Ok("payments".to_string()),
                    "company" => return Ok("company".to_string()),
                    _ => {}
                }
            }
        }
        
        Err(anyhow!("No plugin found to handle this command"))
    }
    
    /// Get all commands for a plugin
    pub fn get_plugin_commands(&self, plugin_id: &str) -> Vec<String> {
        self.plugin_routes
            .get(plugin_id)
            .cloned()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_router_registration() {
        let mut router = CommandRouter::new();
        
        router.register_command("invoice", "create", &["new".to_string(), "add".to_string()]);
        
        // Test main command
        let tokens = vec![Token::Verb("create".to_string())];
        assert_eq!(router.route(&tokens).unwrap(), "invoice");
        
        // Test alias
        let tokens = vec![Token::Verb("new".to_string())];
        assert_eq!(router.route(&tokens).unwrap(), "invoice");
    }
    
    #[test]
    fn test_router_unregister() {
        let mut router = CommandRouter::new();
        
        router.register_command("invoice", "create", &["new".to_string()]);
        router.register_command("payment", "pay", &["receive".to_string()]);
        
        // Verify both work
        assert!(router.route(&[Token::Verb("create".to_string())]).is_ok());
        assert!(router.route(&[Token::Verb("pay".to_string())]).is_ok());
        
        // Unregister invoice plugin
        router.unregister_plugin("invoice");
        
        // Invoice commands should fail
        assert!(router.route(&[Token::Verb("create".to_string())]).is_err());
        
        // Payment commands should still work
        assert!(router.route(&[Token::Verb("pay".to_string())]).is_ok());
    }
}