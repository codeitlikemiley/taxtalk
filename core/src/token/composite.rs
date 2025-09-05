use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::token::traits::{Token, TokenPattern, ValidationError, TokenMetadata, TokenPosition};
use crate::token::base::BaseTokenFields;
use crate::impl_token_base;

/// Composite token that contains multiple tokens
#[derive(Debug)]
pub struct CompositeToken {
    pub base: BaseTokenFields,
    pub tokens: Vec<Box<dyn Token>>,
    pub composition_type: CompositionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompositionType {
    Sequence,      // Tokens in order
    Group,         // Tokens as a group
    Alternative,   // One of the tokens
    Computed,      // Result of computation
    Transaction,   // Represents a complete transaction
    Command,       // Represents a command to execute
}

impl Token for CompositeToken {
    fn token_type(&self) -> &str {
        "composite"
    }
    
    impl_token_base!(CompositeToken);
    
    fn to_json(&self) -> Value {
        json!({
            "type": "composite",
            "uuid": self.uuid().to_string(),
            "composition_type": self.composition_type,
            "tokens": self.tokens.iter().map(|t| t.to_json()).collect::<Vec<_>>(),
            "raw_text": self.raw_text(),
        })
    }
    
    fn matches(&self, pattern: &dyn TokenPattern) -> bool {
        pattern.matches(self)
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        // Validate all contained tokens
        for token in &self.tokens {
            token.validate()?;
        }
        
        // Additional validation based on composition type
        match self.composition_type {
            CompositionType::Transaction => {
                // Ensure we have required tokens for a transaction
                let has_action = self.tokens.iter().any(|t| t.token_type() == "action");
                let has_entity = self.tokens.iter().any(|t| t.token_type() == "entity_ref");
                
                if !has_action {
                    return Err(ValidationError::MissingRequiredField {
                        field: "action".to_string(),
                    });
                }
                if !has_entity {
                    return Err(ValidationError::MissingRequiredField {
                        field: "entity".to_string(),
                    });
                }
            }
            CompositionType::Command => {
                // Ensure we have at least one action token
                let has_action = self.tokens.iter().any(|t| t.token_type() == "action");
                if !has_action {
                    return Err(ValidationError::MissingRequiredField {
                        field: "action".to_string(),
                    });
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    fn can_compose_with(&self, other: &dyn Token) -> bool {
        // Check if any contained token can compose
        self.tokens.iter().any(|t| t.can_compose_with(other))
    }

    fn display_text(&self) -> String {
        self.tokens
            .iter()
            .map(|t| t.display_text())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl CompositeToken {
    pub fn new(tokens: Vec<Box<dyn Token>>, composition_type: CompositionType) -> Self {
        let raw_text = tokens
            .iter()
            .map(|t| t.raw_text())
            .collect::<Vec<_>>()
            .join(" ");
        
        let position = if tokens.is_empty() {
            crate::token::traits::TokenPosition {
                start: 0,
                end: 0,
                line: 0,
                column: 0,
            }
        } else {
            let first = tokens.first().unwrap().position();
            let last = tokens.last().unwrap().position();
            crate::token::traits::TokenPosition {
                start: first.start,
                end: last.end,
                line: first.line,
                column: first.column,
            }
        };
        
        Self {
            base: BaseTokenFields::new(raw_text, position),
            tokens,
            composition_type,
        }
    }
    
    pub fn find_token_by_type(&self, token_type: &str) -> Option<&dyn Token> {
        self.tokens
            .iter()
            .find(|t| t.token_type() == token_type)
            .map(|t| t.as_ref())
    }
    
    pub fn find_all_tokens_by_type(&self, token_type: &str) -> Vec<&dyn Token> {
        self.tokens
            .iter()
            .filter(|t| t.token_type() == token_type)
            .map(|t| t.as_ref())
            .collect()
    }
    
    pub fn add_token(&mut self, token: Box<dyn Token>) {
        self.tokens.push(token);
        // Update raw_text
        self.base.raw_text = self.tokens
            .iter()
            .map(|t| t.raw_text())
            .collect::<Vec<_>>()
            .join(" ");
    }
    
    pub fn remove_token(&mut self, uuid: uuid::Uuid) -> Option<Box<dyn Token>> {
        let index = self.tokens.iter().position(|t| t.uuid() == uuid)?;
        Some(self.tokens.remove(index))
    }
}