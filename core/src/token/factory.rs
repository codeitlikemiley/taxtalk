use std::collections::HashMap;
use serde_json::Value;
use anyhow::{Result, anyhow};

use crate::token::traits::{Token, TokenPosition};
use crate::token::types::*;
use crate::token::philippine::*;
use crate::token::base::BaseTokenFields;

/// Trait for token creators
pub trait TokenCreator: Send + Sync {
    fn token_type(&self) -> &str;
    fn create(&self, input: &str, position: TokenPosition) -> Result<Box<dyn Token>>;
    fn create_from_json(&self, json: Value) -> Result<Box<dyn Token>>;
    fn patterns(&self) -> Vec<String>; // Regex patterns this creator handles
}

/// Token factory for creating tokens from patterns
pub struct TokenFactory {
    creators: HashMap<String, Box<dyn TokenCreator>>,
}

impl TokenFactory {
    pub fn new() -> Self {
        let mut factory = Self {
            creators: HashMap::new(),
        };
        
        // Register default creators
        factory.register_default_creators();
        factory
    }
    
    fn register_default_creators(&mut self) {
        // Register basic token creators
        self.register_creator(Box::new(AmountCreator));
        self.register_creator(Box::new(ActionCreator));
        self.register_creator(Box::new(QuantityCreator));
        self.register_creator(Box::new(EntityRefCreator));
        
        // Register Philippine-specific creators
        self.register_creator(Box::new(PhilippineVatCreator));
        self.register_creator(Box::new(PhilippineEwtCreator));
        self.register_creator(Box::new(SeniorPwdDiscountCreator));
    }
    
    pub fn register_creator(&mut self, creator: Box<dyn TokenCreator>) {
        self.creators.insert(creator.token_type().to_string(), creator);
    }
    
    pub fn create_token(
        &self, 
        token_type: &str, 
        input: &str, 
        position: TokenPosition
    ) -> Result<Box<dyn Token>> {
        self.creators
            .get(token_type)
            .ok_or_else(|| anyhow!("Unknown token type: {}", token_type))?
            .create(input, position)
    }
    
    pub fn create_from_json(&self, json: Value) -> Result<Box<dyn Token>> {
        let token_type = json.get("type")
            .and_then(|t| t.as_str())
            .ok_or_else(|| anyhow!("Missing type field in JSON"))?;
        
        self.creators
            .get(token_type)
            .ok_or_else(|| anyhow!("Unknown token type: {}", token_type))?
            .create_from_json(json)
    }
    
    pub fn get_patterns(&self) -> HashMap<String, Vec<String>> {
        self.creators
            .iter()
            .map(|(k, v)| (k.clone(), v.patterns()))
            .collect()
    }
}

// Concrete creators

struct AmountCreator;

impl TokenCreator for AmountCreator {
    fn token_type(&self) -> &str {
        "amount"
    }
    
    fn create(&self, input: &str, position: TokenPosition) -> Result<Box<dyn Token>> {
        // Parse amount from input
        let value = input.parse::<rust_decimal::Decimal>()
            .unwrap_or_else(|_| rust_decimal::Decimal::ZERO);
        
        Ok(Box::new(Amount {
            base: BaseTokenFields::new(input.to_string(), position),
            value,
            currency: Some("PHP".to_string()),
            format: crate::token::traits::AmountFormat::Currency,
        }))
    }
    
    fn create_from_json(&self, json: Value) -> Result<Box<dyn Token>> {
        let value = json.get("value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing value field"))?
            .parse::<rust_decimal::Decimal>()?;
        
        let raw_text = json.get("raw_text")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let position = TokenPosition {
            start: 0,
            end: raw_text.len(),
            line: 0,
            column: 0,
        };
        
        Ok(Box::new(Amount {
            base: BaseTokenFields::new(raw_text, position),
            value,
            currency: json.get("currency")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            format: crate::token::traits::AmountFormat::Currency,
        }))
    }
    
    fn patterns(&self) -> Vec<String> {
        vec![
            r"^\d+(\.\d{2})?$".to_string(),
            r"^PHP\s*\d+(\.\d{2})?$".to_string(),
            r"^₱\s*\d+(\.\d{2})?$".to_string(),
        ]
    }
}

struct ActionCreator;

impl TokenCreator for ActionCreator {
    fn token_type(&self) -> &str {
        "action"
    }
    
    fn create(&self, input: &str, position: TokenPosition) -> Result<Box<dyn Token>> {
        Ok(Box::new(ActionToken {
            base: BaseTokenFields::new(input.to_string(), position),
            verb: input.to_string(),
            tense: crate::token::traits::Tense::Present,
            variations: std::collections::HashSet::new(),
        }))
    }
    
