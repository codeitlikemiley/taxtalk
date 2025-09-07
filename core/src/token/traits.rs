use std::any::{Any, TypeId};
use std::fmt::Debug;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use async_trait::async_trait;
use rust_decimal::Decimal;
use anyhow::Error;

/// Base trait that all tokens must implement
pub trait Token: Debug + Send + Sync + Any {
    /// Unique type identifier for this token
    fn token_type(&self) -> &str;
    
    /// UUID for this token instance (deterministic when possible)
    fn uuid(&self) -> Uuid;
    
    /// Original text that created this token
    fn raw_text(&self) -> &str;
    
    /// Position in original input
    fn position(&self) -> &TokenPosition;
    
    /// Serialize to JSON for transport
    fn to_json(&self) -> Value;
    
    /// Clone as boxed trait object
    fn clone_box(&self) -> Box<dyn Token>;
    
    /// Downcast to concrete type
    fn as_any(&self) -> &dyn Any;
    
    /// Mutable downcast
    fn as_any_mut(&mut self) -> &mut dyn Any;
    
    /// Type ID for runtime type checking
    fn type_id(&self) -> TypeId;
    
    /// Check if this token matches a pattern
    fn matches(&self, pattern: &dyn TokenPattern) -> bool;
    
    /// Validate this token
    fn validate(&self) -> Result<(), ValidationError>;
    
    /// Get metadata about this token
    fn metadata(&self) -> TokenMetadata;
    
    /// Check if this token can compose with another
    fn can_compose_with(&self, other: &dyn Token) -> bool;
    
    /// Display text for UI
    fn display_text(&self) -> String {
        self.raw_text().to_string()
    }
}

/// Position information for error reporting and reconstruction
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TokenPosition {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

/// Metadata about a token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetadata {
    pub source_plugin: Option<String>,
    pub confidence: f32,
    pub alternatives: Vec<String>,
    pub tags: Vec<String>,
    pub created_at: u64,
}

/// Pattern matching for tokens
pub trait TokenPattern: Debug + Send + Sync {
    fn matches(&self, token: &dyn Token) -> bool;
    fn describe(&self) -> String;
}

/// Validation errors for tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationError {
    InvalidToken { reason: String },
    IncompatibleTokens { token1: String, token2: String },
    MissingRequiredField { field: String },
    InvalidValue { field: String, value: String },
    Custom(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidToken { reason } => write!(f, "Invalid token: {reason}"),
            ValidationError::IncompatibleTokens { token1, token2 } => {
                write!(f, "Incompatible tokens: {token1} and {token2}")
            }
            ValidationError::MissingRequiredField { field } => {
                write!(f, "Missing required field: {field}")
            }
            ValidationError::InvalidValue { field, value } => {
                write!(f, "Invalid value for field {field}: {value}")
            }
            ValidationError::Custom(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Entity search results for token resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySearchResult {
    pub id: String,
    pub name: String,
    pub entity_type: String,
    pub score: f32,
}

/// Tax computation result
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TaxComputation {
    pub base: Decimal,
    pub tax: Decimal,
    pub total: Decimal,
}

/// Computation context for computed tokens
#[derive(Debug)]
pub struct ComputationContext {
    pub tokens: Vec<Box<dyn Token>>,
    pub variables: std::collections::HashMap<String, Value>,
}

/// Tense for action tokens
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Tense {
    Past,
    Present,
    Future,
}

/// Amount format
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AmountFormat {
    Standard,
    Accounting,
    Currency,
}

/// Entity Resolver trait
#[async_trait]
pub trait EntityResolver: Debug + Send + Sync {
    async fn resolve(&self, entity_id: &str) -> Result<Value, Error>;
    async fn search(&self, query: &str) -> Result<Vec<EntitySearchResult>, Error>;
    fn entity_type(&self) -> &str;
}