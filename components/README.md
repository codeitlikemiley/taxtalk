# TaxTalk UI Component Library

A modular, testable, and isolated Leptos component system for building modern business applications.

## 🏗️ Architecture

This component library follows a workspace-based architecture where each component lives in its own crate for true isolation and independent testing.

```
components/
├── core/               # Shared utilities and types
├── taxtalk-ui/        # Main re-export crate
├── file-upload/       # File upload component
├── dynamic-table/     # Dynamic table component (WIP)
├── conditional-form/  # Conditional form component (WIP)
├── receipt-scanner/   # Receipt scanner component (WIP)
└── showcase/          # Interactive component showcase
```

## 🚀 Quick Start

### Using the Library

Add to your `Cargo.toml`:

```toml
[dependencies]
taxtalk-ui = { path = "../components/taxtalk-ui" }
```

Use in your Leptos app:

```rust
use leptos::prelude::*;
use taxtalk_ui::{FileUpload, FileUploadMode};

#[component]
fn App() -> impl IntoView {
    let mode = Signal::derive(|| FileUploadMode::Single);
    
    view! {
        <FileUpload
            mode=mode
            on_upload=Callback::new(|files| {
                log!("Files uploaded: {:?}", files);
            })
        />
    }
}
```

### Running the Showcase

```bash
cd components/showcase
trunk serve --open
```

This will open an interactive showcase at http://localhost:8080 where you can:
- View all available components
- Interact with live demos
- See code examples
- Review API documentation

## 🧪 Testing Components

### Test Individual Component

```bash
cd components/file-upload
trunk serve  # Interactive testing at http://localhost:8081
cargo test   # Unit tests
```

### Test All Components

```bash
cd components
cargo test --workspace
```

## 📦 Available Components

### ✅ Ready

- **FileUpload** - Drag & drop file upload with validation
  - Single/Multiple file modes
  - File type filtering
  - Size validation
  - Preview support

### 🚧 In Progress

- **DynamicTable** - Data table with computed columns
- **ConditionalForm** - Forms with dynamic field visibility
- **ReceiptScanner** - Philippine receipt OCR scanner

## 🎨 Styling

Components use scoped CSS with prefixes to avoid conflicts:

| Component | CSS Prefix |
|-----------|-----------|
| FileUpload | `tui-fu-` |
| DynamicTable | `tui-dt-` |
| ConditionalForm | `tui-cf-` |
| ReceiptScanner | `tui-rs-` |

Override styles in your app:

```css
.tui-fu-dropzone {
    /* Custom dropzone styles */
}
```

## 🛠️ Development

### Adding a New Component

1. Create component directory:
```bash
cd components
mkdir my-component
cd my-component
```

2. Create `Cargo.toml`:
```toml
[package]
name = "taxtalk-my-component"
version.workspace = true
# ... workspace inheritance
```

3. Implement component in `src/lib.rs`

4. Add to workspace in `components/Cargo.toml`:
```toml
[workspace]
members = [
    # ...
    "my-component"
]
```

5. Re-export in `taxtalk-ui/src/lib.rs`:
```rust
pub mod my_component {
    pub use taxtalk_my_component::*;
}
```

### Building All Components

```bash
cd components
cargo build --workspace
```

### Generate Documentation

```bash
cd components
cargo doc --workspace --no-deps --open
```

## 📁 Component Structure

Each component follows this structure:

```
component-name/
├── Cargo.toml         # Component metadata
├── src/
│   ├── lib.rs        # Public API
│   ├── component.rs  # Main component
│   └── types.rs      # Component types
├── styles/
│   └── component.css # Component styles
├── examples/         # Runnable examples
│   └── basic.rs
├── tests/           # Integration tests
│   └── integration.rs
├── index.html       # For trunk serve
├── Trunk.toml       # Build configuration
└── README.md        # Component documentation
```

## 🔄 CI/CD

Components are tested independently in CI:

- Each component builds separately
- Tests run in isolation
- Examples are compiled
- Documentation is generated

## 📝 License

MIT

## 🤝 Contributing

1. Create a feature branch
2. Implement component in its own crate
3. Add examples and tests
4. Update showcase
5. Submit PR

For more information, see [COMPONENT_LIBRARY_ARCHITECTURE.md](../COMPONENT_LIBRARY_ARCHITECTURE.md)