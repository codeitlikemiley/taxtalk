# Token Trait Architecture

## Core Token Trait Definition

```rust
use std::any::{Any, TypeId};
use std::fmt::Debug;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use async_trait::async_trait;

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
```

## Base Token Implementation Helper

```rust
/// Helper macro to implement common Token trait methods
#[macro_export]
macro_rules! impl_token_base {
    ($type:ty) => {
        fn uuid(&self) -> Uuid {
            self.uuid
        }
        
        fn raw_text(&self) -> &str {
            &self.raw_text
        }
        
        fn position(&self) -> &TokenPosition {
            &self.position
        }
        
        fn clone_box(&self) -> Box<dyn Token> {
            Box::new(self.clone())
        }
        
        fn as_any(&self) -> &dyn Any {
            self
        }
        
        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
        
        fn type_id(&self) -> TypeId {
            TypeId::of::<$type>()
        }
        
        fn metadata(&self) -> TokenMetadata {
            self.metadata.clone()
        }
    };
}

/// Base fields that most tokens will have
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseTokenFields {
    pub uuid: Uuid,
    pub raw_text: String,
    pub position: TokenPosition,
    pub metadata: TokenMetadata,
}

impl BaseTokenFields {
    pub fn new(raw_text: String, position: TokenPosition) -> Self {
        let uuid = Uuid::new_v4(); // Or use v5 for deterministic
        Self {
            uuid,
            raw_text,
            position,
            metadata: TokenMetadata {
                source_plugin: None,
                confidence: 1.0,
                alternatives: vec![],
                tags: vec![],
                created_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
        }
    }
}
```

## Concrete Token Implementations

```rust
/// Entity Reference Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRef {
    pub base: BaseTokenFields,
    pub plugin: String,
    pub entity_type: String,
    pub entity_id: String,
    pub display_name: Option<String>,
    pub resolver: Arc<dyn EntityResolver>,
}

impl Token for EntityRef {
    fn token_type(&self) -> &str {
        "entity_ref"
    }
    
    impl_token_base!(EntityRef);
    
    fn to_json(&self) -> Value {
        json!({
            "type": "entity_ref",
            "uuid": self.uuid.to_string(),
            "plugin": self.plugin,
            "entity_type": self.entity_type,
            "entity_id": self.entity_id,
            "display_name": self.display_name,
            "raw_text": self.raw_text,
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

/// Entity Resolver trait
#[async_trait]
pub trait EntityResolver: Send + Sync {
    async fn resolve(&self, entity_id: &str) -> Result<Value, Error>;
    async fn search(&self, query: &str) -> Result<Vec<EntitySearchResult>, Error>;
    fn entity_type(&self) -> &str;
}

/// Amount Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub base: BaseTokenFields,
    pub value: Decimal,
    pub currency: Option<Box<dyn Token>>, // Can reference a Currency token
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
            "value": self.value.to_string(),
            "currency": self.currency.as_ref().map(|c| c.to_json()),
            "format": self.format,
            "raw_text": self.raw_text,
        })
    }
    
    fn matches(&self, pattern: &dyn TokenPattern) -> bool {
        pattern.matches(self)
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        if self.value < Decimal::zero() {
            return Err(ValidationError::InvalidToken {
                reason: "Amount cannot be negative".to_string(),
            });
        }
        Ok(())
    }
    
    fn can_compose_with(&self, other: &dyn Token) -> bool {
        // Amounts can compose with taxes, discounts, etc.
        matches!(other.token_type(), "tax" | "discount" | "entity_ref")
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
            "verb": self.verb,
            "tense": self.tense,
            "variations": self.variations,
            "raw_text": self.raw_text,
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
    
    fn can_compose_with(&self, other: &dyn Token) -> bool {
        // Actions compose with almost everything
        true
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
            "value": self.value.to_string(),
            "unit": self.unit,
            "raw_text": self.raw_text,
        })
    }
    
    fn matches(&self, pattern: &dyn TokenPattern) -> bool {
        pattern.matches(self)
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        if self.value <= Decimal::zero() {
            return Err(ValidationError::InvalidToken {
                reason: "Quantity must be positive".to_string(),
            });
        }
        Ok(())
    }
    
    fn can_compose_with(&self, other: &dyn Token) -> bool {
        matches!(other.token_type(), "product" | "entity_ref" | "amount")
    }
}
```

## Specialized Token Traits

```rust
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
```

## Composite Token Implementation

```rust
/// Composite token that contains multiple tokens
#[derive(Debug, Clone)]
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
}

impl Token for CompositeToken {
    fn token_type(&self) -> &str {
        "composite"
    }
    
    impl_token_base!(CompositeToken);
    
    fn to_json(&self) -> Value {
        json!({
            "type": "composite",
            "composition_type": self.composition_type,
            "tokens": self.tokens.iter().map(|t| t.to_json()).collect::<Vec<_>>(),
            "raw_text": self.raw_text,
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
        Ok(())
    }
    
    fn can_compose_with(&self, other: &dyn Token) -> bool {
        // Check if any contained token can compose
        self.tokens.iter().any(|t| t.can_compose_with(other))
    }
}
```

