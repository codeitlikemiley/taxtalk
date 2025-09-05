use async_trait::async_trait;
use rust_decimal::Decimal;
use serde_json::Value;
use anyhow::Error;

use crate::token::traits::{Token, TaxComputation, ComputationContext};

/// Trait for tokens that represent tax calculations
pub trait TaxToken: Token {
    fn tax_type(&self) -> &str;
    fn rate(&self) -> Option<Decimal>;
    fn is_inclusive(&self) -> bool;
    fn compute(&self, amount: Decimal) -> TaxComputation;
    fn applies_to(&self) -> Vec<String>; // What token types this tax applies to
}

/// Trait for tokens that can be resolved to entities
#[async_trait]
pub trait ResolvableToken: Token {
    async fn resolve(&self) -> Result<Value, Error>;
    fn needs_resolution(&self) -> bool;
    fn resolution_plugin(&self) -> &str;
}

/// Trait for tokens that carry computed values
pub trait ComputedToken: Token {
    fn compute(&self, context: &ComputationContext) -> Result<Value, Error>;
    fn dependencies(&self) -> Vec<String>; // Other token types needed for computation
}

/// Trait for tokens that can transform other tokens
pub trait TransformerToken: Token {
    fn can_transform(&self, token: &dyn Token) -> bool;
    fn transform(&self, token: Box<dyn Token>) -> Result<Box<dyn Token>, Error>;
}

/// Trait for discount tokens
pub trait DiscountToken: Token {
    fn discount_type(&self) -> DiscountType;
    fn value(&self) -> Decimal;
    fn applies_to(&self) -> Vec<String>;
    fn compute_discount(&self, amount: Decimal) -> Decimal;
}

#[derive(Debug, Clone, Copy)]
pub enum DiscountType {
    Percentage,
    Fixed,
    Senior,
    PWD,
    Promotional,
}

/// Trait for document tokens (receipts, invoices, etc)
pub trait DocumentToken: Token {
    fn document_type(&self) -> &str;
    fn document_number(&self) -> Option<String>;
    fn series(&self) -> Option<String>;
    fn date(&self) -> Option<chrono::NaiveDate>;
    fn is_sequential(&self) -> bool;
}

/// Trait for payment method tokens
pub trait PaymentToken: Token {
    fn payment_method(&self) -> PaymentMethod;
    fn reference_number(&self) -> Option<String>;
    fn processor(&self) -> Option<String>;
}

#[derive(Debug, Clone)]
pub enum PaymentMethod {
    Cash,
    Check { number: String, bank: String },
    BankTransfer { account: String },
    CreditCard { last_four: String },
    GCash { mobile: String },
    Maya { mobile: String },
    Other(String),
}