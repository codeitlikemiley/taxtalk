use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;

use crate::token::traits::{Token, TokenPattern, ValidationError, TaxComputation, TokenMetadata, TokenPosition};
use crate::token::base::BaseTokenFields;
use crate::token::specialized::TaxToken;
use crate::token::specialized::{DiscountToken, DiscountType};
use crate::impl_token_base;

/// Philippine VAT Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilippineVatToken {
    pub base: BaseTokenFields,
    pub rate: Decimal,
    pub inclusive: bool,
    pub applies_to: Option<Uuid>, // UUID of amount token this applies to
}

impl Token for PhilippineVatToken {
    fn token_type(&self) -> &str {
        "tax:vat:ph"
    }
    
    impl_token_base!(PhilippineVatToken);
    
    fn to_json(&self) -> Value {
        json!({
            "type": "tax:vat:ph",
            "uuid": self.uuid().to_string(),
            "rate": self.rate.to_string(),
            "inclusive": self.inclusive,
            "applies_to": self.applies_to.map(|u| u.to_string()),
            "raw_text": self.raw_text(),
        })
    }
    
    fn matches(&self, pattern: &dyn TokenPattern) -> bool {
        pattern.matches(self)
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        if self.rate != dec!(0.12) {
            return Err(ValidationError::InvalidToken {
                reason: "Philippine VAT rate must be 12%".to_string(),
            });
        }
        Ok(())
    }
    
    fn can_compose_with(&self, other: &dyn Token) -> bool {
        matches!(other.token_type(), "amount")
    }
}

impl TaxToken for PhilippineVatToken {
    fn tax_type(&self) -> &str {
        "VAT"
    }
    
    fn rate(&self) -> Option<Decimal> {
        Some(self.rate)
    }
    
    fn is_inclusive(&self) -> bool {
        self.inclusive
    }
    
    fn compute(&self, amount: Decimal) -> TaxComputation {
        if self.inclusive {
            // VAT-inclusive: extract the base from total
            let base = amount / (Decimal::ONE + self.rate);
            let tax = amount - base;
            TaxComputation { base, tax, total: amount }
        } else {
            // VAT-exclusive: add VAT to base
            let tax = amount * self.rate;
            TaxComputation { base: amount, tax, total: amount + tax }
        }
    }
    
    fn applies_to(&self) -> Vec<String> {
        vec!["amount".to_string()]
    }
}

/// Expanded Withholding Tax (EWT) Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilippineEwtToken {
    pub base: BaseTokenFields,
    pub rate: Decimal,
    pub ewt_type: EwtType,
    pub applies_to: Option<Uuid>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EwtType {
    ProfessionalServices, // 10% for VAT-registered, 5% for non-VAT
    Goods,               // 1%
    Rental,              // 5%
    Other(Decimal),      // Custom rate
}

impl Token for PhilippineEwtToken {
    fn token_type(&self) -> &str {
        "tax:ewt:ph"
    }
    
    impl_token_base!(PhilippineEwtToken);
    
    fn to_json(&self) -> Value {
        json!({
            "type": "tax:ewt:ph",
            "uuid": self.uuid().to_string(),
            "rate": self.rate.to_string(),
            "ewt_type": format!("{:?}", self.ewt_type),
            "applies_to": self.applies_to.map(|u| u.to_string()),
            "raw_text": self.raw_text(),
        })
    }
    
    fn matches(&self, pattern: &dyn TokenPattern) -> bool {
        pattern.matches(self)
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        match self.ewt_type {
            EwtType::ProfessionalServices => {
                if self.rate != dec!(0.10) && self.rate != dec!(0.05) {
                    return Err(ValidationError::InvalidToken {
                        reason: "Professional services EWT must be 10% or 5%".to_string(),
                    });
                }
            }
            EwtType::Goods => {
                if self.rate != dec!(0.01) {
                    return Err(ValidationError::InvalidToken {
                        reason: "Goods EWT must be 1%".to_string(),
                    });
                }
            }
            EwtType::Rental => {
                if self.rate != dec!(0.05) {
                    return Err(ValidationError::InvalidToken {
                        reason: "Rental EWT must be 5%".to_string(),
                    });
                }
            }
            EwtType::Other(_) => {}
        }
        Ok(())
    }
    
    fn can_compose_with(&self, other: &dyn Token) -> bool {
        matches!(other.token_type(), "amount" | "tax:vat:ph")
    }
}

impl TaxToken for PhilippineEwtToken {
    fn tax_type(&self) -> &str {
        "EWT"
    }
    
    fn rate(&self) -> Option<Decimal> {
        Some(self.rate)
    }
    
    fn is_inclusive(&self) -> bool {
        false // EWT is always deducted from the gross amount
    }
    
    fn compute(&self, amount: Decimal) -> TaxComputation {
        let tax = amount * self.rate;
        TaxComputation { 
            base: amount, 
            tax, 
            total: amount - tax // EWT reduces the amount to be paid
        }
    }
    
    fn applies_to(&self) -> Vec<String> {
        vec!["amount".to_string()]
    }
}

/// Senior Citizen/PWD Discount Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilippineSeniorPwdToken {
    pub base: BaseTokenFields,
    pub discount_type: SeniorPwdType,
    pub id_number: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SeniorPwdType {
    Senior,
    PWD,
}

impl Token for PhilippineSeniorPwdToken {
    fn token_type(&self) -> &str {
        "discount:senior_pwd:ph"
    }
    
    impl_token_base!(PhilippineSeniorPwdToken);
    
    fn to_json(&self) -> Value {
        json!({
            "type": "discount:senior_pwd:ph",
            "uuid": self.uuid().to_string(),
            "discount_type": format!("{:?}", self.discount_type),
            "id_number": self.id_number,
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
        matches!(other.token_type(), "amount" | "product")
    }
}

impl DiscountToken for PhilippineSeniorPwdToken {
    fn discount_type(&self) -> DiscountType {
        match self.discount_type {
            SeniorPwdType::Senior => DiscountType::Senior,
            SeniorPwdType::PWD => DiscountType::PWD,
        }
    }
    
    fn value(&self) -> Decimal {
        dec!(0.20) // 20% discount
    }
    
    fn applies_to(&self) -> Vec<String> {
        vec!["amount".to_string(), "product".to_string()]
    }
    
    fn compute_discount(&self, amount: Decimal) -> Decimal {
        amount * self.value()
    }
}

/// BIR Form Reference Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirFormToken {
    pub base: BaseTokenFields,
    pub form_type: BirFormType,
    pub period: Option<String>, // e.g., "2024-01" for January 2024
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BirFormType {
    Form2550M,  // Monthly VAT
    Form2550Q,  // Quarterly VAT
    Form1601C,  // Monthly Remittance of Income Tax Withheld
    Form1601E,  // Monthly Remittance of Expanded Withholding Tax
    Form1701Q,  // Quarterly Income Tax Return for Corporation
    Form2307,   // Certificate of Creditable Tax Withheld at Source
}

impl Token for BirFormToken {
    fn token_type(&self) -> &str {
        "bir_form:ph"
    }
    
    impl_token_base!(BirFormToken);
    
    fn to_json(&self) -> Value {
        json!({
            "type": "bir_form:ph",
            "uuid": self.uuid().to_string(),
            "form_type": format!("{:?}", self.form_type),
            "period": self.period,
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
        matches!(other.token_type(), "date" | "action")
    }
}