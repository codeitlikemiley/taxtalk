// Core Crux + WASM Plugin Architecture
// Main entry point for the plugin system

pub mod event_bus;
pub mod manifest;
pub mod plugin_loader;
pub mod schema;
pub mod tokenizer;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginResponse {
    pub plugin_id: String,
    pub command: String,
    pub result: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct PluginSystem {
    plugin_loader: plugin_loader::PluginLoader,
    event_bus: event_bus::EventBus,
    tokenizer: tokenizer::CommandTokenizer,
    schema_validator: schema::SchemaValidator,
}

impl PluginSystem {
    pub fn new() -> Self {
        Self {
            plugin_loader: plugin_loader::PluginLoader::new(),
            event_bus: event_bus::EventBus::new(),
            tokenizer: tokenizer::CommandTokenizer::new(),
            schema_validator: schema::SchemaValidator::new(),
        }
    }

    pub async fn process_message(&mut self, message: ChatMessage) -> Result<PluginResponse, Box<dyn std::error::Error>> {
        // Parse the message
        let parsed_command = self.tokenizer.parse_command(&message.content)?;

        // Validate against schemas if available
        if let Some(entity) = &parsed_command.entity {
            // Add schema validation here when schemas are loaded
        }

        // Route to appropriate plugin via event bus
        let event = parsed_command.to_event()?;
        let response = self.event_bus.dispatch_event(event).await?;

        Ok(PluginResponse {
            plugin_id: parsed_command.plugin,
            command: parsed_command.action,
            result: response,
            timestamp: chrono::Utc::now(),
        })
    }

    pub fn load_plugin(&mut self, manifest_path: &std::path::Path, wasm_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let manifest = manifest::PluginManifest::from_file(manifest_path)?;
        self.plugin_loader.load_plugin(wasm_path, manifest)?;
        Ok(())
    }

    pub fn list_plugins(&self) -> Vec<String> {
        self.plugin_loader.list_plugins().into_iter().map(|s| s.to_string()).collect()
    }

    pub fn add_schema(&mut self, name: String, schema: schema::JsonSchema) {
        self.schema_validator.add_schema(name, schema);
    }
}

impl Default for PluginSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_system_creation() {
        let system = PluginSystem::new();
        assert!(system.list_plugins().is_empty());
    }

    #[test]
    fn test_chat_message_creation() {
        let message = ChatMessage {
            id: Uuid::new_v4(),
            content: "@invoice create client:uriah".to_string(),
            timestamp: chrono::Utc::now(),
            user_id: Some("user123".to_string()),
        };
        assert_eq!(message.content, "@invoice create client:uriah");
    }
}