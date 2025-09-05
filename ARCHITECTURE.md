# Crux-Based Plugin Architecture Guide

## System Overview

Crux serves as the state management and capability layer at the heart of the system. All components interact through Crux, with Wasmtime extending it dynamically through plugins.

## Layer Responsibilities

| Layer | Technology | Purpose |
|-------|------------|---------|
| Core | Crux + Wasmtime | State management, cross-platform rendering, plugin runtime, event bus |
| Server | Axum + Core | HTTP + SSE/WebSocket bridge, chat interface, dynamic plugin loader |
| Plugins | WASM + wit-bindgen | Modular features (accounting, invoicing, CRM, inventory) |

## Crux Foundation

### Core Components
- Shared state model (`App` struct) with `Model`, `View`, and `Effect` logic
- Capabilities system (`http`, `render`, `kv`, `pub/sub`)
- WIT files for cross-platform APIs

### Plugin Integration
- Plugins exposed as Crux capabilities dynamically
- Plugins leverage Crux features without hardcoding
- UI and business logic remain strictly separated

## Project Structure

```
project-root/
├── core/
│   ├── wit/
│   │   ├── core.wit
│   │   └── world.wit
│   ├── src/
│   │   ├── lib.rs             # Crux App state & plugin manager
│   │   ├── plugin_loader.rs   # Wasmtime plugin loader
│   │   ├── manifest.rs        # Plugin manifest + schema
│   │   ├── event_bus.rs       # Event system
│   │   ├── tokenizer.rs       # Parsing via nom + tokenizers
│   │   └── schema.rs          # Validation of entities
│   └── Cargo.toml
├── plugins/
│   ├── invoice/
│   │   ├── src/lib.rs
│   │   └── Cargo.toml
│   └── company/
│       ├── src/lib.rs
│       └── Cargo.toml
└── server/
    ├── src/main.rs
    └── Cargo.toml
```

## WIT Interface Definitions

### core.wit
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

### world.wit
```wit
package crux:shared-lib;

world shared {
    export core;
    import http: func(request: string) -> string;
    import render: func(view: string);
    import kv: func(key: string, value: string);
}
```

## Plugin Architecture

### Plugin Characteristics
- Dynamic capability provider
- Registered in Crux at runtime by Wasmtime loader
- Consumes Crux capabilities (HTTP calls, storage, pub/sub)

### Plugin Examples
- `invoice` plugin imports `http` capability for external tax APIs
- `crm` plugin imports `kv` for customer record persistence
- Core uses `render` for UI updates on state changes

## Plugin Manifest Format

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

## Chat Processing Flow

### Input Processing
1. User input: `@client uriah bought 5kg of pineapple for 1000 pesos`
2. `tokenizer.rs` parses using nom + tokenizers

### Structured Data Output
```json
{
  "command": "create_invoice",
  "client": "uriah",
  "items": [
    { "unit": "kg", "qty": 5, "name": "pineapple", "amount": 1000, "currency": "PHP" }
  ],
  "metadata": {
    "client": { "id": 42, "name": "Uriah", "age": 36 }
  }
}
```

### Event Processing
3. Crux `update()` receives JSON as event
4. Crux routes event through event bus to correct plugin
5. Plugin executes in Wasmtime sandbox
6. Plugin operations:
   - Fetch data via HTTP capability
   - Store state via KV capability
   - Trigger `render` for UI updates

## Core Dependencies

### Primary Crates
| Crate | Purpose |
|-------|---------|
| `crux_core` | Crux shared runtime (state, effect, view) |
| `wasmtime` | Dynamic plugin loading |
| `serde` | Event and schema serialization |
| `serde_json` | JSON payloads for events and chat data |
| `nom` | Chat command parsing into structured tokens |
| `tokenizers` | NLP tokenization for complex phrases |
| `notify` | Hot-reload plugins on file changes |
| `uuid` | Entity identifiers |
| `thiserror` | Structured error handling |
| `tracing` | Logging and debugging |

### Cargo.toml Configuration
```toml
[dependencies]
crux_core = "0.7.0"
wasmtime = "36.0.2"
serde = { version = "1.0.210", features = ["derive"] }
serde_json = "1.0.128"
tokio = { version = "1.40.0", features = ["macros", "rt-multi-thread"] }
nom = "8.0.0"
tokenizers = "0.22.0"
notify = "8.2.0"
uuid = { version = "1.10.0", features = ["v4"] }
thiserror = "1.0.65"
tracing = "0.1.41"
tracing-subscriber = "0.3.20"
```

## Runtime Architecture

```
┌────────────┐
│   Server   │
│   (Axum)   │
└─────┬──────┘
      │
      v
┌────────────┐
│    Core    │
│ (Crux App) │
└─────┬──────┘
      │ Plugins loaded via Wasmtime
      v
┌────────────┐
│  Plugins   │
│   (WASM)   │
└────────────┘
```

### Processing Steps
1. Server receives chat text
2. Core parses, validates, dispatches event
3. Crux state machine determines next action
4. Event bus calls correct plugin
5. Plugin runs in Wasmtime sandbox
6. Plugin consumes Crux capabilities
7. Result returned, Core triggers render, Server pushes to UI

## Architecture Benefits

### Predictable State Flow
- Crux provides Redux/Elm-like state management
- All state changes flow through defined channels

### Plugin Isolation
- Plugins completely decoupled
- No direct inter-plugin communication
- Sandbox execution environment

### Dynamic Extension
- Add features by dropping in WASM files
- No server code changes required
- Hot-reload capability for development

### Clean Separation
- Business logic in Crux core
- Dynamic modules as plugins
- Presentation layer via render capability

## Implementation Phases

### Phase 1: Minimal Core
- Scaffold Crux-based core with dummy plugin
- Basic state management structure
- Simple plugin loading mechanism

### Phase 2: Manifest System
- Extend manifest loading for dynamic capability registration
- Plugin dependency resolution
- Schema validation system

### Phase 3: Chat Integration
- Integrate tokenizer parsing into Crux `update()` function
- Complete event bus implementation
- Natural language processing pipeline