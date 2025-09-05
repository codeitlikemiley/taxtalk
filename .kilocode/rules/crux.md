# Crux-Based Plugin Architecture Rules

## Project-Specific Architecture

### Core Layer (Crux + Wasmtime)
- The core layer MUST use Crux as the state management foundation
- All state changes go through Crux's `update()` function
- Use Wasmtime for dynamic plugin loading and execution
- Never bypass the Crux state machine for business logic
- All plugins communicate through Crux capabilities, never directly

### Plugin System Rules
- Plugins MUST be compiled to WASM and loaded via Wasmtime
- Each plugin MUST have a manifest.json defining capabilities and schemas
- Plugins MUST implement the WIT interface defined in `core/wit/core.wit`
- Plugins can only access host capabilities explicitly declared in their manifest
- Use `wit-bindgen` for generating plugin bindings

### Event Flow Architecture
```
Chat Input → Tokenizer → Crux Update → Event Bus → Plugin → Capabilities → Render
```
- Follow this exact flow for all user interactions
- Never short-circuit the event bus
- All parsing goes through `tokenizer.rs` using nom + tokenizers

## File Organization Standards

### Required Project Structure
```
project-root/
├── core/                      # Crux + Wasmtime runtime
│   ├── wit/                   # WIT files for plugin interfaces
│   ├── src/
│   │   ├── lib.rs             # Main Crux App
│   │   ├── plugin_loader.rs   # Wasmtime management
│   │   ├── manifest.rs        # Plugin manifest handling
│   │   ├── event_bus.rs       # Event routing system
│   │   ├── tokenizer.rs       # Chat parsing (nom + tokenizers)
│   │   └── schema.rs          # Entity validation
├── plugins/                   # Individual business modules
│   ├── [plugin-name]/
│   │   ├── src/lib.rs
│   │   ├── manifest.json
│   │   └── schemas/
└── server/                    # HTTP interface (Axum)
```

### Naming Conventions
- Plugin directories: lowercase with hyphens (`invoice-manager`, `crm-contacts`)
- WIT files: snake_case (`core.wit`, `shared_types.wit`)
- Rust modules: snake_case following standard conventions
- Capability names: kebab-case (`http-client`, `key-value`, `pub-sub`)

## WIT Interface Standards

### Core Interface Requirements
```wit
// All plugins MUST implement this interface
resource instance {
    constructor();
    update: func(data: list<u8>) -> result<list<u8>, string>;
    resolve: func(effect-id: u32, data: list<u8>) -> result<list<u8>, string>;
    view: func() -> result<list<u8>, string>;
    schema: func() -> string;
}
```

### Capability Import Rules
- Only import capabilities that are declared in the plugin manifest
- Use standard capability names: `http`, `kv`, `render`, `pub-sub`
- Never create custom capability interfaces without architectural approval

## Plugin Development Rules

### Manifest Requirements
Every plugin MUST include:
- `id`: Unique identifier
- `version`: Semantic versioning
- `entry`: WASM file name
- `capabilities`: List of required Crux capabilities
- `commands`: Chat commands and aliases
- `schemas`: Entity schema definitions
- `dependencies`: Other plugins this depends on

### Plugin Implementation Standards
- Use `#[no_mangle]` for all exported functions
- Handle all errors gracefully - never panic in plugin code
- Serialize all data exchange as JSON through serde
- Implement proper lifecycle methods (init, update, cleanup)
- Document all exported functions with rustdoc

### Schema Validation
- All entities MUST have corresponding JSON schemas
- Validate all input data against schemas before processing
- Use the core `schema.rs` module for validation logic
- Never trust data from chat input without validation

## Chat Processing Rules

### Tokenization Standards
- Use nom for structured parsing of chat commands
- Use tokenizers crate for NLP processing of natural language
- Parse format: `@command entity action parameters`
- Examples:
  ```
  @client uriah bought 5kg of pineapple for 1000 pesos
  @invoice create for client:uriah items:pineapple:5kg:1000
  ```

### Command Routing
- Route parsed commands through the event bus
- Match commands to plugins based on manifest registration
- Support command aliases for user convenience
- Provide helpful error messages for unrecognized commands