    fn create_from_json(&self, json: Value) -> Result<Box<dyn Token>> {
        let verb = json.get("verb")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing verb field"))?
            .to_string();
        
        let raw_text = json.get("raw_text")
            .and_then(|v| v.as_str())
            .unwrap_or(&verb)
            .to_string();
        
        let position = TokenPosition {
            start: 0,
            end: raw_text.len(),
            line: 0,
            column: 0,
        };
        
        Ok(Box::new(ActionToken {
            base: BaseTokenFields::new(raw_text, position),
            verb,
            tense: crate::token::traits::Tense::Present,
            variations: std::collections::HashSet::new(),
        }))
    }
    
    fn patterns(&self) -> Vec<String> {
        vec![
            r"^(buy|bought|sell|sold|pay|paid|invoice|receive|received)$".to_string(),
        ]
    }
}

struct QuantityCreator;

impl TokenCreator for QuantityCreator {
    fn token_type(&self) -> &str {
        "quantity"
    }
    
    fn create(&self, input: &str, position: TokenPosition) -> Result<Box<dyn Token>> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let value = parts.get(0)
            .and_then(|s| s.parse::<rust_decimal::Decimal>().ok())
            .unwrap_or(rust_decimal::Decimal::ONE);
        let unit = parts.get(1).map(|s| s.to_string());
        
        Ok(Box::new(Quantity {
            base: BaseTokenFields::new(input.to_string(), position),
            value,
            unit,
        }))
    }
    
    fn create_from_json(&self, json: Value) -> Result<Box<dyn Token>> {
        let value = json.get("value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing value field"))?
            .parse::<rust_decimal::Decimal>()?;
        
        let raw_text = json.get("raw_text")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let position = TokenPosition {
            start: 0,
            end: raw_text.len(),
            line: 0,
            column: 0,
        };
        
        Ok(Box::new(Quantity {
            base: BaseTokenFields::new(raw_text, position),
            value,
            unit: json.get("unit")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        }))
    }
    
    fn patterns(&self) -> Vec<String> {
        vec![
            r"^\d+\s*(kg|g|L|ml|pcs|boxes|units)$".to_string(),
        ]
    }
}

struct EntityRefCreator;

impl TokenCreator for EntityRefCreator {
    fn token_type(&self) -> &str {
        "entity_ref"
    }
    
    fn create(&self, input: &str, position: TokenPosition) -> Result<Box<dyn Token>> {
        // Parse entity reference (e.g., "@client Juan" or "@supplier ABC Corp")
        let parts: Vec<&str> = input.trim_start_matches('@').split_whitespace().collect();
        let entity_type = parts.get(0).unwrap_or(&"unknown").to_string();
        let entity_id = parts.get(1..).map(|p| p.join(" ")).unwrap_or_default();
        
        Ok(Box::new(EntityRef {
            base: BaseTokenFields::new(input.to_string(), position),
            plugin: "core".to_string(),
            entity_type,
            entity_id: entity_id.clone(),
            display_name: Some(entity_id),
            resolver: None,
        }))
    }
    
    fn create_from_json(&self, json: Value) -> Result<Box<dyn Token>> {
        let entity_type = json.get("entity_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing entity_type field"))?
            .to_string();
        
        let entity_id = json.get("entity_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing entity_id field"))?
            .to_string();
        
        let raw_text = json.get("raw_text")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let position = TokenPosition {
            start: 0,
            end: raw_text.len(),
            line: 0,
            column: 0,
        };
        
        Ok(Box::new(EntityRef {
            base: BaseTokenFields::new(raw_text, position),
            plugin: json.get("plugin")
                .and_then(|v| v.as_str())
                .unwrap_or("core")
                .to_string(),
            entity_type,
            entity_id: entity_id.clone(),
            display_name: json.get("display_name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .or(Some(entity_id)),
            resolver: None,
        }))
    }
    
    fn patterns(&self) -> Vec<String> {
        vec![
            r"^@(client|customer|supplier|vendor|employee)\s+.+$".to_string(),
        ]
    }
}

struct PhilippineVatCreator;

impl TokenCreator for PhilippineVatCreator {
    fn token_type(&self) -> &str {
        "tax:vat:ph"
    }
    
    fn create(&self, input: &str, position: TokenPosition) -> Result<Box<dyn Token>> {
        let inclusive = input.contains("inclusive") || input.contains("with VAT");
        
        Ok(Box::new(PhilippineVatToken {
            base: BaseTokenFields::new(input.to_string(), position),
            rate: rust_decimal_macros::dec!(0.12),
            inclusive,
            applies_to: None,
        }))
    }
    
