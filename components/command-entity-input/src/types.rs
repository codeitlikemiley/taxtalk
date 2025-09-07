use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputMode {
    Command,
    AI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub supports_entities: Vec<String>,
    pub pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub plural: String,
    pub description: String,
    pub aliases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    pub command: Option<String>,
    pub entity: Option<String>,
    pub parameters: Vec<String>,
    pub requesting_entity: bool,
    pub at_position: Option<usize>,
    pub mode: InputMode,
}

impl Default for ParseResult {
    fn default() -> Self {
        Self {
            command: None,
            entity: None,
            parameters: vec![],
            requesting_entity: false,
            at_position: None,
            mode: InputMode::AI,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandToken {
    pub token_type: TokenType,
    pub value: String,
    pub position: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
    Command,
    Entity,
    Parameter,
    Operator,
    Unknown,
}