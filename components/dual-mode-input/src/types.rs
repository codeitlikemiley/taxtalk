use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputMode {
    Tokenization,
    AI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandInfo {
    pub command: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub plugin_id: String,
    pub required_tokens: Vec<TokenInfo>,
    pub optional_tokens: Vec<TokenInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub token_type: String,
    pub description: String,
    pub multiple: bool,
    pub entity_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub entity_type: String,
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Clone)]
pub struct CommandToken {
    pub command: String,
    pub display: String,
}

#[derive(Debug, Clone)]
pub struct EntityToken {
    pub entity_type: String,
    pub entity: Entity,
}

#[derive(Debug, Clone)]
pub struct TokenizedInput {
    pub command: Option<CommandToken>,
    pub entities: Vec<EntityToken>,
    pub parameters: Vec<String>,
    pub raw_text: String,
}

impl TokenizedInput {
    pub fn new() -> Self {
        Self {
            command: None,
            entities: vec![],
            parameters: vec![],
            raw_text: String::new(),
        }
    }
    
    pub fn to_command_string(&self) -> String {
        let mut parts = Vec::new();
        
        if let Some(ref cmd) = self.command {
            parts.push(cmd.command.clone());
        }
        
        for entity in &self.entities {
            parts.push(format!("@{} {}", entity.entity_type, entity.entity.name));
        }
        
        for param in &self.parameters {
            parts.push(param.clone());
        }
        
        parts.join(" ")
    }
}

// Default commands for demo
impl CommandInfo {
    pub fn default_commands() -> Vec<Self> {
        vec![
            CommandInfo {
                command: "create".to_string(),
                aliases: vec!["new".to_string(), "add".to_string()],
                description: "Create a new record".to_string(),
                plugin_id: "core".to_string(),
                required_tokens: vec![
                    TokenInfo {
                        token_type: "entity_ref".to_string(),
                        description: "Entity to create".to_string(),
                        multiple: false,
                        entity_type: None,
                    },
                ],
                optional_tokens: vec![],
            },
            CommandInfo {
                command: "invoice".to_string(),
                aliases: vec!["bill".to_string(), "charge".to_string()],
                description: "Create or manage invoices".to_string(),
                plugin_id: "invoice".to_string(),
                required_tokens: vec![
                    TokenInfo {
                        token_type: "entity_ref".to_string(),
                        description: "Client to invoice".to_string(),
                        multiple: false,
                        entity_type: Some("client".to_string()),
                    },
                ],
                optional_tokens: vec![
                    TokenInfo {
                        token_type: "entity_ref".to_string(),
                        description: "Products or services".to_string(),
                        multiple: true,
                        entity_type: Some("product".to_string()),
                    },
                ],
            },
            CommandInfo {
                command: "pay".to_string(),
                aliases: vec!["payment".to_string(), "paid".to_string()],
                description: "Record a payment".to_string(),
                plugin_id: "payment".to_string(),
                required_tokens: vec![
                    TokenInfo {
                        token_type: "entity_ref".to_string(),
                        description: "Payer (client or supplier)".to_string(),
                        multiple: false,
                        entity_type: None,
                    },
                ],
                optional_tokens: vec![
                    TokenInfo {
                        token_type: "amount".to_string(),
                        description: "Payment amount".to_string(),
                        multiple: false,
                        entity_type: None,
                    },
                ],
            },
            CommandInfo {
                command: "report".to_string(),
                aliases: vec!["generate".to_string(), "summary".to_string()],
                description: "Generate reports".to_string(),
                plugin_id: "reporting".to_string(),
                required_tokens: vec![],
                optional_tokens: vec![
                    TokenInfo {
                        token_type: "date_range".to_string(),
                        description: "Report period".to_string(),
                        multiple: false,
                        entity_type: None,
                    },
                ],
            },
        ]
    }
}