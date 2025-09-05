# Chat Tokenizer and Command Processing Rules

## Tokenization Architecture

### Parser Stack Requirements
- Use `nom` for structured command parsing
- Use `tokenizers` crate for natural language processing
- Implement multi-stage parsing pipeline
- Support both structured and natural language inputs

### Parsing Pipeline Flow
```
Raw Input → Preprocessing → Command Detection → Entity Extraction → Validation → Event Creation
```

## Command Format Standards

### Structured Command Format
```
@{plugin} {action} {entity} {parameters}
```

Examples:
```
@invoice create client:uriah items:pineapple:5kg:1000:PHP
@client add name:"John Doe" email:john@example.com
@payment record amount:1000 currency:PHP from:uriah invoice:123
```

### Natural Language Format
Support conversational inputs:
```
"uriah bought 5kg of pineapple for 1000 pesos"
"create an invoice for john with 3 items"
"record payment of 500 from client 42"
```

## Nom Parser Implementation

### Parser Combinators Structure
```rust
use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, digit1, space0, space1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, tuple},
};

// Command parser structure
fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("@")(input)?;
    let (input, plugin) = alpha1(input)?;
    let (input, _) = space1(input)?;
    let (input, action) = alpha1(input)?;
    // ... continue parsing
}
```

### Required Parser Functions
- `parse_command`: Main command entry point
- `parse_plugin_name`: Extract plugin identifier
- `parse_action`: Extract action verb
- `parse_entity`: Extract entity type and identifier
- `parse_parameters`: Extract key-value parameters
- `parse_natural_language`: Handle conversational input

## Entity Recognition Rules

### Entity Types
Support these core business entities:
- `client`: Customers/clients
- `invoice`: Bills and invoices
- `payment`: Payment records
- `item`: Inventory items
- `company`: Company information

### Entity Identification Patterns
```rust
// ID patterns to recognize
fn parse_entity_id(input: &str) -> IResult<&str, EntityId> {
    alt((
        map(preceded(tag("id:"), digit1), |id| EntityId::Numeric(id.parse().unwrap())),
        map(preceded(tag("uuid:"), uuid_pattern), EntityId::Uuid),
        map(quoted_string, EntityId::Named),
        map(alpha1, EntityId::Named),
    ))(input)
}
```

### Relationship Extraction
- `for client:uriah`: Links action to specific client
- `from invoice:123`: References source invoice
- `with items:...`: Specifies related items
- `at location:warehouse`: Specifies context

## Parameter Parsing Rules

### Key-Value Parameter Format
```
key:value
key:"quoted value"
key:value1,value2,value3
```

### Quantity and Currency Parsing
```rust
// Parse quantities with units
fn parse_quantity(input: &str) -> IResult<&str, Quantity> {
    let (input, amount) = float(input)?;
    let (input, unit) = opt(alpha1)(input)?;
    Ok((input, Quantity { amount, unit: unit.map(|s| s.to_string()) }))
}

// Parse currency amounts
fn parse_currency(input: &str) -> IResult<&str, Money> {
    let (input, amount) = float(input)?;
    let (input, _) = space0(input)?;
    let (input, currency) = alpha1(input)?;
    Ok((input, Money { amount, currency: currency.to_string() }))
}
```

### Date and Time Parsing
Support relative and absolute dates:
```
"today", "yesterday", "next week"
"2024-01-15", "01/15/2024"
"15 days ago", "in 3 months"
```

## Natural Language Processing

### Tokenizer Configuration
```rust
use tokenizers::Tokenizer;

// Configure tokenizer for business context
fn setup_tokenizer() -> Result<Tokenizer, Box<dyn std::error::Error>> {
    let mut tokenizer = Tokenizer::from_pretrained("bert-base-uncased", None)?;
    
    // Add business-specific vocabulary
    tokenizer.add_tokens(&[
        "invoice", "client", "payment", "pesos", "kg", "items"
    ]);
    
    Ok(tokenizer)
}
```

