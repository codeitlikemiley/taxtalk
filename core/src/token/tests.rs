#[cfg(test)]
mod tests {
    use crate::token::*;
    use rust_decimal_macros::dec;
    
    #[test]
    fn test_amount_token_creation() {
        let position = TokenPosition {
            start: 0,
            end: 4,
            line: 1,
            column: 1,
        };
        
        let amount = Amount {
            base: BaseTokenFields::new("1000".to_string(), position),
            value: dec!(1000),
            currency: Some("PHP".to_string()),
            format: AmountFormat::Currency,
        };
        
        assert_eq!(amount.token_type(), "amount");
        assert_eq!(amount.value, dec!(1000));
        assert_eq!(amount.currency, Some("PHP".to_string()));
        assert!(amount.validate().is_ok());
    }
    
    #[test]
    fn test_philippine_vat_computation() {
        let vat = PhilippineVatToken {
            base: BaseTokenFields::new("VAT".to_string(), TokenPosition {
                start: 0,
                end: 3,
                line: 1,
                column: 1,
            }),
            rate: dec!(0.12),
            inclusive: false,
            applies_to: None,
        };
        
        // Test VAT-exclusive calculation
        let computation = vat.compute(dec!(1000));
        assert_eq!(computation.base, dec!(1000));
        assert_eq!(computation.tax, dec!(120));
        assert_eq!(computation.total, dec!(1120));
        
        // Test VAT-inclusive calculation
        let mut vat_inclusive = vat;
        vat_inclusive.inclusive = true;
        let computation = vat_inclusive.compute(dec!(1120));
        assert_eq!(computation.base, dec!(1000));
        assert_eq!(computation.tax, dec!(120));
        assert_eq!(computation.total, dec!(1120));
    }
    
    #[test]
    fn test_token_factory() {
        let factory = TokenFactory::new();
        let position = TokenPosition {
            start: 0,
            end: 10,
            line: 1,
            column: 1,
        };
        
        // Create an amount token
        let amount_token = factory.create_token("amount", "5000", position).unwrap();
        assert_eq!(amount_token.token_type(), "amount");
        
        // Create an action token
        let action_token = factory.create_token("action", "buy", position).unwrap();
        assert_eq!(action_token.token_type(), "action");
    }
    
    #[test]
    fn test_composite_token() {
        let position = TokenPosition {
            start: 0,
            end: 20,
            line: 1,
            column: 1,
        };
        
        let amount = Box::new(Amount {
            base: BaseTokenFields::new("1000".to_string(), position),
            value: dec!(1000),
            currency: Some("PHP".to_string()),
            format: AmountFormat::Currency,
        });
        
        let action = Box::new(ActionToken {
            base: BaseTokenFields::new("bought".to_string(), position),
            verb: "bought".to_string(),
            tense: Tense::Past,
            variations: std::collections::HashSet::new(),
        });
        
        let composite = CompositeToken::new(
            vec![action, amount],
            CompositionType::Transaction
        );
        
        assert_eq!(composite.token_type(), "composite");
        assert_eq!(composite.tokens.len(), 2);
        assert!(composite.find_token_by_type("amount").is_some());
        assert!(composite.find_token_by_type("action").is_some());
    }
    
    #[test]
    fn test_token_validation() {
        let validator = TokenValidator::new();
        
        // Test valid amount
        let valid_amount = Amount {
            base: BaseTokenFields::new("1000".to_string(), TokenPosition {
                start: 0,
                end: 4,
                line: 1,
                column: 1,
            }),
            value: dec!(1000),
            currency: Some("PHP".to_string()),
            format: AmountFormat::Currency,
        };
        assert!(validator.validate_token(&valid_amount).is_ok());
        
        // Test invalid amount (negative)
        let invalid_amount = Amount {
            base: BaseTokenFields::new("-100".to_string(), TokenPosition {
                start: 0,
                end: 4,
                line: 1,
                column: 1,
            }),
            value: dec!(-100),
            currency: Some("PHP".to_string()),
            format: AmountFormat::Currency,
        };
        assert!(validator.validate_token(&invalid_amount).is_err());
    }
}