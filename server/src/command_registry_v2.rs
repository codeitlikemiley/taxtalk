use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

/// Generic command that can be applied to different entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub supports_entities: Vec<String>,
}

/// Entity that commands can operate on
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub plural: String,
    pub description: String,
    pub aliases: Vec<String>,
}

/// Token requirement for a command-entity combination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRequirement {
    pub token_type: String,
    pub description: String,
    pub multiple: bool,
}

/// Requirements for a specific command-entity combination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandEntityRequirements {
    pub required: Vec<TokenRequirement>,
    pub optional: Vec<TokenRequirement>,
}

/// Registry for commands and entities
pub struct CommandRegistryV2 {
    /// Generic commands (create, read, update, delete, etc.)
    commands: HashMap<String, Command>,
    
    /// Entities that commands can operate on
    entities: HashMap<String, Entity>,
    
    /// Requirements for command-entity combinations
    requirements: HashMap<(String, String), CommandEntityRequirements>,
    
    /// Valid command words (for quick lookup)
    command_words: HashSet<String>,
    
    /// Valid entity words (including aliases)
    entity_words: HashMap<String, String>, // word -> canonical entity name
}

impl CommandRegistryV2 {
    pub fn new() -> Self {
        // Initialize with basic CRUD commands
        let mut registry = Self {
            commands: HashMap::new(),
            entities: HashMap::new(),
            requirements: HashMap::new(),
            command_words: HashSet::new(),
            entity_words: HashMap::new(),
        };
        
        // Add basic CRUD commands
        registry.add_command(Command {
            name: "create".to_string(),
            description: "Create a new record".to_string(),
            supports_entities: vec![],
        });
        
        registry.add_command(Command {
            name: "read".to_string(),
            description: "Read/view records".to_string(),
            supports_entities: vec![],
        });
        
        registry.add_command(Command {
            name: "update".to_string(),
            description: "Update existing records".to_string(),
            supports_entities: vec![],
        });
        
        registry.add_command(Command {
            name: "delete".to_string(),
            description: "Delete records".to_string(),
            supports_entities: vec![],
        });
        
        registry.add_command(Command {
            name: "list".to_string(),
            description: "List multiple records".to_string(),
            supports_entities: vec![],
        });
        
        registry.add_command(Command {
            name: "search".to_string(),
            description: "Search for records".to_string(),
            supports_entities: vec![],
        });
        
        registry
    }
    
    /// Add a command to the registry
    pub fn add_command(&mut self, command: Command) {
        self.command_words.insert(command.name.clone());
        self.commands.insert(command.name.clone(), command);
    }
    
    /// Add an entity to the registry
    pub fn add_entity(&mut self, entity: Entity) {
        // Add the main entity name
        self.entity_words.insert(entity.name.clone(), entity.name.clone());
        self.entity_words.insert(entity.plural.clone(), entity.name.clone());
        
        // Add aliases
        for alias in &entity.aliases {
            self.entity_words.insert(alias.clone(), entity.name.clone());
        }
        
        self.entities.insert(entity.name.clone(), entity);
    }
    
    /// Add requirements for a command-entity combination
    pub fn add_requirements(&mut self, command: String, entity: String, requirements: CommandEntityRequirements) {
        self.requirements.insert((command.clone(), entity.clone()), requirements);
        
        // Update the command to know it supports this entity
        if let Some(cmd) = self.commands.get_mut(&command) {
            if !cmd.supports_entities.contains(&entity) {
                cmd.supports_entities.push(entity);
            }
        }
    }
    
    /// Parse text to detect command and entity
    pub fn parse_input(&self, text: &str) -> ParseResult {
        let words: Vec<&str> = text.split_whitespace().collect();
        
        let mut detected_command = None;
        let mut detected_entity = None;
        let mut at_symbol_position = None;
        
        // Look for command words (exact match at word boundaries)
        for (i, word) in words.iter().enumerate() {
            let word_lower = word.to_lowercase();
            
            // Check for commands
            if self.command_words.contains(&word_lower) {
                detected_command = Some(word_lower.clone());
            }
            
            // Check for entities
            if let Some(canonical) = self.entity_words.get(&word_lower) {
                detected_entity = Some(canonical.clone());
            }
            
            // Check for @ symbol
            if word.starts_with('@') {
                at_symbol_position = Some(i);
            }
        }
        
        // Check if text ends with @ (user is about to type entity)
        let requesting_entity = text.trim_end().ends_with('@');
        
        ParseResult {
            command: detected_command,
            entity: detected_entity,
            requesting_entity,
            at_position: at_symbol_position,
        }
    }
    
    /// Get available entities for a command
    pub fn get_available_entities(&self, command: &str) -> Vec<String> {
        if let Some(cmd) = self.commands.get(command) {
            if cmd.supports_entities.is_empty() {
                // If no specific entities, return all
                self.entities.keys().cloned().collect()
            } else {
                cmd.supports_entities.clone()
            }
        } else {
            vec![]
        }
    }
    
    /// Get available tokens for a command-entity combination
    pub fn get_available_tokens(&self, command: &str, entity: &str) -> Vec<TokenInfo> {
        let mut tokens = Vec::new();
        
        if let Some(requirements) = self.requirements.get(&(command.to_string(), entity.to_string())) {
            for req in &requirements.required {
                tokens.push(TokenInfo {
                    token_type: req.token_type.clone(),
                    description: req.description.clone(),
                    multiple: req.multiple,
                    required: true,
                });
            }
            
            for req in &requirements.optional {
                tokens.push(TokenInfo {
                    token_type: req.token_type.clone(),
                    description: req.description.clone(),
                    multiple: req.multiple,
                    required: false,
                });
            }
        }
        
        tokens
    }
    
    /// Get all commands
    pub fn get_all_commands(&self) -> Vec<Command> {
        self.commands.values().cloned().collect()
    }
    
    /// Get all entities
    pub fn get_all_entities(&self) -> Vec<Entity> {
        self.entities.values().cloned().collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    pub command: Option<String>,
    pub entity: Option<String>,
    pub requesting_entity: bool,
    pub at_position: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub token_type: String,
    pub description: String,
    pub multiple: bool,
    pub required: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_input() {
        let mut registry = CommandRegistryV2::new();
        
        registry.add_entity(Entity {
            name: "invoice".to_string(),
            plural: "invoices".to_string(),
            description: "Sales invoice".to_string(),
            aliases: vec!["bill".to_string()],
        });
        
        // Test command detection
        let result = registry.parse_input("create invoice");
        assert_eq!(result.command, Some("create".to_string()));
        assert_eq!(result.entity, Some("invoice".to_string()));
        
        // Test alias detection
        let result = registry.parse_input("create bill");
        assert_eq!(result.command, Some("create".to_string()));
        assert_eq!(result.entity, Some("invoice".to_string()));
        
        // Test @ detection
        let result = registry.parse_input("create invoice @");
        assert!(result.requesting_entity);
        
        // Test partial input
        let result = registry.parse_input("create");
        assert_eq!(result.command, Some("create".to_string()));
        assert_eq!(result.entity, None);
    }
}