# KiloCode Rules for Crux + WASM Plugin Architecture

## Core Architecture Enforcement

### Crux as Foundation
- Crux MUST be the state management and capability layer at the heart of the system
- All components MUST interact through Crux - never bypass it
- Use Crux's Model/Update/View/Effect pattern for all state changes
- Plugins are dynamic capability providers registered in Crux at runtime

### Layer Responsibilities (Strict)
- **Core**: Crux + Wasmtime for state management, rendering, plugin runtime, event bus
- **Server**: Axum + Core for HTTP/SSE/WebSocket bridge and chat interface  
- **Plugins**: WASM + wit-bindgen for modular business features

### Required Project Structure
```
project-root/
├── core/                      # Crux + Wasmtime runtime
│   ├── wit/
│   │   ├── core.wit          # Shared Crux interface
│   │   └── world.wit         # Capability imports/exports
│   ├── src/
│   │   ├── lib.rs            # Crux App state & plugin manager
│   │   ├── plugin_loader.rs  # Wasmtime plugin loader
│   │   ├── manifest.rs       # Plugin manifest + schema
│   │   ├── event_bus.rs      # Event system
│   │   ├── tokenizer.rs      # nom + tokenizers parsing
│   │   └── schema.rs         # Entity validation
├── plugins/
│   ├── invoice/src/lib.rs
│   └── company/src/lib.rs
└── server/src/main.rs
```

## WIT Interface Standards

### Core Interface (Required)
File: `core/wit/core.wit`
```wit
package crux:shared-lib;

interface core {
    resource instance {
        constructor();
        update: func(data: list<u8>) -> result<list<u8>, string>;
        resolve: func(effect-id: u32, data: list<u8>) -> result<list<u8>, string>;
        view: func() -> result<list<u8>, string>;
        schema: func() -> string;
    }
}
```

### World Capabilities (Required)
File: `core/wit/world.wit`
```wit
package crux:shared-lib;

world shared {
    export core;
    import http: func(request: string) -> string;
    import render: func(view: string);
    import kv: func(key: string, value: string);
}
```

## Plugin Architecture Rules

### Plugin Integration with Crux
- Each plugin is a dynamic capability provider
- Plugins are registered in Crux at runtime by Wasmtime loader
- Plugins consume Crux capabilities (http, kv, render, pub/sub) - never direct access
- Examples: invoice plugin uses `http` for tax APIs, crm plugin uses `kv` for persistence

### Plugin Manifest Format (Exact)
Every plugin MUST include `plugin.json`:
```json
{
  "id": "invoice",
  "version": "0.1.0", 
  "entry": "lib.wasm",
  "capabilities": ["http", "kv"],
  "commands": [
    {
      "name": "create_invoice",
      "schema": "invoice_schema.json",
      "aliases": ["@invoice"]
    }
  ],
  "schemas": {
    "invoice": "schemas/invoice.json",
    "client": "schemas/client.json"
  },
  "dependencies": ["company", "payment"]
}
```

### Plugin Communication Rules
- Plugins are completely decoupled - no direct calls between them
- All communication through Crux event bus
- Adding features = drop in WASM file, no server code changes
- Clean separation: Business logic (Crux core), Dynamic modules (plugins), Presentation (render capability)

## Chat Processing Flow

### Input Processing Pattern
1. User input: `@client uriah bought 5kg of pineapple for 1000 pesos`
2. `tokenizer.rs` (nom + tokenizers) parses into structured data
3. Crux `update()` receives JSON as event
4. Crux routes event → Event bus → Correct plugin
5. Plugin executes in Wasmtime sandbox using Crux capabilities
6. Result returned → Core triggers `render` → Server pushes to UI

### Tokenizer Requirements
- Use nom for structured parsing of chat commands
- Use tokenizers crate for NLP processing
- Parse tokens like `@client`, `@invoice` into structured commands
- Output structured data with metadata for entity resolution

### Expected Data Flow
Input: Chat text → Tokenizer → Structured JSON → Crux update → Event bus → Plugin execution → Render

## Required Dependencies

### Core Module Dependencies (Exact Versions)
```toml
[dependencies]
crux_core = "0.7.0"
wasmtime = "36.0.2"
serde = { version = "1.0.210", features = ["derive"] }
serde_json = "1.0.128"
tokio = { version = "1.40.0", features = ["macros", "rt-multi-thread"] }
nom = "8.0.0"
tokenizers = "0.21.2"
notify = "6.1.1"
uuid = { version = "1.10.0", features = ["v4"] }
thiserror = "1.0.65"
tracing = "0.1.40"
tracing-subscriber = "0.3.18"
```

## Runtime Flow Architecture

### Processing Steps (Strict Order)
1. Server receives chat text
2. Core parses → validates → dispatches event  
3. Crux state machine determines next action
4. Event bus calls correct plugin
5. Plugin runs safely in Wasmtime sandbox
6. Plugin consumes Crux capabilities (HTTP, KV, etc.)
7. Result returned → Core triggers render → Server pushes to UI

### State Management Rules
- All state changes go through Crux update function
- Use predictable state flow like Redux/Elm
- Never bypass Crux state machine for business logic
- All plugin side-effects go through Crux effects for auditability

## Plugin Development Standards

### Plugin Implementation Requirements
- Compile to WASM target: `wasm32-wasip2`
- Implement WIT interface defined in `core/wit/core.wit`
- Use wit-bindgen for generating plugin bindings
- Access host capabilities only through declared manifest capabilities

### Plugin Isolation Rules
- Plugins run in Wasmtime sandbox
- No direct file system access
- No direct network access except through `http` capability
- Plugin errors must not crash core system

## Event System Requirements

### Event Bus Implementation
- Implement publish/subscribe pattern through Crux
- Support retries with exponential backoff
- Include Dead Letter Queue (DLQ) for failed events
- Add metrics hooks for observability

### Event Flow Rules
- All inter-plugin communication via events
- Event routing determined by plugin manifest
- Support multi-plugin workflows (e.g., create invoice → update books)
- Events include trace_id for debugging

## Schema and Validation

### Entity Schema Requirements
- All entities must have corresponding JSON schemas
- Validate all input data against schemas before processing
- Use schema versioning for evolution
- Cache schemas per plugin for performance

### Tokenizer Integration
- Parse chat commands using nom combinators
- Support natural language with tokenizers crate
- Extract entities and metadata from chat text
- Produce structured commands for plugin execution

## Development Workflow

### Hot Reload Support
- Use notify to watch plugin directory for changes
- Automatically reload plugins on WASM file changes
- Preserve system state during plugin reloads
- Provide clear reload success/failure feedback

### Plugin Development
- Build plugins: `cargo build --target wasm32-wasip2 --release`
- Place WASM + manifest in `server/plugins/`
- Server auto-detects and loads plugins
- Support plugin versioning and dependencies

## Error Handling and Observability

### Error Isolation
- Plugin errors must be caught and logged
- Provide fallback behavior when plugins fail
- Use Wasmtime's trap handling for safety
- Never allow plugin errors to crash core

### Monitoring Requirements
- Add structured logging with tracing
- Include plugin_id and event context in logs
- Implement metrics for events, failures, retries
- Support debugging through trace_id propagation

## Security and Configuration

### Plugin Security
- Plugins access config via host function `get_config(key)`
- Secrets via `get_secret(key)` from secure backend
- No direct filesystem access to secrets
- Audit all plugin capability usage

### Capability Access
- Only capabilities declared in manifest are available
- Use standard capability names: `http`, `kv`, `render`, `pub-sub`
- Plugins cannot create custom capability interfaces
- All capability calls go through Crux