### Data Flow
1. Raw chat text → `tokenizer.rs`
2. Structured data → Crux `update()`
3. Event → `event_bus.rs`
4. Plugin execution in Wasmtime sandbox
5. Result → Crux state update
6. UI update via `render` capability

## Dependency Management

### Core Dependencies (Required)
```toml
crux_core = "0.7.0"           # State management
wasmtime = "36.0.2"           # Plugin runtime
serde = { version = "1.0.210", features = ["derive"] }
serde_json = "1.0.128"        # JSON serialization
nom = "8.0.0"                 # Chat parsing
tokenizers = "0.21.2"         # NLP processing
notify = "6.1.1"              # Hot reload
uuid = { version = "1.10.0", features = ["v4"] }
thiserror = "1.0.65"          # Error handling
tracing = "0.1.40"            # Logging
```

### Plugin Dependencies
- Plugins should minimize external dependencies
- Use only WASI-compatible crates in plugins
- Prefer core capabilities over direct external calls
- Document all plugin dependencies in manifest

## Error Handling Standards

### Error Propagation
- Use `Result<T, E>` for all fallible operations
- Create custom error types using `thiserror`
- Never use `unwrap()` or `expect()` in production code
- Provide meaningful error context for debugging

### Plugin Error Isolation
- Plugin errors MUST NOT crash the core system
- Catch and log all plugin exceptions
- Provide fallback behavior when plugins fail
- Use Wasmtime's trap handling for safety

## Security and Sandboxing

### Plugin Isolation
- Plugins run in Wasmtime sandbox with limited capabilities
- Only capabilities declared in manifest are available
- No direct file system access unless explicitly granted
- No network access except through `http` capability

### Input Validation
- Validate all chat input before processing
- Sanitize data passed between plugins
- Use schema validation for all entity operations
- Never trust plugin-provided data without verification

## Testing Requirements

### Core Testing
- Unit tests for all tokenizer parsing logic
- Integration tests for plugin loading/unloading
- Test Crux state transitions thoroughly
- Mock capabilities for isolated testing

### Plugin Testing
- Each plugin MUST have comprehensive unit tests
- Test plugin manifest loading and validation
- Test capability usage and error handling
- Use `wasmtime` test harness for WASM testing

### End-to-End Testing
- Test complete chat command flows
- Verify plugin interactions through event bus
- Test hot-reload functionality
- Performance testing for plugin execution

## Performance Guidelines

### Plugin Loading
- Implement lazy loading for plugins
- Cache compiled WASM modules
- Support hot-reload during development
- Minimize startup time for new plugin instances

### Memory Management
- Monitor memory usage of plugin instances
- Implement plugin lifecycle management
- Clean up resources when plugins are unloaded
- Use memory-efficient serialization formats

### Capability Optimization
- Pool capability instances when possible
- Implement capability caching where appropriate
- Monitor capability call frequency and performance
- Optimize JSON serialization/deserialization

## Development Workflow

### Hot Reload Support
- Watch plugin directories for changes
- Automatically recompile and reload modified plugins
- Preserve system state during plugin reloads
- Provide clear feedback on reload success/failure

### Debugging
- Use `tracing` for structured logging throughout
- Include plugin ID and event context in all logs
- Implement debug modes for verbose plugin execution
- Support plugin debugging through Wasmtime

### Documentation
- Document all capabilities and their interfaces
- Maintain up-to-date plugin development guides
- Include examples for common plugin patterns
- Keep WIT interface documentation current

## Business Domain Rules

### Entity Management
- Use UUID for all entity identifiers
- Implement proper entity relationships through schemas
- Support entity lifecycle events (create, update, delete)
- Maintain audit trails for important business operations

### Command Patterns
- Support natural language commands where possible
- Provide shortcuts for common operations
- Implement command history and suggestions
- Support bulk operations through batch commands

### Data Consistency
- Ensure cross-plugin data consistency
- Implement proper transaction boundaries
- Handle concurrent access to shared data
- Provide rollback capabilities for failed operations