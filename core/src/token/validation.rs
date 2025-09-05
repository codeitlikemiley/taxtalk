use crate::token::traits::{Token, ValidationError};

/// Token validator that ensures tokens meet requirements
pub struct TokenValidator {
    rules: Vec<Box<dyn ValidationRule>>,
}

pub trait ValidationRule: Send + Sync {
    fn validate(&self, token: &dyn Token) -> Result<(), ValidationError>;
    fn applies_to(&self) -> &str; // Token type this rule applies to, or "*" for all
    fn name(&self) -> &str;
}

impl TokenValidator {
    pub fn new() -> Self {
        let mut validator = Self {
            rules: Vec::new(),
        };
        
        // Add default rules
        validator.add_default_rules();
        validator
    }
    
    fn add_default_rules(&mut self) {
        self.add_rule(Box::new(AmountNonNegativeRule));
        self.add_rule(Box::new(QuantityPositiveRule));
        self.add_rule(Box::new(EntityRefNotEmptyRule));
        self.add_rule(Box::new(PhilippineVatRateRule));
        self.add_rule(Box::new(CompositeTransactionRule));
    }
    
    pub fn add_rule(&mut self, rule: Box<dyn ValidationRule>) {
        self.rules.push(rule);
    }
    
    pub fn validate_token(&self, token: &dyn Token) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        
        // First, run token's own validation
        if let Err(e) = token.validate() {
            errors.push(e);
        }
        
