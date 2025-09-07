use serde::{Serialize, Deserialize};

/// Simplified Token enum for the new architecture
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Token {
    Text(String),
    Verb(String),
    Amount { 
        value: f64, 
        currency: String 
    },
    EntityRef { 
        entity_type: String, 
        identifier: String 
    },
    Date(String),
    Identifier(String),
    TaxType { 
        tax_type: String, 
        rate: Option<f64> 
    },
    PaymentMethod(String),
}

impl Token {
    pub fn text(s: impl Into<String>) -> Self {
        Token::Text(s.into())
    }
    
    pub fn verb(s: impl Into<String>) -> Self {
        Token::Verb(s.into())
    }
    
    pub fn amount(value: f64, currency: impl Into<String>) -> Self {
        Token::Amount {
            value,
            currency: currency.into(),
        }
    }
    
    pub fn entity_ref(entity_type: impl Into<String>, identifier: impl Into<String>) -> Self {
        Token::EntityRef {
            entity_type: entity_type.into(),
            identifier: identifier.into(),
        }
    }
}