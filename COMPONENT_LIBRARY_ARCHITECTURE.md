# Component Library Architecture Plan
## Modular, Testable, and Isolated Leptos Component System

### Executive Summary
This document outlines a comprehensive plan to restructure our Leptos components into a proper component library with isolated testing, following best practices from successful Leptos component libraries like Thaw UI, Leptonic, and dioxus-material.

---

## 🎯 Goals & Objectives

1. **Component Isolation**: Each component as its own crate for independent development and testing
2. **Storybook-like Testing**: Individual component demos with interactive props
3. **CSS Scoping**: Tailwind CSS properly scoped per component
4. **Version Consistency**: Maintain same Leptos, WASM, and Tailwind versions across all crates
5. **Documentation**: Auto-generated component documentation with examples
6. **CI/CD Ready**: Each component can be tested and built independently

---

## 📚 Research: How Leptos Component Libraries Do It

### 1. **Thaw UI** (Most Popular Leptos Component Library)
- Uses workspace structure with individual component crates
- Has a `thaw_components` parent crate that re-exports all components
- Each component has its own `examples/` folder
- Uses `demo-site` crate for showcasing all components
- CSS is handled via `stylers` macro for scoped styles

### 2. **Leptonic** 
- Monorepo with workspace members
- `leptonic` core crate + `leptonic-theme` for styling
- Uses `book` crate for documentation site
- Components are feature-gated for selective inclusion
- CSS uses SCSS with component-specific stylesheets

### 3. **Material Yew/Leptos Patterns**
- Component per module approach
- Shared `utils` crate for common functionality
- `playground` app for testing components
- Uses CSS-in-Rust approach with `styled` macro

### Best Practices Identified:
- Workspace-based architecture
- Shared utilities crate
- Demo/playground application
- Component re-export crate for easy consumption
- Consistent prop patterns using Leptos's `#[prop]` macro

---

## 🏗️ Proposed Architecture

```
taxtalk/
├── Cargo.toml (workspace root)
├── components/              # Component library workspace
│   ├── Cargo.toml          # Workspace definition
│   ├── README.md
│   │
│   ├── taxtalk-ui/         # Main re-export crate
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   └── lib.rs      # Re-exports all components
│   │   └── README.md
│   │
│   ├── core/               # Shared utilities and types
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── types.rs    # Common types (ComponentType, etc.)
│   │   │   ├── theme.rs    # Theme configuration
│   │   │   ├── icons.rs    # Icon system
│   │   │   └── utils.rs    # Shared utilities
│   │   └── styles/
│   │       └── base.css    # Base Tailwind setup
│   │
│   ├── file-upload/        # FileUpload Component
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── component.rs
│   │   │   └── types.rs
│   │   ├── styles/
│   │   │   └── file-upload.css
│   │   ├── examples/
│   │   │   ├── basic.rs
│   │   │   ├── multi.rs
│   │   │   └── receipt.rs
│   │   ├── tests/
│   │   │   └── integration.rs
│   │   ├── index.html      # For trunk serve
│   │   ├── Trunk.toml
│   │   └── README.md
│   │
│   ├── dynamic-table/      # DynamicTable Component
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── component.rs
│   │   │   ├── cell.rs
│   │   │   ├── row.rs
│   │   │   └── types.rs
│   │   ├── styles/
│   │   │   └── table.css
│   │   ├── examples/
│   │   │   ├── basic.rs
│   │   │   ├── invoice.rs
│   │   │   └── editable.rs
│   │   ├── tests/
│   │   ├── index.html
│   │   ├── Trunk.toml
│   │   └── README.md
│   │
│   ├── conditional-form/   # ConditionalForm Component
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── component.rs
│   │   │   ├── field.rs
│   │   │   ├── condition.rs
│   │   │   └── types.rs
│   │   ├── styles/
│   │   │   └── form.css
│   │   ├── examples/
│   │   │   ├── payment.rs
│   │   │   ├── wizard.rs
│   │   │   └── validation.rs
│   │   ├── tests/
│   │   ├── index.html
│   │   ├── Trunk.toml
│   │   └── README.md
│   │
│   ├── receipt-scanner/    # ReceiptScanner Component
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── component.rs
│   │   │   ├── ocr.rs
│   │   │   ├── patterns.rs
│   │   │   └── types.rs
│   │   ├── styles/
│   │   │   └── scanner.css
│   │   ├── examples/
│   │   │   ├── basic.rs
│   │   │   ├── philippine.rs
│   │   │   └── mock.rs
│   │   ├── tests/
│   │   ├── index.html
│   │   ├── Trunk.toml
│   │   └── README.md
│   │
│   ├── smart-input/        # Smart Command Input Components
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── command_input.rs
│   │   │   ├── entity_selector.rs
│   │   │   ├── token_parser.rs
│   │   │   └── types.rs
│   │   ├── styles/
│   │   │   └── smart-input.css
│   │   ├── examples/
│   │   ├── tests/
│   │   ├── index.html
│   │   ├── Trunk.toml
│   │   └── README.md
│   │
│   ├── entity-combobox/    # Entity Selection Components
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── single.rs
│   │   │   ├── multi.rs
│   │   │   └── types.rs
│   │   ├── styles/
│   │   ├── examples/
│   │   ├── index.html
│   │   ├── Trunk.toml
│   │   └── README.md
│   │
│   ├── showcase/           # Component showcase/playground
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── app.rs
│   │   │   ├── router.rs
│   │   │   └── pages/
│   │   │       ├── mod.rs
│   │   │       ├── file_upload.rs
│   │   │       ├── table.rs
│   │   │       ├── form.rs
│   │   │       └── scanner.rs
│   │   ├── public/
│   │   ├── styles/
│   │   │   └── showcase.css
│   │   ├── index.html
│   │   ├── Trunk.toml
│   │   └── README.md
│   │
│   └── docs/               # Documentation generator
│       ├── Cargo.toml
│       ├── src/
│       │   └── main.rs     # Generates component docs
│       └── templates/
│           └── component.md
│
├── web/                    # Main application (uses components)
│   ├── Cargo.toml         # Depends on taxtalk-ui
│   └── src/
│       └── main.rs
```