        // Then run external validation rules
        for rule in &self.rules {
            if rule.applies_to() == token.token_type() || rule.applies_to() == "*" {
                if let Err(e) = rule.validate(token) {
                    errors.push(e);
                }
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    pub fn validate_composition(
        &self, 
        tokens: &[Box<dyn Token>]
    ) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        
        // Validate each token
        for token in tokens {
            if let Err(errs) = self.validate_token(token.as_ref()) {
                errors.extend(errs);
            }
        }
        
        // Validate token combinations
        for i in 0..tokens.len() {
            for j in i + 1..tokens.len() {
                if !tokens[i].can_compose_with(tokens[j].as_ref()) {
                    errors.push(ValidationError::IncompatibleTokens {
                        token1: tokens[i].token_type().to_string(),
                        token2: tokens[j].token_type().to_string(),
                    });
                }
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// Default validation rules

struct AmountNonNegativeRule;

impl ValidationRule for AmountNonNegativeRule {
    fn validate(&self, token: &dyn Token) -> Result<(), ValidationError> {
        // This rule is handled by Amount's own validate() method
        Ok(())
    }
    
    fn applies_to(&self) -> &str {
        "amount"
    }
    
    fn name(&self) -> &str {
        "amount_non_negative"
    }
}

struct QuantityPositiveRule;

impl ValidationRule for QuantityPositiveRule {
    fn validate(&self, token: &dyn Token) -> Result<(), ValidationError> {
        // This rule is handled by Quantity's own validate() method
        Ok(())
    }
    
    fn applies_to(&self) -> &str {
        "quantity"
    }
    
    fn name(&self) -> &str {
        "quantity_positive"
    }
}

struct EntityRefNotEmptyRule;

impl ValidationRule for EntityRefNotEmptyRule {
    fn validate(&self, token: &dyn Token) -> Result<(), ValidationError> {
        // This rule is handled by EntityRef's own validate() method
        Ok(())
    }
    
    fn applies_to(&self) -> &str {
        "entity_ref"
    }
    
    fn name(&self) -> &str {
        "entity_ref_not_empty"
    }
}

struct PhilippineVatRateRule;

impl ValidationRule for PhilippineVatRateRule {
    fn validate(&self, token: &dyn Token) -> Result<(), ValidationError> {
        // This rule is handled by PhilippineVatToken's own validate() method
        Ok(())
    }
    
    fn applies_to(&self) -> &str {
        "tax:vat:ph"
    }
    
    fn name(&self) -> &str {
        "philippine_vat_rate"
    }
}

struct CompositeTransactionRule;

impl ValidationRule for CompositeTransactionRule {
    fn validate(&self, token: &dyn Token) -> Result<(), ValidationError> {
        if token.token_type() != "composite" {
            return Ok(());
        }
        
        // Additional validation for composite tokens is handled in CompositeToken's validate()
        Ok(())
    }
    
    fn applies_to(&self) -> &str {
        "composite"
    }
    
    fn name(&self) -> &str {
        "composite_transaction"
    }
}

/// Business rule validation
pub struct BusinessRuleValidator {
    rules: Vec<Box<dyn BusinessRule>>,
}

pub trait BusinessRule: Send + Sync {
    fn validate(&self, tokens: &[Box<dyn Token>]) -> Result<(), ValidationError>;
    fn name(&self) -> &str;
}

impl BusinessRuleValidator {
    pub fn new() -> Self {
        let mut validator = Self {
            rules: Vec::new(),
        };
        
        validator.add_default_business_rules();
        validator
    }
    
    fn add_default_business_rules(&mut self) {
        self.add_rule(Box::new(VatWithEwtRule));
        self.add_rule(Box::new(SeniorDiscountNoVatRule));
        self.add_rule(Box::new(TransactionBalanceRule));
    }
    
    pub fn add_rule(&mut self, rule: Box<dyn BusinessRule>) {
        self.rules.push(rule);
    }
    
    pub fn validate(&self, tokens: &[Box<dyn Token>]) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();
        
        for rule in &self.rules {
            if let Err(e) = rule.validate(tokens) {
                errors.push(e);
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// Business rules

struct VatWithEwtRule;

impl BusinessRule for VatWithEwtRule {
    fn validate(&self, tokens: &[Box<dyn Token>]) -> Result<(), ValidationError> {
        // Check if we have both VAT and EWT tokens
        let has_vat = tokens.iter().any(|t| t.token_type() == "tax:vat:ph");
        let has_ewt = tokens.iter().any(|t| t.token_type() == "tax:ewt:ph");
        
        if has_vat && has_ewt {
            // This is actually valid in Philippines - both can apply
            Ok(())
        } else {
            Ok(())
        }
    }
    
    fn name(&self) -> &str {
        "vat_with_ewt"
    }
}

struct SeniorDiscountNoVatRule;

impl BusinessRule for SeniorDiscountNoVatRule {
    fn validate(&self, tokens: &[Box<dyn Token>]) -> Result<(), ValidationError> {
        // Senior/PWD discounts make items VAT-exempt
        let has_senior_discount = tokens.iter()
            .any(|t| t.token_type() == "discount:senior_pwd:ph");
        
        if has_senior_discount {
            let has_vat = tokens.iter().any(|t| t.token_type() == "tax:vat:ph");
            if has_vat {
                return Err(ValidationError::Custom(
                    "Senior/PWD discounts make items VAT-exempt".to_string()
                ));
            }
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "senior_discount_no_vat"
    }
}

struct TransactionBalanceRule;

impl BusinessRule for TransactionBalanceRule {
    fn validate(&self, tokens: &[Box<dyn Token>]) -> Result<(), ValidationError> {
        // For accounting transactions, debits must equal credits
        // This is a simplified check - real implementation would be more complex
        
        // Look for transaction composite tokens
        let transaction_tokens: Vec<_> = tokens.iter()
            .filter(|t| t.token_type() == "composite")
            .collect();
        
        // For now, just ensure we have proper structure
        // Real implementation would check debit/credit balance
        Ok(())
    }
    
    fn name(&self) -> &str {
        "transaction_balance"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::types::Amount;
    use crate::token::base::BaseTokenFields;
    use crate::token::traits::TokenPosition;
    
    #[test]
    fn test_token_validator() {
        let validator = TokenValidator::new();
        
        // Create a valid amount token
        let amount = Amount {
            base: BaseTokenFields::new(
                "1000".to_string(),
                TokenPosition { start: 0, end: 4, line: 1, column: 1 }
            ),
            value: rust_decimal_macros::dec!(1000),
            currency: Some("PHP".to_string()),
            format: crate::token::traits::AmountFormat::Currency,
        };
        
        // Should validate successfully
        assert!(validator.validate_token(&amount).is_ok());
        
        // Create an invalid amount token (negative)
        let invalid_amount = Amount {
            base: BaseTokenFields::new(
                "-100".to_string(),
                TokenPosition { start: 0, end: 4, line: 1, column: 1 }
            ),
            value: rust_decimal_macros::dec!(-100),
            currency: Some("PHP".to_string()),
            format: crate::token::traits::AmountFormat::Currency,
        };
        
        // Should fail validation
        assert!(validator.validate_token(&invalid_amount).is_err());
    }
}