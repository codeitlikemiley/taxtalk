# Dependencies & Architecture Rules

## Project Structure Requirements

### Layer Architecture
- **MUST** maintain three distinct layers:
  - `core/` - Shared runtime with plugin loader, schemas, event bus, tokenizer, validation
  - `plugins/*/` - Each plugin as separate WASM crate (e.g., `invoice`, `company`)  
  - `server/` - HTTP server and dynamic plugin host
- **MUST NOT** create circular dependencies between layers
- Plugins **MUST ONLY** depend on `wit-bindgen` and core serialization crates
- Server **MUST** import core via `core = { path = "../core" }`

### Cargo.toml Structure
- Each plugin **MUST** have `crate-type = ["cdylib"]` for WASM builds
- **MUST** use workspace structure with proper path dependencies
- **MUST NOT** duplicate dependency versions across crates without justification

## Core Layer Dependencies

### Required Core Crates
When working with the `core/` layer, **ALWAYS** use these exact versions:

```toml
# WASM Runtime - REQUIRED
wasmtime = "36.0.1"

# Serialization - REQUIRED  
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.143"
serde_with = "3.12.0"

# Async runtime - REQUIRED
tokio = { version = "1.47.1", features = ["macros", "rt-multi-thread", "fs", "signal"] }

# Parsing & Tokenization - REQUIRED
nom = "8.0.0" 
tokenizers = "0.21.2"

# Error handling - REQUIRED
thiserror = "2.0"

# Logging - REQUIRED
tracing = "0.1.40"
tracing-subscriber = "0.3.18"

# Utilities - REQUIRED
async-trait = "0.1.81"
uuid = { version = "1.10.0", features = ["v4"] }
notify = "6.1.1"
```

### Optional Core Crates
```toml
# Database persistence - OPTIONAL
sqlx = { version = "0.8.2", features = ["sqlite", "runtime-tokio-rustls", "macros"] }
```

### Core Layer Restrictions
- **MUST NOT** add web server dependencies (axum, tower, etc.) to core
- **MUST NOT** add UI-specific dependencies to core
- **MUST** keep core focused on runtime, schemas, and plugin management
- **ALWAYS** use async-trait for plugin trait definitions

## Server Layer Dependencies

### Required Server Crates
When working with the `server/` layer, **ALWAYS** use these exact versions:

```toml
# Web server - REQUIRED
axum = { version = "0.8.4", features = ["json", "ws", "headers"] }
tower = "0.5.1"

# Async runtime - REQUIRED (must match core version)
tokio = { version = "1.47.1", features = ["macros", "rt-multi-thread"] }

# Serialization - REQUIRED (must match core versions)
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.143"

# Logging - REQUIRED (must match core versions)
tracing = "0.1.40"
tracing-subscriber = "0.3.18"

# Core dependency - REQUIRED
core = { path = "../core" }
```

### Server Layer Restrictions
- **MUST NOT** duplicate core functionality in server
- **MUST** use path dependency to core, not version dependency
- **MUST** maintain version consistency with core for shared crates
- **MUST NOT** add plugin-specific logic directly to server

## Plugin Layer Dependencies

### Required Plugin Crates
For **EVERY** plugin in `plugins/*/`, **ALWAYS** use these exact versions:

```toml
# WASM bindings - REQUIRED
wit-bindgen = "0.44.0"

# Serialization - REQUIRED (must match core versions)
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.143"

# Error handling - REQUIRED
thiserror = "2.0"

# Utilities - REQUIRED (must match core version)
uuid = { version = "1.10.0", features = ["v4"] }
```

### Plugin Cargo.toml Requirements
Every plugin **MUST** include:
```toml
[lib]
crate-type = ["cdylib"]  # Required for WASM builds
```

### Plugin Restrictions
- **MUST NOT** add tokio or async runtime dependencies to plugins
- **MUST NOT** add file system access crates (notify, etc.)
- **MUST NOT** add network or HTTP dependencies
- **MUST NOT** add database dependencies
- **MUST** remain lightweight and focused on business logic only
- **MUST** communicate with host only through wit-bindgen interfaces

## Version Consistency Rules

### Shared Crate Versions
These crates **MUST** use identical versions across all layers:
- `serde` = "1.0.219"
- `serde_json` = "1.0.143"
- `thiserror` = "2.0"
- `uuid` = "1.10.0"
- `tracing` = "0.1.40"
- `tokio` = "1.47.1" (core and server only)

### Version Update Policy
- **MUST** update shared dependency versions in ALL affected crates simultaneously
- **MUST** test compatibility across all layers before version bumps
- **MUST** document breaking changes in dependency updates

## Dependency Addition Guidelines

### Before Adding New Dependencies
- **MUST** justify why existing crates cannot fulfill the requirement
- **MUST** ensure the crate aligns with the layer's responsibilities
- **MUST** verify license compatibility
- **MUST** check for security vulnerabilities
- **MUST** prefer crates from the approved list below

### Approved Additional Crates

#### Core Layer Additions (if needed)
```toml
# Configuration management
config = "0.14.0"

# Time handling
chrono = { version = "0.4.38", features = ["serde"] }

# HTTP client for plugin registry
reqwest = { version = "0.12.9", features = ["json"] }
```

#### Server Layer Additions (if needed)
```toml
# Authentication
jsonwebtoken = "9.3.0"

# Rate limiting
tower-rate-limit = "0.1.0"

# CORS
tower-http = { version = "0.6.1", features = ["cors"] }
```

### Forbidden Dependencies
- **NEVER** add these to any layer:
  - Outdated async runtimes (async-std, smol)
  - Blocking HTTP clients (ureq in core/server)
  - Unsafe WASM runtimes
  - Deprecated serialization crates (rustc-serialize)
  - Logging crates other than tracing family

## Build Configuration

### Cargo Features
- **MUST** use feature flags for optional functionality
- **MUST** keep default features minimal
- **MUST** document all custom features

### WASM Build Requirements
For plugins:
```bash
# MUST use this target for plugin builds
cargo build --target wasm32-wasi --release
```

## Error Handling Standards

### Error Types
- **MUST** use `thiserror` for all error definitions
- **MUST** implement `From` conversions for error propagation
- **MUST** provide meaningful error messages
- **MUST NOT** use `unwrap()` or `expect()` in production code paths

### Example Error Structure
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Invalid command: {command}")]
    InvalidCommand { command: String },
}
```

## Testing Dependencies

### Test-Only Dependencies
For `[dev-dependencies]` only:
```toml
tokio-test = "0.4.4"
tempfile = "3.13.0"
mockall = "0.13.1"
```

## Documentation Requirements

### Dependency Documentation
- **MUST** document why each dependency is needed
- **MUST** document any non-obvious feature flags
- **MUST** keep dependency comments up to date
- **MUST** document version constraints and compatibility requirements

### Example Documentation
```toml
# Plugin runtime - enables dynamic WASM execution with sandboxing
wasmtime = "36.0.1"

# JSON serialization - required for plugin communication protocol  
serde_json = "1.0.143"
```

---

## Enforcement Notes

These rules ensure:
- ✅ Clean separation between layers
- ✅ Latest stable dependency versions across the project
- ✅ WASM compatibility for plugins
- ✅ Maintainable and secure dependency management
- ✅ Future-proof architecture for scaling

**When in doubt, prefer fewer dependencies over more, and always justify additions in PR descriptions.**