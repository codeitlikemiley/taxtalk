use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Defines the schema for tokens required by a plugin action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSchema {
    pub plugin: String,
    pub action: String,
    pub required_tokens: Vec<TokenDefinition>,
    pub optional_tokens: Vec<TokenDefinition>,
}

/// Definition of a single token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenDefinition {
    pub name: String,
    pub token_type: TokenType,
    pub prompt: String,
    pub input_type: InputType,
    pub validation: Option<TokenValidation>,
    pub default: Option<serde_json::Value>,
    pub depends_on: Option<Vec<String>>,
}

/// Type of token value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    String,
    Number,
    Boolean,
    Date,
    EntityReference(String), // Entity type like "client", "invoice"
    Array(Box<TokenType>),
    Object(HashMap<String, TokenType>),
}

/// How the token should be collected from user
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputType {
    Chat,           // Simple text input in chat
    Form,           // Complex form with multiple fields
    Autocomplete,   // Entity search and selection
    Select,         // Dropdown selection
    YesNo,          // Boolean quick buttons
    DatePicker,     // Calendar widget
    FilePicker,     // File upload
}

/// Validation rules for token values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenValidation {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub pattern: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub required_fields: Option<Vec<String>>,
    pub allowed_values: Option<Vec<String>>,
}

impl TokenSchema {
    /// Create schema for invoice creation
    pub fn invoice_create() -> Self {
        TokenSchema {
            plugin: "invoice".to_string(),
            action: "create".to_string(),
            required_tokens: vec![
                TokenDefinition {
                    name: "client".to_string(),
                    token_type: TokenType::EntityReference("client".to_string()),
                    prompt: "For which client is this invoice?".to_string(),
                    input_type: InputType::Autocomplete,
                    validation: None,
                    default: None,
                    depends_on: None,
                },
                TokenDefinition {
                    name: "amount".to_string(),
                    token_type: TokenType::Number,
                    prompt: "What is the invoice amount?".to_string(),
                    input_type: InputType::Chat,
                    validation: Some(TokenValidation {
                        min: Some(0.01),
                        max: Some(999999999.99),
                        pattern: None,
                        min_length: None,
                        max_length: None,
                        required_fields: None,
                        allowed_values: None,
                    }),
                    default: None,
                    depends_on: None,
                },
                TokenDefinition {
                    name: "items".to_string(),
                    token_type: TokenType::Array(Box::new(TokenType::Object({
                        let mut fields = HashMap::new();
                        fields.insert("description".to_string(), TokenType::String);
                        fields.insert("quantity".to_string(), TokenType::Number);
                        fields.insert("unit_price".to_string(), TokenType::Number);
                        fields
                    }))),
                    prompt: "What items or services are you invoicing for?".to_string(),
                    input_type: InputType::Form,
                    validation: Some(TokenValidation {
                        min: Some(1.0), // At least 1 item
                        max: None,
                        pattern: None,
                        min_length: None,
                        max_length: None,
                        required_fields: Some(vec![
                            "description".to_string(),
                            "quantity".to_string(),
                            "unit_price".to_string(),
                        ]),
                        allowed_values: None,
                    }),
                    default: None,
                    depends_on: None,
                },
            ],
            optional_tokens: vec![
                TokenDefinition {
                    name: "due_date".to_string(),
                    token_type: TokenType::Date,
                    prompt: "When is payment due? (default: 30 days)".to_string(),
                    input_type: InputType::DatePicker,
                    validation: None,
                    default: Some(serde_json::json!("+30days")),
                    depends_on: None,
                },
                TokenDefinition {
                    name: "notes".to_string(),
                    token_type: TokenType::String,
                    prompt: "Any additional notes?".to_string(),
                    input_type: InputType::Chat,
                    validation: Some(TokenValidation {
                        min: None,
                        max: None,
                        pattern: None,
                        min_length: None,
                        max_length: Some(500),
                        required_fields: None,
                        allowed_values: None,
                    }),
                    default: None,
                    depends_on: None,
                },
                TokenDefinition {
                    name: "tax_exempt".to_string(),
                    token_type: TokenType::Boolean,
                    prompt: "Is this tax exempt?".to_string(),
                    input_type: InputType::YesNo,
                    validation: None,
                    default: Some(serde_json::json!(false)),
                    depends_on: None,
                },
            ],
        }
    }

    /// Create schema for payment recording
    pub fn payment_create() -> Self {
        TokenSchema {
            plugin: "payment".to_string(),
            action: "create".to_string(),
            required_tokens: vec![
                TokenDefinition {
                    name: "client".to_string(),
                    token_type: TokenType::EntityReference("client".to_string()),
                    prompt: "From which client is this payment?".to_string(),
                    input_type: InputType::Autocomplete,
                    validation: None,
                    default: None,
                    depends_on: None,
                },
                TokenDefinition {
                    name: "amount".to_string(),
                    token_type: TokenType::Number,
                    prompt: "How much was paid?".to_string(),
                    input_type: InputType::Chat,
                    validation: Some(TokenValidation {
                        min: Some(0.01),
                        max: Some(999999999.99),
                        pattern: None,
                        min_length: None,
                        max_length: None,
                        required_fields: None,
                        allowed_values: None,
                    }),
                    default: None,
                    depends_on: None,
                },
                TokenDefinition {
                    name: "method".to_string(),
                    token_type: TokenType::String,
                    prompt: "How was the payment made?".to_string(),
                    input_type: InputType::Select,
                    validation: Some(TokenValidation {
                        min: None,
                        max: None,
                        pattern: None,
                        min_length: None,
                        max_length: None,
                        required_fields: None,
                        allowed_values: Some(vec![
                            "cash".to_string(),
                            "check".to_string(),
                            "bank_transfer".to_string(),
                            "gcash".to_string(),
                            "maya".to_string(),
                            "credit_card".to_string(),
                        ]),
                    }),
                    default: Some(serde_json::json!("cash")),
                    depends_on: None,
                },
            ],
            optional_tokens: vec![
                TokenDefinition {
                    name: "invoice_id".to_string(),
                    token_type: TokenType::EntityReference("invoice".to_string()),
                    prompt: "Is this payment for a specific invoice?".to_string(),
                    input_type: InputType::Autocomplete,
                    validation: None,
                    default: None,
                    depends_on: Some(vec!["client".to_string()]), // Only show invoices for selected client
                },
                TokenDefinition {
                    name: "reference_number".to_string(),
                    token_type: TokenType::String,
                    prompt: "Reference number (check #, transaction ID)?".to_string(),
                    input_type: InputType::Chat,
                    validation: None,
                    default: None,
                    depends_on: Some(vec!["method".to_string()]), // Only for non-cash
                },
            ],
        }
    }

    /// Get token definition by name
    pub fn get_token(&self, name: &str) -> Option<&TokenDefinition> {
        self.required_tokens
            .iter()
            .find(|t| t.name == name)
            .or_else(|| self.optional_tokens.iter().find(|t| t.name == name))
    }

    /// Check if a token is required
    pub fn is_required(&self, name: &str) -> bool {
        self.required_tokens.iter().any(|t| t.name == name)
    }

    /// Get all token names
    pub fn all_token_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for token in &self.required_tokens {
            names.push(token.name.clone());
        }
        for token in &self.optional_tokens {
            names.push(token.name.clone());
        }
        names
    }
}