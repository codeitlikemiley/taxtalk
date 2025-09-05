# Plugin Development Rules

## Plugin Manifest Standards

### Required Manifest Structure
Every plugin MUST include a complete manifest.json:

```json
{
  "id": "invoice-manager",
  "version": "0.1.0",
  "entry": "lib.wasm",
  "capabilities": ["http", "kv", "render"],
  "commands": [
    {
      "name": "create_invoice",
      "schema": "invoice_schema.json",
      "aliases": ["@invoice", "@inv"]
    }
  ],
  "schemas": {
    "invoice": "schemas/invoice.json",
    "client": "schemas/client.json"
  },
  "dependencies": ["company", "payment"]
}
```

### Plugin ID Conventions
- Use kebab-case for plugin IDs
- Make IDs descriptive of business function
- Avoid generic names like "manager" or "handler"
- Examples: `invoice-manager`, `crm-contacts`, `inventory-tracker`

## WIT Interface Implementation

### Required Exports
All plugins MUST export these functions:
```rust
#[no_mangle]
pub extern "C" fn constructor() -> *mut PluginInstance

#[no_mangle] 
pub extern "C" fn update(data: *const u8, len: usize) -> UpdateResult

#[no_mangle]
pub extern "C" fn resolve(effect_id: u32, data: *const u8, len: usize) -> ResolveResult

#[no_mangle]
pub extern "C" fn view() -> ViewResult

#[no_mangle] 
pub extern "C" fn schema() -> *const c_char
```

### Memory Management
- Always free allocated memory in plugin code
- Use `Box::from_raw` and `Box::into_raw` for FFI
- Never leak memory across the WASM boundary
- Implement proper Drop traits for plugin resources

## Business Logic Patterns

### Entity Creation Commands
When processing entity creation (invoices, clients, etc.):
1. Parse command into structured data
2. Validate against schema
3. Check business rules (e.g., client exists)
4. Generate unique ID
5. Store via KV capability
6. Emit creation event
7. Return success response

### Command Parsing Examples
Support these natural language patterns:
```
@client uriah bought 5kg of pineapple for 1000 pesos
@invoice create for client:uriah items:pineapple:5kg:1000:PHP
@payment record 1000 pesos from uriah for invoice:123
@inventory add 10 units of pineapple at 200 pesos each
```

## Capability Usage Rules

### HTTP Capability
- Use for external API calls only
- Implement proper timeout handling
- Cache responses when appropriate
- Handle network failures gracefully

### KV (Key-Value) Capability
- Use consistent key naming patterns: `{plugin_id}:{entity_type}:{id}`
- Implement proper serialization/deserialization
- Handle concurrent access scenarios
- Implement soft delete patterns where needed

### Render Capability
- Only call render after state changes
- Include minimal data needed for UI updates
- Use consistent view data structures
- Implement partial updates when possible

## Error Handling in Plugins

### Error Types
Define plugin-specific error types:
```rust
#[derive(Debug, thiserror::Error)]
pub enum InvoiceError {
    #[error("Client {0} not found")]
    ClientNotFound(String),
    
    #[error("Invalid amount: {0}")]
    InvalidAmount(f64),
    
    #[error("Schema validation failed: {0}")]
    ValidationError(String),
}
```

### Error Propagation
- Never panic in plugin code
- Always return proper error results
- Include context in error messages
- Log errors using structured logging

## Testing Plugin Code

### Unit Testing Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_invoice_creation() {
        // Test invoice creation logic
    }
    
    #[test] 
    fn test_command_parsing() {
        // Test chat command parsing
    }
    
    #[test]
    fn test_schema_validation() {
        // Test schema validation
    }
}
```

### Mock Capabilities
- Create mock implementations for testing
- Test capability failure scenarios
- Verify capability call patterns
- Test plugin isolation

## Schema Definition Standards

### JSON Schema Format
Use JSON Schema Draft 7 for all entity definitions:
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "id": {"type": "string", "format": "uuid"},
    "created_at": {"type": "string", "format": "date-time"},
    "amount": {"type": "number", "minimum": 0}
  },
  "required": ["id", "created_at", "amount"]
}
```

### Schema Validation Rules
- Validate all input data before processing
- Use the core schema validation module
- Provide clear validation error messages
- Support schema versioning and migration

## Plugin Dependencies

### Dependency Declaration
- List all plugin dependencies in manifest
- Use semantic versioning for dependencies
- Handle circular dependencies gracefully
- Support optional dependencies

### Inter-Plugin Communication
- Use event bus for plugin communication
- Never call plugin functions directly
- Emit events for state changes
- Subscribe to relevant events from other plugins

## Performance Optimization

### WASM Optimization
- Use `opt-level = "s"` for size optimization
- Enable LTO in release builds
- Minimize string allocations
- Use efficient serialization formats

### Caching Strategies
- Cache frequently accessed data
- Implement TTL for cached data
- Use memory-efficient cache structures
- Clear cache on relevant events

## Hot Reload Support

### State Preservation
- Implement state serialization/deserialization
- Preserve important state across reloads
- Handle version migration during reload
- Provide rollback on reload failure

### Development Workflow
- Watch source files for changes
- Automatically rebuild and reload
- Provide reload status feedback
- Support partial reloads when possible