---

## 📦 Workspace Configuration

### Root Workspace (`/components/Cargo.toml`)
```toml
[workspace]
members = [
    "core",
    "taxtalk-ui",
    "file-upload",
    "dynamic-table",
    "conditional-form",
    "receipt-scanner",
    "smart-input",
    "entity-combobox",
    "showcase",
    "docs"
]

[workspace.package]
version = "0.1.0"
authors = ["TaxTalk Team"]
edition = "2021"
license = "MIT"

[workspace.dependencies]
leptos = { version = "0.7.0", features = ["csr", "nightly"] }
leptos_meta = "0.7.0"
leptos_router = "0.7.0"
console_error_panic_hook = "0.1"
wasm-bindgen = "0.2"
web-sys = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.10", features = ["v4", "wasm-bindgen"] }
gloo = "0.11"

# Shared version pins
[workspace.dependencies.taxtalk-ui-core]
path = "core"
version = "0.1.0"
```

### Individual Component Crate (`/components/file-upload/Cargo.toml`)
```toml
[package]
name = "taxtalk-file-upload"
version.workspace = true
authors.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
leptos.workspace = true
web-sys = { workspace = true, features = ["File", "DragEvent", "DataTransfer", "FileList", "HtmlInputElement"] }
wasm-bindgen.workspace = true
uuid.workspace = true
serde.workspace = true
serde_json.workspace = true
taxtalk-ui-core.workspace = true

[dev-dependencies]
wasm-bindgen-test = "0.3"
leptos_axum = "0.7.0"

[features]
default = []
hydrate = ["leptos/hydrate"]
ssr = ["leptos/ssr"]

[[example]]
name = "basic"
required-features = ["csr"]

[[example]]
name = "multi"
required-features = ["csr"]
```

---

## 🎨 CSS Strategy

### Option 1: Tailwind with PostCSS (Recommended)
Each component crate has its own `tailwind.config.js`:

```javascript
// components/file-upload/tailwind.config.js
module.exports = {
  content: ["./src/**/*.{rs,html}"],
  prefix: 'tui-fu-', // Component-specific prefix
  theme: {
    extend: {
      // Component-specific extensions
    }
  },
  plugins: [],
}
```

Build process:
```bash
# In each component directory
npx tailwindcss -i ./styles/input.css -o ./styles/output.css --watch
```

### Option 2: CSS Modules Pattern
```rust
// In component
use taxtalk_ui_core::style;

#[component]
pub fn FileUpload() -> impl IntoView {
    let class = style::scoped("file-upload", include_str!("../styles/file-upload.css"));
    
    view! {
        <div class=class("container")>
            // ...
        </div>
    }
}
```

### Option 3: Styled Components (Using stylers crate)
```rust
use stylers::style;

#[component]
pub fn FileUpload() -> impl IntoView {
    let css = style! {
        .container {
            border: 2px dashed #ccc;
            padding: 2rem;
        }
    };
    
    view! {
        <div class=css.container>
            // ...
        </div>
    }
}
```

---

## 🧪 Testing Strategy

### 1. Component-Level Testing
Each component has its own test suite:

```rust
// components/file-upload/tests/integration.rs
use wasm_bindgen_test::*;
use taxtalk_file_upload::FileUpload;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_single_file_upload() {
    // Test implementation
}

#[wasm_bindgen_test]
fn test_drag_and_drop() {
    // Test implementation
}
```

### 2. Example-Based Development
Each component has runnable examples:

```rust
// components/file-upload/examples/basic.rs
use leptos::prelude::*;
use taxtalk_file_upload::{FileUpload, FileUploadMode};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (mode, set_mode) = signal(FileUploadMode::Single);
    
    view! {
        <div class="p-8">
            <h1>"File Upload Example"</h1>
            <FileUpload
                mode=mode
                on_upload=|files| {
                    log!("Uploaded: {:?}", files);
                }
            />
        </div>
    }
}
```

Run individual component:
```bash
cd components/file-upload
trunk serve --open
```

### 3. Visual Testing with Showcase
The showcase app provides:
- Interactive component gallery
- Props playground
- Code examples
- API documentation

---

## 📁 File Migration Plan

### Phase 1: Core Setup
1. Create workspace structure
2. Setup shared core utilities
3. Configure build tools

### Phase 2: Component Migration
Move existing components to new structure:

| Current Location | New Location | Priority |
|-----------------|--------------|----------|
| `web/src/components/file_upload.rs` | `components/file-upload/` | P0 |
| `web/src/components/dynamic_table.rs` | `components/dynamic-table/` | P0 |
| `web/src/components/conditional_form.rs` | `components/conditional-form/` | P0 |
| `web/src/components/receipt_scanner.rs` | `components/receipt-scanner/` | P0 |
| `web/src/components/smart_command_input.rs` | `components/smart-input/` | P1 |
| `web/src/components/entity_combobox.rs` | `components/entity-combobox/` | P1 |
| `web/src/components/multi_select_combobox.rs` | `components/entity-combobox/` | P1 |
| `web/src/components/chat.rs` | `components/chat/` | P2 |
| `web/src/components/guided_chat.rs` | `components/chat/` | P2 |

### Phase 3: Integration
1. Update main `web` app to use `taxtalk-ui` crate
2. Remove old component files
3. Update imports

---

## 🚀 Build & Development Scripts

### Root Scripts (`/components/package.json`)
```json
{
  "scripts": {
    "dev": "npm-run-all --parallel dev:*",
    "dev:file-upload": "cd file-upload && trunk serve",
    "dev:table": "cd dynamic-table && trunk serve",
    "dev:showcase": "cd showcase && trunk serve --open",
    "build": "npm-run-all build:*",
    "build:components": "cargo build --workspace",
    "build:styles": "npm-run-all build:styles:*",
    "build:styles:file-upload": "cd file-upload && npm run build:css",
    "test": "cargo test --workspace",
    "test:wasm": "wasm-pack test --headless --firefox",
    "lint": "cargo clippy --workspace",
    "doc": "cargo doc --workspace --no-deps --open"
  }
}
```

