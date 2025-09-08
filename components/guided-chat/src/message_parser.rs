use crate::types::*;
use std::collections::HashMap;

/// Parses and formats chat messages
#[derive(Clone)]
pub struct MessageParser {
    // Could add configuration for markdown, emoji support, etc.
}

impl MessageParser {
    pub fn new() -> Self {
        Self {}
    }
    
    /// Parse user input based on expected input type
    pub fn parse_input(&self, input: &str, input_type: &InputType) -> Result<serde_json::Value, String> {
        match input_type {
            InputType::Number => {
                let num = input.parse::<f64>()
                    .map_err(|_| "Invalid number format".to_string())?;
                Ok(serde_json::json!(num))
            }
            InputType::Currency => {
                // Remove currency symbols and commas
                let cleaned = input
                    .replace("₱", "")
                    .replace("PHP", "")
                    .replace(",", "")
                    .trim()
                    .to_string();
                
                let amount = cleaned.parse::<f64>()
                    .map_err(|_| "Invalid currency amount".to_string())?;
                
                Ok(serde_json::json!({
                    "amount": amount,
                    "currency": "PHP"
                }))
            }
            InputType::Date => {
                // Basic date parsing - could be enhanced with chrono
                Ok(serde_json::json!(input))
            }
            InputType::Boolean => {
                let lower = input.to_lowercase();
                let value = match lower.as_str() {
                    "yes" | "y" | "true" | "1" | "oo" => true,  // "oo" is Filipino for yes
                    "no" | "n" | "false" | "0" | "hindi" => false,  // "hindi" is Filipino for no
                    _ => return Err("Please answer yes or no".to_string()),
                };
                Ok(serde_json::json!(value))
            }
            InputType::Select(options) => {
                // Match input against options
                let lower_input = input.to_lowercase();
                for option in options {
                    if option.value.to_lowercase() == lower_input ||
                       option.label.to_lowercase() == lower_input {
                        return Ok(serde_json::json!(option.value));
                    }
                }
                Err(format!("Please select one of the available options"))
            }
            InputType::MultiSelect(_) => {
                // Parse comma-separated values
                let values: Vec<String> = input
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                Ok(serde_json::json!(values))
            }
            InputType::EntityRef(entity_type) => {
                // Return the input as-is for entity lookup
                Ok(serde_json::json!({
                    "entity_type": entity_type,
                    "query": input
                }))
            }
            _ => {
                // Default: return as string
                Ok(serde_json::json!(input))
            }
        }
    }
    
    /// Format a value for display based on input type
    pub fn format_value(&self, value: &serde_json::Value, input_type: &InputType) -> String {
        match input_type {
            InputType::Currency => {
                if let Some(amount) = value.as_f64() {
                    format!("₱{:,.2}", amount)
                } else if let Some(obj) = value.as_object() {
                    if let Some(amount) = obj.get("amount").and_then(|v| v.as_f64()) {
                        format!("₱{:,.2}", amount)
                    } else {
                        value.to_string()
                    }
                } else {
                    value.to_string()
                }
            }
            InputType::Boolean => {
                if let Some(b) = value.as_bool() {
                    if b { "Yes" } else { "No" }.to_string()
                } else {
                    value.to_string()
                }
            }
            InputType::Date => {
                // Could format with chrono
                value.to_string().replace("\"", "")
            }
            _ => {
                value.to_string().replace("\"", "")
            }
        }
    }
    
    /// Extract entities from a message (e.g., @client mentions)
    pub fn extract_entities(&self, message: &str) -> Vec<(String, String)> {
        let mut entities = Vec::new();
        
        // Look for @mentions
        let parts: Vec<&str> = message.split_whitespace().collect();
        for (i, part) in parts.iter().enumerate() {
            if part.starts_with('@') {
                let entity_type = part[1..].to_string();
                
                // Get the next word(s) as the entity name
                let mut entity_name = String::new();
                for j in (i + 1)..parts.len() {
                    if parts[j].starts_with('@') {
                        break;
                    }
                    if !entity_name.is_empty() {
                        entity_name.push(' ');
                    }
                    entity_name.push_str(parts[j]);
                }
                
                if !entity_name.is_empty() {
                    entities.push((entity_type, entity_name));
                }
            }
        }
        
        entities
    }
    
    /// Parse quick commands (e.g., /skip, /back, /help)
    pub fn parse_command(&self, message: &str) -> Option<ActionType> {
        if !message.starts_with('/') {
            return None;
        }
        
        let command = message[1..].to_lowercase();
        match command.as_str() {
            "skip" => Some(ActionType::Skip),
            "back" | "prev" | "previous" => Some(ActionType::Back),
            "cancel" | "quit" | "exit" => Some(ActionType::Cancel),
            "help" | "?" => Some(ActionType::Custom("help".to_string())),
            _ => None,
        }
    }
    
    /// Format a summary of collected data
    pub fn format_summary(&self, data: &HashMap<String, serde_json::Value>, workflow: &Workflow) -> String {
        let mut summary = String::from("Summary:\n");
        
        for step in &workflow.steps {
            if let Some(value) = data.get(&step.id) {
                let formatted = self.format_value(value, &step.input_type);
                summary.push_str(&format!("• {}: {}\n", step.name, formatted));
            }
        }
        
        summary
    }
}