## Plugin-Specific Token Example

```rust
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
            "rate": self.rate.to_string(),
            "inclusive": self.inclusive,
            "applies_to": self.applies_to.map(|u| u.to_string()),
            "raw_text": self.raw_text,
        })
    }
    
    fn matches(&self, pattern: &dyn TokenPattern) -> bool {
        pattern.matches(self)
    }
    
    fn validate(&self) -> Result<(), ValidationError> {
        if self.rate != Decimal::from_str("0.12").unwrap() {
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
            let base = amount / (Decimal::ONE + self.rate);
            let tax = amount - base;
            TaxComputation { base, tax, total: amount }
        } else {
            let tax = amount * self.rate;
            TaxComputation { base: amount, tax, total: amount + tax }
        }
    }
    
    fn applies_to(&self) -> Vec<String> {
        vec!["amount".to_string()]
    }
}
```

## Token Factory and Registry

```rust
/// Token factory for creating tokens from patterns
pub struct TokenFactory {
    creators: HashMap<String, Box<dyn TokenCreator>>,
}

/// Trait for token creators
pub trait TokenCreator: Send + Sync {
    fn token_type(&self) -> &str;
    fn create(&self, input: &str, position: TokenPosition) -> Result<Box<dyn Token>, Error>;
    fn patterns(&self) -> Vec<String>; // Regex patterns this creator handles
}

impl TokenFactory {
    pub fn register_creator(&mut self, creator: Box<dyn TokenCreator>) {
        self.creators.insert(creator.token_type().to_string(), creator);
    }
    
    pub fn create_token(
        &self, 
        token_type: &str, 
        input: &str, 
        position: TokenPosition
    ) -> Result<Box<dyn Token>, Error> {
        self.creators
            .get(token_type)
            .ok_or_else(|| Error::UnknownTokenType(token_type.to_string()))?
            .create(input, position)
    }
    
    pub fn create_from_json(&self, json: Value) -> Result<Box<dyn Token>, Error> {
        let token_type = json.get("type")
            .and_then(|t| t.as_str())
            .ok_or_else(|| Error::InvalidJson("Missing type field".to_string()))?;
        
        // Recreate token from JSON
        match token_type {
            "entity_ref" => Ok(Box::new(self.entity_ref_from_json(json)?)),
            "amount" => Ok(Box::new(self.amount_from_json(json)?)),
            "action" => Ok(Box::new(self.action_from_json(json)?)),
            _ => {
                // Try plugin-specific creators
                self.creators
                    .get(token_type)
                    .ok_or_else(|| Error::UnknownTokenType(token_type.to_string()))?
                    .create_from_json(json)
            }
        }
    }
}
```

## Token Validation and Type Checking

```rust
/// Token validator that ensures tokens meet requirements
pub struct TokenValidator {
    rules: Vec<Box<dyn ValidationRule>>,
}

pub trait ValidationRule: Send + Sync {
    fn validate(&self, token: &dyn Token) -> Result<(), ValidationError>;
    fn applies_to(&self) -> &str; // Token type this rule applies to
}

impl TokenValidator {
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
```

## Token Pattern Matching

```rust
/// Pattern implementations for matching tokens
pub struct TypePattern {
    pub token_type: String,
}

impl TokenPattern for TypePattern {
    fn matches(&self, token: &dyn Token) -> bool {
        token.token_type() == self.token_type
    }
    
    fn describe(&self) -> String {
        format!("Type: {}", self.token_type)
    }
}

pub struct CompositePattern {
    pub patterns: Vec<Box<dyn TokenPattern>>,
    pub mode: PatternMode,
}

pub enum PatternMode {
    All,  // All patterns must match
    Any,  // Any pattern must match
    None, // No pattern must match
}

impl TokenPattern for CompositePattern {
    fn matches(&self, token: &dyn Token) -> bool {
        match self.mode {
            PatternMode::All => self.patterns.iter().all(|p| p.matches(token)),
            PatternMode::Any => self.patterns.iter().any(|p| p.matches(token)),
            PatternMode::None => !self.patterns.iter().any(|p| p.matches(token)),
        }
    }
    
    fn describe(&self) -> String {
        format!("{:?} of: {}", self.mode, 
            self.patterns.iter()
                .map(|p| p.describe())
                .collect::<Vec<_>>()
                .join(", "))
    }
}
```

Now the token system is complete with:

1. **Full Token trait** with all necessary methods
2. **Base implementations** for common tokens
3. **Specialized traits** for tax, resolvable, computed tokens
4. **Composite tokens** for complex structures
5. **Token factory** for creation and serialization
6. **Validation system** with rules
7. **Pattern matching** for flexible token selection
8. **Type safety** through Any trait and downcasting

This provides the complete foundation for the plugin system where each plugin can create its own token types while maintaining interoperability!