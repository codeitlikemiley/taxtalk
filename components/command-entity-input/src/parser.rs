use crate::types::*;
use regex::Regex;

pub struct CommandParser {
    commands: Vec<Command>,
    entities: Vec<Entity>,
}

impl CommandParser {
    pub fn new() -> Self {
        Self {
            commands: Self::default_commands(),
            entities: Self::default_entities(),
        }
    }
    
    pub fn with_commands(mut self, commands: Vec<Command>) -> Self {
        self.commands = commands;
        self
    }
    
    pub fn with_entities(mut self, entities: Vec<Entity>) -> Self {
        self.entities = entities;
        self
    }
    
    pub fn parse(&self, input: &str) -> ParseResult {
        let input_lower = input.to_lowercase();
        let tokens = self.tokenize(&input_lower);
        
        let mut result = ParseResult::default();
        
        // Check for @ symbol indicating entity request
        if let Some(at_pos) = input.rfind('@') {
            result.requesting_entity = true;
            result.at_position = Some(at_pos);
            
            // Check if there's text after @
            let after_at = &input[at_pos + 1..];
            if !after_at.is_empty() {
                // Try to match entity
                for entity in &self.entities {
                    if entity.name.to_lowercase().starts_with(&after_at.to_lowercase()) ||
                       entity.plural.to_lowercase().starts_with(&after_at.to_lowercase()) ||
                       entity.aliases.iter().any(|a| a.to_lowercase().starts_with(&after_at.to_lowercase())) {
                        result.entity = Some(entity.name.clone());
                        break;
                    }
                }
            }
        }
        
        // Find command
        for token in &tokens {
            if token.token_type == TokenType::Command {
                for cmd in &self.commands {
                    if cmd.name.to_lowercase() == token.value.to_lowercase() ||
                       cmd.pattern.contains(&token.value.to_lowercase()) {
                        result.command = Some(cmd.name.clone());
                        result.mode = InputMode::Command;
                        break;
                    }
                }
            }
        }
        
        // Extract parameters
        for token in &tokens {
            if token.token_type == TokenType::Parameter {
                result.parameters.push(token.value.clone());
            }
        }
        
        result
    }
    
    fn tokenize(&self, input: &str) -> Vec<CommandToken> {
        let mut tokens = Vec::new();
        let words: Vec<&str> = input.split_whitespace().collect();
        let mut position = 0;
        
        for word in words {
            let token_type = if self.is_command(word) {
                TokenType::Command
            } else if word.starts_with('@') {
                TokenType::Entity
            } else if word == "to" || word == "from" || word == "for" || word == "with" {
                TokenType::Operator
            } else {
                TokenType::Parameter
            };
            
            tokens.push(CommandToken {
                token_type,
                value: word.to_string(),
                position,
            });
            
            position += word.len() + 1;
        }
        
        tokens
    }
    
    fn is_command(&self, word: &str) -> bool {
        let word_lower = word.to_lowercase();
        self.commands.iter().any(|cmd| {
            cmd.name.to_lowercase() == word_lower ||
            cmd.pattern.contains(&word_lower)
        })
    }
    
    fn default_commands() -> Vec<Command> {
        vec![
            Command {
                name: "create".to_string(),
                description: "Create a new record".to_string(),
                supports_entities: vec!["invoice".to_string(), "client".to_string(), "product".to_string()],
                pattern: "create|new|add".to_string(),
            },
            Command {
                name: "update".to_string(),
                description: "Update existing record".to_string(),
                supports_entities: vec!["invoice".to_string(), "client".to_string(), "product".to_string()],
                pattern: "update|edit|modify|change".to_string(),
            },
            Command {
                name: "delete".to_string(),
                description: "Delete a record".to_string(),
                supports_entities: vec!["invoice".to_string(), "client".to_string(), "product".to_string()],
                pattern: "delete|remove|destroy".to_string(),
            },
            Command {
                name: "list".to_string(),
                description: "List records".to_string(),
                supports_entities: vec!["invoice".to_string(), "client".to_string(), "product".to_string(), "payment".to_string()],
                pattern: "list|show|display".to_string(),
            },
            Command {
                name: "search".to_string(),
                description: "Search for records".to_string(),
                supports_entities: vec![],
                pattern: "search|find|lookup".to_string(),
            },
            Command {
                name: "generate".to_string(),
                description: "Generate reports or documents".to_string(),
                supports_entities: vec!["report".to_string(), "statement".to_string()],
                pattern: "generate|make|produce".to_string(),
            },
        ]
    }
    
    fn default_entities() -> Vec<Entity> {
        vec![
            Entity {
                name: "invoice".to_string(),
                plural: "invoices".to_string(),
                description: "Sales invoice or billing document".to_string(),
                aliases: vec!["inv".to_string(), "bill".to_string()],
            },
            Entity {
                name: "client".to_string(),
                plural: "clients".to_string(),
                description: "Customer or client record".to_string(),
                aliases: vec!["customer".to_string(), "cust".to_string()],
            },
            Entity {
                name: "product".to_string(),
                plural: "products".to_string(),
                description: "Product or service".to_string(),
                aliases: vec!["item".to_string(), "service".to_string()],
            },
            Entity {
                name: "payment".to_string(),
                plural: "payments".to_string(),
                description: "Payment transaction".to_string(),
                aliases: vec!["pay".to_string(), "transaction".to_string()],
            },
            Entity {
                name: "report".to_string(),
                plural: "reports".to_string(),
                description: "Business report or summary".to_string(),
                aliases: vec!["summary".to_string()],
            },
            Entity {
                name: "expense".to_string(),
                plural: "expenses".to_string(),
                description: "Business expense".to_string(),
                aliases: vec!["cost".to_string()],
            },
        ]
    }
}

pub fn validate_command(parse_result: &ParseResult) -> Result<(), String> {
    if parse_result.mode == InputMode::Command {
        if parse_result.command.is_none() {
            return Err("No valid command found".to_string());
        }
        
        // Some commands require entities
        if let Some(ref cmd) = parse_result.command {
            if (cmd == "create" || cmd == "update" || cmd == "delete") && parse_result.entity.is_none() {
                return Err(format!("Command '{}' requires an entity", cmd));
            }
        }
    }
    
    Ok(())
}