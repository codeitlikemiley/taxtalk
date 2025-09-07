use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::token_schema::{TokenSchema, TokenDefinition};
use crate::tokenizer::Tokenizer;

/// Result of validating a command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub action: Option<String>,
    pub entity: Option<String>,
    pub plugin: Option<String>,
    pub missing_required: Vec<MissingToken>,
    pub missing_optional: Vec<MissingToken>,
    pub provided_tokens: HashMap<String, ProvidedToken>,
    pub defaults_applied: HashMap<String, serde_json::Value>,
    pub next_token: Option<String>,
}

/// Information about a missing token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingToken {
    pub name: String,
    pub definition: TokenDefinition,
}

/// Information about a provided token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvidedToken {
    pub raw_value: String,
    pub parsed_value: serde_json::Value,
    pub resolved_entity: Option<serde_json::Value>,
}

pub struct TokenValidator {
    tokenizer: Tokenizer,
    schemas: HashMap<(String, String), TokenSchema>,
}

impl Default for TokenValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenValidator {
    pub fn new() -> Self {
        let mut schemas = HashMap::new();
        
        // Register built-in schemas
        schemas.insert(
            ("invoice".to_string(), "create".to_string()),
            TokenSchema::invoice_create(),
        );
        schemas.insert(
            ("payment".to_string(), "create".to_string()),
            TokenSchema::payment_create(),
        );
        
        Self {
            tokenizer: Tokenizer::new(),
            schemas,
        }
    }
    
    /// Validate a command and identify missing tokens
    pub async fn validate(&self, command: &str) -> Result<ValidationResult> {
        // Parse the command to identify action and entity
        let (action, entity, plugin) = self.parse_command_intent(command)?;
        
        // Get the appropriate schema
        let schema = self.schemas
            .get(&(plugin.clone(), action.clone()))
            .ok_or_else(|| anyhow::anyhow!("No schema found for {}/{}", plugin, action))?;
        
        // Extract tokens from the command
        let provided_tokens = self.extract_tokens(command, schema)?;
        
        // Identify missing required tokens
        let mut missing_required = Vec::new();
        for token_def in &schema.required_tokens {
            if !provided_tokens.contains_key(&token_def.name) {
                // Check if there's a default value
                if token_def.default.is_none() {
                    missing_required.push(MissingToken {
                        name: token_def.name.clone(),
                        definition: token_def.clone(),
                    });
                }
            }
        }
        
        // Identify missing optional tokens
        let mut missing_optional = Vec::new();
        for token_def in &schema.optional_tokens {
            if !provided_tokens.contains_key(&token_def.name) {
                missing_optional.push(MissingToken {
                    name: token_def.name.clone(),
                    definition: token_def.clone(),
                });
            }
        }
        
        // Apply defaults
        let mut defaults_applied = HashMap::new();
        for token_def in schema.required_tokens.iter().chain(schema.optional_tokens.iter()) {
            if let Some(default) = &token_def.default {
                if !provided_tokens.contains_key(&token_def.name) {
                    defaults_applied.insert(token_def.name.clone(), default.clone());
                }
            }
        }
        
        // Determine next token to collect
        let next_token = missing_required.first().map(|t| t.name.clone());
        
        // Check if command is valid (all required tokens present or have defaults)
        let valid = missing_required.is_empty() || 
                   missing_required.iter().all(|t| defaults_applied.contains_key(&t.name));
        
        Ok(ValidationResult {
            valid,
            action: Some(action),
            entity: Some(entity),
            plugin: Some(plugin),
            missing_required,
            missing_optional,
            provided_tokens,
            defaults_applied,
            next_token,
        })
    }
    
    /// Parse command to identify action, entity, and plugin
    fn parse_command_intent(&self, command: &str) -> Result<(String, String, String)> {
        let lower = command.to_lowercase();
        
        // Detect action
        let action = if lower.contains("create") || lower.contains("new") || lower.contains("add") {
            "create"
        } else if lower.contains("update") || lower.contains("edit") || lower.contains("modify") {
            "update"
        } else if lower.contains("delete") || lower.contains("remove") || lower.contains("cancel") {
            "delete"
        } else if lower.contains("list") || lower.contains("show") || lower.contains("get") {
            "list"
        } else if lower.contains("pay") || lower.contains("paid") {
            "create" // Payment creation
        } else {
            "create" // Default action
        };
        
        // Detect entity and plugin
        let (entity, plugin) = if lower.contains("invoice") {
            ("invoice", "invoice")
        } else if lower.contains("payment") || lower.contains("pay") {
            ("payment", "payment")
        } else if lower.contains("client") || lower.contains("customer") {
            ("client", "client")
        } else if lower.contains("expense") {
            ("expense", "expense")
        } else if lower.contains("supplier") || lower.contains("vendor") {
            ("supplier", "supplier")
        } else {
            // Try to infer from context
            ("unknown", "unknown")
        };
        
        Ok((action.to_string(), entity.to_string(), plugin.to_string()))
    }
    
