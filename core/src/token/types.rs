use std::sync::Arc;
use std::collections::HashSet;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::token::traits::{
    Token, TokenPattern, ValidationError, TokenPosition, 
    EntityResolver, Tense, AmountFormat, TokenMetadata
};
use crate::token::base::BaseTokenFields;
use crate::impl_token_base;

/// Entity Reference Token
#[derive(Debug, Clone)]
pub struct EntityRef {
    pub base: BaseTokenFields,
    pub plugin: String,
    pub entity_type: String,
    pub entity_id: String,
    pub display_name: Option<String>,
    pub resolver: Option<Arc<dyn EntityResolver>>,
}

impl Token for EntityRef {
    fn token_type(&self) -> &str {
        "entity_ref"
    }
    
    impl_token_base!(EntityRef);
    
    fn to_json(&self) -> Value {
        json!({
            "type": "entity_ref",
            "uuid": self.uuid().to_string(),
            "plugin": self.plugin,
            "entity_type": self.entity_type,
            "entity_id": self.entity_id,
            "display_name": self.display_name,
            "raw_text": self.raw_text(),
        })
    }
    
    fn matches(&self, pattern: &dyn TokenPattern) -> bool {
        pattern.matches(self)
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        if self.entity_id.is_empty() {
            return Err(ValidationError::InvalidToken {
                reason: "Entity ID cannot be empty".to_string(),
            });
        }
        Ok(())
    }
    
    fn can_compose_with(&self, other: &dyn Token) -> bool {
        // Entities can compose with actions, amounts, etc.
        matches!(other.token_type(), "action" | "amount" | "quantity")
    }
    
    fn display_text(&self) -> String {
        self.display_name.clone()
            .unwrap_or_else(|| format!("@{} {}", self.entity_type, self.entity_id))
    }
}

/// Amount Token
#[derive(Debug, Clone)]
pub struct Amount {
    pub base: BaseTokenFields,
    pub value: Decimal,
    pub currency: Option<String>, // Currency code
    pub format: AmountFormat,
}

impl Token for Amount {
    fn token_type(&self) -> &str {
        "amount"
    }
    
    impl_token_base!(Amount);
    
    fn to_json(&self) -> Value {
        json!({
            "type": "amount",
            "uuid": self.uuid().to_string(),
            "value": self.value.to_string(),
            "currency": self.currency,
            "format": self.format,
            "raw_text": self.raw_text(),
        })
    }
    
    fn matches(&self, pattern: &dyn TokenPattern) -> bool {
        pattern.matches(self)
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        if self.value < Decimal::ZERO {
            return Err(ValidationError::InvalidToken {
                reason: "Amount cannot be negative".to_string(),
            });
        }
        Ok(())
    }
    
    fn can_compose_with(&self, other: &dyn Token) -> bool {
        // Amounts can compose with taxes, discounts, etc.
        matches!(other.token_type(), "tax" | "discount" | "entity_ref" | "tax:vat:ph")
    }

    fn display_text(&self) -> String {
        match &self.currency {
            Some(cur) => format!("{} {}", cur, self.value),
            None => self.value.to_string(),
        }
    }
}

/// Action Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionToken {
    pub base: BaseTokenFields,
    pub verb: String,
    pub tense: Tense,
    pub variations: HashSet<String>,
}

impl Token for ActionToken {
    fn token_type(&self) -> &str {
        "action"
    }
    
    impl_token_base!(ActionToken);
    
    fn to_json(&self) -> Value {
        json!({
            "type": "action",
            "uuid": self.uuid().to_string(),
            "verb": self.verb,
            "tense": self.tense,
            "variations": self.variations.iter().cloned().collect::<Vec<_>>(),
            "raw_text": self.raw_text(),
        })
    }
    
    fn matches(&self, pattern: &dyn TokenPattern) -> bool {
        pattern.matches(self)
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        if self.verb.is_empty() {
            return Err(ValidationError::InvalidToken {
                reason: "Action verb cannot be empty".to_string(),
            });
        }
        Ok(())
    }
    
    fn can_compose_with(&self, _other: &dyn Token) -> bool {
        // Actions compose with almost everything
        true
    }

    fn display_text(&self) -> String {
        self.verb.clone()
    }
}

/// Quantity Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quantity {
    pub base: BaseTokenFields,
    pub value: Decimal,
    pub unit: Option<String>,
}

impl Token for Quantity {
    fn token_type(&self) -> &str {
        "quantity"
    }
    
    impl_token_base!(Quantity);
    
    fn to_json(&self) -> Value {
        json!({
            "type": "quantity",
            "uuid": self.uuid().to_string(),
            "value": self.value.to_string(),
            "unit": self.unit,
            "raw_text": self.raw_text(),
        })
    }
    
    fn matches(&self, pattern: &dyn TokenPattern) -> bool {
        pattern.matches(self)
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        if self.value <= Decimal::ZERO {
            return Err(ValidationError::InvalidToken {
                reason: "Quantity must be positive".to_string(),
            });
        }
        Ok(())
    }
    
    fn can_compose_with(&self, other: &dyn Token) -> bool {
        matches!(other.token_type(), "product" | "entity_ref" | "amount")
    }

    fn display_text(&self) -> String {
        match &self.unit {
            Some(unit) => format!("{} {}", self.value, unit),
            None => self.value.to_string(),
        }
    }
}

/// Date Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateToken {
    pub base: BaseTokenFields,
    pub date: chrono::NaiveDate,
    pub format: String,
}

impl Token for DateToken {
    fn token_type(&self) -> &str {
        "date"
    }
    
    impl_token_base!(DateToken);
    
    fn to_json(&self) -> Value {
        json!({
            "type": "date",
            "uuid": self.uuid().to_string(),
            "date": self.date.to_string(),
            "format": self.format,
            "raw_text": self.raw_text(),
        })
    }
    
    fn matches(&self, pattern: &dyn TokenPattern) -> bool {
        pattern.matches(self)
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
    
    fn can_compose_with(&self, other: &dyn Token) -> bool {
        matches!(other.token_type(), "action" | "entity_ref" | "amount")
    }

    fn display_text(&self) -> String {
        self.date.format(&self.format).to_string()
    }
}

/// Product Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductToken {
    pub base: BaseTokenFields,
    pub name: String,
    pub sku: Option<String>,
    pub category: Option<String>,
}

impl Token for ProductToken {
    fn token_type(&self) -> &str {
        "product"
    }
    
    impl_token_base!(ProductToken);
    
    fn to_json(&self) -> Value {
        json!({
            "type": "product",
            "uuid": self.uuid().to_string(),
            "name": self.name,
            "sku": self.sku,
            "category": self.category,
            "raw_text": self.raw_text(),
        })
    }
    
    fn matches(&self, pattern: &dyn TokenPattern) -> bool {
        pattern.matches(self)
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        if self.name.is_empty() {
            return Err(ValidationError::InvalidToken {
                reason: "Product name cannot be empty".to_string(),
            });
        }
        Ok(())
    }
    
    fn can_compose_with(&self, other: &dyn Token) -> bool {
        matches!(other.token_type(), "quantity" | "amount" | "action")
    }

    fn display_text(&self) -> String {
        self.name.clone()
    }
}