### Component Build (`/components/file-upload/build.sh`)
```bash
#!/bin/bash
# Build component CSS
npx tailwindcss -i ./styles/input.css -o ./styles/output.css --minify

# Build WASM
wasm-pack build --target web --out-dir pkg

# Run tests
wasm-pack test --headless --firefox
```

---

## 📚 Documentation Strategy

### 1. README per Component
Each component has comprehensive documentation:
```markdown
# FileUpload Component

## Installation
```toml
taxtalk-file-upload = "0.1.0"
```

## Usage
```rust
use taxtalk_file_upload::{FileUpload, FileUploadMode};

#[component]
fn MyApp() -> impl IntoView {
    // ...
}
```

## Props
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| mode | Signal<FileUploadMode> | Single | Upload mode |
| on_upload | Callback<Vec<UploadedFile>> | - | Upload handler |

## Examples
See `examples/` directory for runnable examples.
```

### 2. API Documentation
Generated using `cargo doc`:
```bash
cd components
cargo doc --workspace --no-deps --open
```

### 3. Interactive Showcase
Live component playground with:
- Props editor
- Live preview
- Code examples
- Copy-paste snippets

---

## 🔄 CI/CD Pipeline

### GitHub Actions Workflow
```yaml
name: Component Library CI

on:
  push:
    paths:
      - 'components/**'
  pull_request:
    paths:
      - 'components/**'

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        component:
          - file-upload
          - dynamic-table
          - conditional-form
          - receipt-scanner
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly
          target: wasm32-unknown-unknown
      
      - name: Test Component
        run: |
          cd components/${{ matrix.component }}
          cargo test
          wasm-pack test --headless --firefox
      
      - name: Build Component
        run: |
          cd components/${{ matrix.component }}
          trunk build --release
```

---

## 🎯 Success Metrics

1. **Development Speed**: 50% faster component development
2. **Test Coverage**: 80%+ per component
3. **Build Time**: < 30s per component
4. **Bundle Size**: < 50KB per component (gzipped)
5. **Documentation**: 100% API coverage

---

## 📅 Implementation Timeline

### Week 1: Foundation
- [ ] Setup workspace structure
- [ ] Create core utilities crate
- [ ] Configure build tools
- [ ] Setup CSS pipeline

### Week 2: Component Migration
- [ ] Migrate FileUpload component
- [ ] Migrate DynamicTable component
- [ ] Create first examples
- [ ] Setup testing framework

### Week 3: Complete Migration
- [ ] Migrate remaining P0 components
- [ ] Build showcase app
- [ ] Write documentation
- [ ] Setup CI/CD

### Week 4: Polish & Release
- [ ] Performance optimization
- [ ] Bundle size optimization
- [ ] Documentation review
- [ ] Internal release

---

## 🔗 References

- [Thaw UI](https://github.com/thaw-ui/thaw) - Best practices for Leptos components
- [Leptonic](https://github.com/lpotthast/leptonic) - Workspace structure
- [Yew Component Patterns](https://github.com/yewstack/yew) - Component architecture
- [Trunk Documentation](https://trunkrs.dev/) - Build configuration
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) - Testing and packaging

---

## 💡 Key Decisions

1. **Workspace over Monorepo**: Better isolation and testing
2. **Tailwind with Prefixes**: Avoids CSS conflicts
3. **Trunk for Each Component**: Independent development
4. **Shared Core**: Reduces duplication
5. **Example-Driven**: Better documentation and testing

---

## 🚦 Next Steps

1. **Review this plan** with the team
2. **Choose CSS strategy** (Tailwind prefix vs CSS-in-Rust)
3. **Prioritize components** for migration
4. **Set up initial workspace** structure
5. **Migrate first component** as proof of concept

This architecture will give us:
- ✅ True component isolation
- ✅ Independent testing
- ✅ Scoped CSS
- ✅ Better documentation
- ✅ Faster development
- ✅ Easier maintenance
- ✅ Professional component library structure