### Intent Recognition
Identify user intents from natural language:
- Creation: "create", "add", "new", "make"
- Update: "update", "change", "modify", "edit"
- Query: "show", "list", "find", "get", "search"
- Delete: "remove", "delete", "cancel"

### Entity Extraction from Natural Language
```rust
// Extract entities from tokenized text
fn extract_entities(tokens: &[String]) -> Vec<Entity> {
    let mut entities = Vec::new();
    
    // Look for client names
    if let Some(client_idx) = tokens.iter().position(|t| t == "client") {
        if let Some(name) = tokens.get(client_idx + 1) {
            entities.push(Entity::Client(name.clone()));
        }
    }
    
    // Look for quantities and amounts
    for (i, token) in tokens.iter().enumerate() {
        if let Ok(amount) = token.parse::<f64>() {
            if let Some(unit) = tokens.get(i + 1) {
                entities.push(Entity::Quantity(amount, unit.clone()));
            }
        }
    }
    
    entities
}
```

## Command Validation Rules

### Syntax Validation
- Verify plugin exists and is loaded
- Check action is supported by plugin
- Validate parameter format and types
- Ensure required parameters are present

### Semantic Validation
- Verify entity references exist
- Check business rule constraints
- Validate data types and ranges
- Ensure user has required permissions

### Error Messages
Provide helpful error messages:
```rust
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unknown plugin: {0}. Available plugins: {1:?}")]
    UnknownPlugin(String, Vec<String>),
    
    #[error("Invalid action '{0}' for plugin '{1}'. Supported actions: {2:?}")]
    InvalidAction(String, String, Vec<String>),
    
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),
    
    #[error("Invalid parameter format: {0}")]
    InvalidParameterFormat(String),
}
```

## Event Generation

### Command to Event Conversion
Transform parsed commands into Crux events:
```rust
pub struct ParsedCommand {
    pub plugin: String,
    pub action: String,
    pub entity: Option<Entity>,
    pub parameters: HashMap<String, Value>,
    pub metadata: CommandMetadata,
}

impl ParsedCommand {
    pub fn to_event(&self) -> Result<Event, ConversionError> {
        Event {
            id: Uuid::new_v4(),
            plugin_id: self.plugin.clone(),
            action: self.action.clone(),
            data: serde_json::to_value(&self.parameters)?,
            timestamp: Utc::now(),
            user_context: self.metadata.user_context.clone(),
        }
    }
}
```

### Event Metadata
Include context in events:
- User information
- Session data
- Timestamp
- Original command text
- Parsing confidence score

## Preprocessing Rules

### Input Sanitization
- Trim whitespace
- Normalize Unicode characters
- Remove or escape dangerous characters
- Convert to lowercase for command matching

### Command Normalization
- Expand abbreviations (@inv → @invoice)
- Handle typos and variations
- Support multiple languages
- Normalize date formats

### Context Preservation
- Maintain conversation context
- Remember recent entities
- Support pronoun resolution ("add it to the invoice")
- Handle command continuation

## Performance Optimization

### Parser Performance
- Cache compiled parser combinators
- Use efficient string operations
- Minimize allocations during parsing
- Implement parser result caching

### Tokenizer Optimization
- Batch tokenization when possible
- Cache tokenizer instances
- Use efficient vocabulary lookup
- Implement lazy tokenization

### Memory Management
- Reuse parser buffers
- Implement string interning for common tokens
- Use cow strings for zero-copy parsing
- Clean up intermediate parsing results

## Testing Command Processing

### Parser Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_structured_command() {
        let input = "@invoice create client:uriah items:pineapple:5kg:1000:PHP";
        let result = parse_command(input).unwrap();
        assert_eq!(result.plugin, "invoice");
        assert_eq!(result.action, "create");
    }
    
    #[test]
    fn test_natural_language() {
        let input = "uriah bought 5kg of pineapple for 1000 pesos";
        let result = parse_natural_language(input).unwrap();
        assert!(result.entities.contains(&Entity::Client("uriah".to_string())));
    }
}
```

### Integration Testing
- Test complete parsing pipeline
- Verify event generation
- Test error handling paths
- Validate context preservation