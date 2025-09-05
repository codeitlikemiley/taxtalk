// Chat command tokenizer using nom and tokenizers
use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, digit1, space0, space1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, tuple},
};
use crate::event_bus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCommand {
    pub plugin: String,
    pub action: String,
    pub entity: Option<Entity>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub metadata: CommandMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandMetadata {
    pub user_context: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Entity {
    Client(String),
    Invoice(String),
    Payment(String),
    Item(String),
    Company(String),
    Quantity(f64, Option<String>), // amount, unit
    Money(f64, String), // amount, currency
}

impl ParsedCommand {
    pub fn to_event(&self) -> Result<event_bus::Event, Box<dyn std::error::Error>> {
        Ok(event_bus::Event {
            id: uuid::Uuid::new_v4(),
            plugin_id: self.plugin.clone(),
            event_type: self.action.clone(),
            payload: serde_json::to_value(&self.parameters)?,
            metadata: self.metadata.clone(),
            timestamp: chrono::Utc::now(),
        })
    }
}

pub struct CommandTokenizer {
    // Add tokenizer state if needed
}

impl CommandTokenizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse_command(&self, input: &str) -> Result<ParsedCommand, ParseError> {
        // Try structured parsing first
        match self.parse_structured(input) {
            Ok(cmd) => Ok(cmd),
            Err(_) => {
                // Fall back to natural language parsing
                self.parse_natural_language(input)
            }
        }
    }

    fn parse_structured(&self, input: &str) -> Result<ParsedCommand, ParseError> {
        let trimmed = input.trim();

        if !trimmed.starts_with('@') {
            return Err(ParseError::InvalidFormat("Command must start with @".to_string()));
        }

        // Parse @plugin action entity:params
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(ParseError::InvalidFormat("Command must have at least plugin and action".to_string()));
        }

        let plugin = parts[0].trim_start_matches('@').to_string();
        let action = parts[1].to_string();

        let mut parameters = HashMap::new();
        let mut entity = None;

        // Parse remaining parts as parameters
        for part in &parts[2..] {
            if let Some((key, value)) = part.split_once(':') {
                let parsed_value = self.parse_parameter_value(value)?;
                parameters.insert(key.to_string(), parsed_value);
            }
        }

        // Try to extract entity from parameters
        if let Some(client) = parameters.get("client") {
            if let serde_json::Value::String(name) = client {
                entity = Some(Entity::Client(name.clone()));
            }
        }

        Ok(ParsedCommand {
            plugin,
            action,
            entity,
            parameters,
            metadata: CommandMetadata {
                user_context: None,
                timestamp: chrono::Utc::now(),
                confidence: 1.0,
            },
        })
    }

    fn parse_natural_language(&self, input: &str) -> Result<ParsedCommand, ParseError> {
        let lower = input.to_lowercase();

        // Simple keyword matching for demo
        let plugin = if lower.contains("invoice") {
            "invoice"
        } else if lower.contains("client") {
            "client"
        } else if lower.contains("payment") {
            "payment"
        } else {
            return Err(ParseError::UnknownPlugin("Could not determine plugin from text".to_string()));
        };

        let action = if lower.contains("create") || lower.contains("add") || lower.contains("new") {
            "create"
        } else if lower.contains("update") || lower.contains("change") {
            "update"
        } else if lower.contains("delete") || lower.contains("remove") {
            "delete"
        } else {
            "create" // default
        };

        // Extract entities from text
        let entity = self.extract_entity_from_text(&lower);

        Ok(ParsedCommand {
            plugin: plugin.to_string(),
            action: action.to_string(),
            entity,
            parameters: HashMap::new(),
            metadata: CommandMetadata {
                user_context: None,
                timestamp: chrono::Utc::now(),
                confidence: 0.8, // Lower confidence for NLP
            },
        })
    }

    fn parse_parameter_value(&self, value: &str) -> Result<serde_json::Value, ParseError> {
        // Try to parse as number
        if let Ok(num) = value.parse::<f64>() {
            return Ok(serde_json::Value::Number(serde_json::Number::from_f64(num).unwrap()));
        }

        // Try to parse as boolean
        if value == "true" {
            return Ok(serde_json::Value::Bool(true));
        }
        if value == "false" {
            return Ok(serde_json::Value::Bool(false));
        }

        // Default to string
        Ok(serde_json::Value::String(value.to_string()))
    }

    fn extract_entity_from_text(&self, text: &str) -> Option<Entity> {
        // Simple entity extraction
        let words: Vec<&str> = text.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            match *word {
                "client" | "customer" => {
                    if let Some(name) = words.get(i + 1) {
                        return Some(Entity::Client(name.to_string()));
                    }
                }
                "invoice" | "bill" => {
                    if let Some(id) = words.get(i + 1) {
                        return Some(Entity::Invoice(id.to_string()));
                    }
                }
                _ => {}
            }
        }

        None
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid command format: {0}")]
    InvalidFormat(String),

    #[error("Unknown plugin: {0}")]
    UnknownPlugin(String),

    #[error("Parameter parsing failed: {0}")]
    ParameterError(String),
}

impl From<nom::Err<nom::error::Error<&str>>> for ParseError {
    fn from(_: nom::Err<nom::error::Error<&str>>) -> Self {
        ParseError::InvalidFormat("Parsing failed".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structured_command() {
        let tokenizer = CommandTokenizer::new();
        let result = tokenizer.parse_command("@invoice create client:uriah amount:1000").unwrap();

        assert_eq!(result.plugin, "invoice");
        assert_eq!(result.action, "create");
        assert!(matches!(result.entity, Some(Entity::Client(_))));
    }

    #[test]
    fn test_natural_language() {
        let tokenizer = CommandTokenizer::new();
        let result = tokenizer.parse_command("create an invoice for uriah").unwrap();

        assert_eq!(result.plugin, "invoice");
        assert_eq!(result.action, "create");
        assert!(result.metadata.confidence < 1.0);
    }

    #[test]
    fn test_invalid_command() {
        let tokenizer = CommandTokenizer::new();
        let result = tokenizer.parse_command("invalid command");

        assert!(result.is_err());
    }
}