    fn create_from_json(&self, json: Value) -> Result<Box<dyn Token>> {
        let raw_text = json.get("raw_text")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let position = TokenPosition {
            start: 0,
            end: raw_text.len(),
            line: 0,
            column: 0,
        };
        
        Ok(Box::new(PhilippineVatToken {
            base: BaseTokenFields::new(raw_text, position),
            rate: rust_decimal_macros::dec!(0.12),
            inclusive: json.get("inclusive")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            applies_to: json.get("applies_to")
                .and_then(|v| v.as_str())
                .and_then(|s| uuid::Uuid::parse_str(s).ok()),
        }))
    }
    
    fn patterns(&self) -> Vec<String> {
        vec![
            r"(?i)(plus\s+)?VAT".to_string(),
            r"(?i)with\s+VAT".to_string(),
            r"(?i)inclusive\s+of\s+VAT".to_string(),
        ]
    }
}

struct PhilippineEwtCreator;

impl TokenCreator for PhilippineEwtCreator {
    fn token_type(&self) -> &str {
        "tax:ewt:ph"
    }
    
    fn create(&self, input: &str, position: TokenPosition) -> Result<Box<dyn Token>> {
        let (rate, ewt_type) = if input.contains("professional") || input.contains("service") {
            (rust_decimal_macros::dec!(0.10), EwtType::ProfessionalServices)
        } else if input.contains("goods") {
            (rust_decimal_macros::dec!(0.01), EwtType::Goods)
        } else if input.contains("rental") {
            (rust_decimal_macros::dec!(0.05), EwtType::Rental)
        } else {
            (rust_decimal_macros::dec!(0.02), EwtType::Other(rust_decimal_macros::dec!(0.02)))
        };
        
        Ok(Box::new(PhilippineEwtToken {
            base: BaseTokenFields::new(input.to_string(), position),
            rate,
            ewt_type,
            applies_to: None,
        }))
    }
    
    fn create_from_json(&self, json: Value) -> Result<Box<dyn Token>> {
        let raw_text = json.get("raw_text")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let position = TokenPosition {
            start: 0,
            end: raw_text.len(),
            line: 0,
            column: 0,
        };
        
        let rate = json.get("rate")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<rust_decimal::Decimal>().ok())
            .unwrap_or(rust_decimal_macros::dec!(0.02));
        
        Ok(Box::new(PhilippineEwtToken {
            base: BaseTokenFields::new(raw_text, position),
            rate,
            ewt_type: EwtType::Other(rate),
            applies_to: json.get("applies_to")
                .and_then(|v| v.as_str())
                .and_then(|s| uuid::Uuid::parse_str(s).ok()),
        }))
    }
    
    fn patterns(&self) -> Vec<String> {
        vec![
            r"(?i)EWT".to_string(),
            r"(?i)withholding\s+tax".to_string(),
            r"(?i)less\s+\d+%\s+EWT".to_string(),
        ]
    }
}

struct SeniorPwdDiscountCreator;

impl TokenCreator for SeniorPwdDiscountCreator {
    fn token_type(&self) -> &str {
        "discount:senior_pwd:ph"
    }
    
    fn create(&self, input: &str, position: TokenPosition) -> Result<Box<dyn Token>> {
        let discount_type = if input.contains("senior") {
            SeniorPwdType::Senior
        } else {
            SeniorPwdType::PWD
        };
        
        Ok(Box::new(PhilippineSeniorPwdToken {
            base: BaseTokenFields::new(input.to_string(), position),
            discount_type,
            id_number: None,
        }))
    }
    
    fn create_from_json(&self, json: Value) -> Result<Box<dyn Token>> {
        let raw_text = json.get("raw_text")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let position = TokenPosition {
            start: 0,
            end: raw_text.len(),
            line: 0,
            column: 0,
        };
        
        let discount_type = json.get("discount_type")
            .and_then(|v| v.as_str())
            .map(|s| match s {
                "Senior" => SeniorPwdType::Senior,
                _ => SeniorPwdType::PWD,
            })
            .unwrap_or(SeniorPwdType::Senior);
        
        Ok(Box::new(PhilippineSeniorPwdToken {
            base: BaseTokenFields::new(raw_text, position),
            discount_type,
            id_number: json.get("id_number")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
        }))
    }
    
    fn patterns(&self) -> Vec<String> {
        vec![
            r"(?i)senior\s+(citizen\s+)?discount".to_string(),
            r"(?i)PWD\s+discount".to_string(),
            r"(?i)20%\s+discount".to_string(),
        ]
    }
}