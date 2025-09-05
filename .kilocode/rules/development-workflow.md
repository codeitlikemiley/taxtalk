# Focused Development Rules for Crux Architecture

## File Organization Standards

### Core Module Structure (Required)
```
core/src/
├── lib.rs                # Crux App state & plugin manager (main entry)
├── plugin_loader.rs      # Wasmtime plugin loader
├── manifest.rs           # Plugin manifest + schema handling
├── event_bus.rs          # Event system implementation
├── tokenizer.rs          # Chat parsing (nom + tokenizers)
└── schema.rs             # Entity validation
```

### Plugin Template Structure
```
plugins/{plugin-name}/
├── src/lib.rs            # Plugin implementation with WIT exports
├── plugin.json           # Manifest declaring capabilities & commands
└── Cargo.toml            # cdylib target with wit-bindgen
```

## Crux Integration Patterns

### Core App Implementation
File: `core/src/lib.rs`
- Implement Crux App struct with Model/Update/View pattern
- Create CruxCapabilityContext for plugin capability access
- Route all state changes through Crux update function
- Never bypass Crux state machine

### Capability Wrapper Implementation
- Implement http, kv, render, pubsub wrappers in capabilities module
- All plugin side-effects must go through Crux effects
- Capabilities are consumed by plugins, never provided by them
- Use consistent capability interfaces across all plugins

## Chat Processing Implementation

### Tokenizer Requirements (nom + tokenizers)
File: `core/src/tokenizer.rs`
- Parse chat commands: `@client uriah bought 5kg of pineapple for 1000 pesos`
- Output structured data with metadata: client info, items, amounts
- Support token forms: `@alias`, `@alias:id`, `@alias("search")`
- Integration with Crux update() function for event creation

### Command Flow Pattern
```
Chat Input → tokenizer.rs → Structured JSON → Crux update() → Event Bus → Plugin
```
- Never skip tokenizer parsing step
- All commands go through Crux state machine
- Event bus routes to correct plugin based on manifest

## Plugin Development Patterns

### Required Plugin Exports
```rust
// All plugins MUST export these functions for WIT interface
#[no_mangle]
pub extern "C" fn constructor() -> PluginInstance;

#[no_mangle] 
pub extern "C" fn update(data: *const u8, len: usize) -> UpdateResult;

#[no_mangle]
pub extern "C" fn view() -> ViewResult;

#[no_mangle]
pub extern "C" fn schema() -> SchemaDefinition;
```

### Plugin Capability Usage
- Access capabilities through host functions only
- Declare all needed capabilities in plugin.json manifest
- Examples: invoice plugin uses `http` for tax APIs, `kv` for storage
- Never access host resources directly

## Build and Deployment

### Plugin Build Process
1. `cd plugins/{plugin-name}`
2. `cargo build --target wasm32-wasip2 --release`  
3. Copy WASM file to `server/plugins/{plugin-name}.wasm`
4. Ensure `plugin.json` manifest is in `server/plugins/`
5. Server auto-detects and loads plugin

### Target Configuration
- MUST use `wasm32-wasip2` target for all plugins
- Plugin Cargo.toml must specify `crate-type = ["cdylib"]`
- Include wit-bindgen dependency for interface generation
- Use only WASI-compatible dependencies in plugins

## Event Bus Implementation

### Event Structure
- Events flow through Crux state machine to event bus
- Event bus routes to plugins based on manifest command declarations
- Support retries, DLQ, and observability hooks
- All events include trace_id for debugging

### Plugin Communication
- Plugins communicate only through events via Crux
- No direct plugin-to-plugin function calls
- Event routing determined by plugin manifest registration
- Support workflow chains (invoice creation → accounting update)

## Hot Reload Development

### File Watching Setup
- Use notify crate to watch `server/plugins/` directory
- Trigger reload on .wasm file changes
- Preserve Crux state during plugin reloads
- Provide reload status feedback to developers

### Development Workflow
1. Modify plugin source code
2. Rebuild plugin WASM
3. File watcher detects change
4. Plugin automatically reloaded
5. Test new functionality immediately

## Schema Validation Integration

### Entity Schema Management
- All entities must have JSON schemas in plugin manifest
- Validate data against schemas before processing
- Cache schemas per plugin for performance
- Support schema versioning and migration

### Validation Flow
- Tokenizer extracts entities from chat
- Core validates entities against plugin schemas
- Validated data passed to plugins via Crux events
- Plugin receives only valid, typed data

## Error Handling Patterns

### Plugin Error Isolation
- Plugin errors caught by Wasmtime sandbox
- Errors logged with plugin_id and context
- Core system continues running despite plugin failures
- Failed events sent to Dead Letter Queue

### Crux Error Handling
- All errors flow through Crux effect system
- Use Result types for all fallible operations
- Include trace_id in error context
- Provide meaningful error messages for debugging

## Testing Requirements

### Integration Testing
- Test complete chat command flow: input → tokenizer → Crux → plugin → output
- Mock Crux capabilities for isolated plugin testing
- Test plugin loading and hot reload functionality
- Verify event bus routing and error handling

### Plugin Testing
- Test plugin WIT interface exports
- Test capability usage with mocked host functions
- Test schema validation and entity processing
- Test plugin manifest parsing and registration

## Configuration and Security

### Plugin Configuration
- Plugins access config via `get_config(key)` host function
- Secrets accessed via `get_secret(key)` from secure backend
- No direct filesystem access from plugins
- All plugin access audited and logged

### Capability Security
- Only declared manifest capabilities available to plugins
- Capability access goes through Crux security layer
- Plugin sandbox prevents unauthorized resource access
- Audit trail for all capability usage

## Observability Implementation

### Structured Logging
- Use tracing crate for structured logs
- Include trace_id in all log entries
- Log plugin_id and event context
- Support log level configuration per plugin

### Metrics Collection
- Track event processing times
- Monitor plugin load/reload success rates
- Count capability usage per plugin
- Track error rates and retry attempts

## Performance Guidelines

### Plugin Loading Optimization
- Cache compiled WASM modules
- Implement lazy loading for unused plugins
- Minimize plugin initialization time
- Support plugin versioning for cache invalidation

### Runtime Performance
- Optimize JSON serialization between core and plugins
- Use efficient event routing algorithms
- Implement capability call pooling where appropriate
- Monitor memory usage of plugin instances