    /// Extract tokens from command text
    fn extract_tokens(&self, command: &str, schema: &TokenSchema) -> Result<HashMap<String, ProvidedToken>> {
        let mut tokens = HashMap::new();
        let lower = command.to_lowercase();
        
        // Extract client/customer name
        if let Some(client) = self.extract_client_name(command) {
            tokens.insert("client".to_string(), ProvidedToken {
                raw_value: client.clone(),
                parsed_value: serde_json::json!(client),
                resolved_entity: None, // Would be resolved from database
            });
        }
        
        // Extract amount
        if let Some(amount) = self.extract_amount(command) {
            tokens.insert("amount".to_string(), ProvidedToken {
                raw_value: amount.to_string(),
                parsed_value: serde_json::json!(amount),
                resolved_entity: None,
            });
        }
        
        // Extract payment method
        if lower.contains("gcash") {
            tokens.insert("method".to_string(), ProvidedToken {
                raw_value: "gcash".to_string(),
                parsed_value: serde_json::json!("gcash"),
                resolved_entity: None,
            });
        } else if lower.contains("maya") {
            tokens.insert("method".to_string(), ProvidedToken {
                raw_value: "maya".to_string(),
                parsed_value: serde_json::json!("maya"),
                resolved_entity: None,
            });
        } else if lower.contains("bank") || lower.contains("transfer") {
            tokens.insert("method".to_string(), ProvidedToken {
                raw_value: "bank_transfer".to_string(),
                parsed_value: serde_json::json!("bank_transfer"),
                resolved_entity: None,
            });
        } else if lower.contains("check") || lower.contains("cheque") {
            tokens.insert("method".to_string(), ProvidedToken {
                raw_value: "check".to_string(),
                parsed_value: serde_json::json!("check"),
                resolved_entity: None,
            });
        }
        
        // Extract date if mentioned
        if lower.contains("today") {
            tokens.insert("date".to_string(), ProvidedToken {
                raw_value: "today".to_string(),
                parsed_value: serde_json::json!(chrono::Utc::now().date_naive().to_string()),
                resolved_entity: None,
            });
        } else if lower.contains("tomorrow") {
            let tomorrow = chrono::Utc::now().date_naive() + chrono::Duration::days(1);
            tokens.insert("date".to_string(), ProvidedToken {
                raw_value: "tomorrow".to_string(),
                parsed_value: serde_json::json!(tomorrow.to_string()),
                resolved_entity: None,
            });
        }
        
        Ok(tokens)
    }
    
    /// Extract client name from command
    fn extract_client_name(&self, command: &str) -> Option<String> {
        let lower = command.to_lowercase();
        
        // Look for patterns like "for Juan", "to Maria", "@client Pedro"
        for preposition in &["for", "to", "from", "@client", "@customer"] {
            if let Some(idx) = lower.find(preposition) {
                let after = &command[idx + preposition.len()..].trim();
                let words: Vec<&str> = after.split_whitespace().collect();
                if !words.is_empty() {
                    let first_word = words[0];
                    // Check if it's not a number
                    if !first_word.chars().all(|c| c.is_numeric() || c == '.' || c == ',') {
                        // Try to get both first and last name
                        if words.len() > 1 && !words[1].chars().all(|c| c.is_numeric() || c == '.' || c == ',') {
                            return Some(format!("{} {}", words[0], words[1]));
                        }
                        return Some(first_word.to_string());
                    }
                }
            }
        }
        
        None
    }
    
    /// Extract amount from command
    fn extract_amount(&self, command: &str) -> Option<f64> {
        let words: Vec<&str> = command.split_whitespace().collect();
        
        for word in words {
            let cleaned = word
                .replace(",", "")
                .replace("pesos", "")
                .replace("peso", "")
                .replace("php", "")
                .replace("₱", "");
            
            // Handle "k" suffix for thousands
            if cleaned.ends_with("k") || cleaned.ends_with("K") {
                if let Ok(num) = cleaned[..cleaned.len()-1].parse::<f64>() {
                    return Some(num * 1000.0);
                }
            }
            
            // Try to parse as regular number
            if let Ok(num) = cleaned.parse::<f64>() {
                if num >= 0.01 {
                    return Some(num);
                }
            }
        }
        
        None
    }
    
    /// Register a custom schema
    pub fn register_schema(&mut self, plugin: String, action: String, schema: TokenSchema) {
        self.schemas.insert((plugin, action), schema);
    }
}