#!/bin/bash

# Component generator script for TaxTalk UI components
# Usage: ./create-component.sh <component-name>

if [ -z "$1" ]; then
    echo "Usage: $0 <component-name>"
    echo "Example: $0 entity-tag"
    exit 1
fi

COMPONENT_NAME=$1
COMPONENT_PATH="components/$COMPONENT_NAME"

# Convert kebab-case to PascalCase for Rust struct names
# Using awk for better portability
PASCAL_NAME=$(echo "$COMPONENT_NAME" | awk -F'-' '{for(i=1;i<=NF;i++) $i=toupper(substr($i,1,1)) substr($i,2)} 1' OFS="")

# Convert to snake_case for module names
SNAKE_NAME=$(echo "$COMPONENT_NAME" | tr '-' '_')

echo "Creating component: $COMPONENT_NAME"
echo "  Pascal case: $PASCAL_NAME"
echo "  Snake case: $SNAKE_NAME"
echo "  Path: $COMPONENT_PATH"

# Create directory structure
mkdir -p "$COMPONENT_PATH/src"
mkdir -p "$COMPONENT_PATH/tests"
mkdir -p "$COMPONENT_PATH/examples"

# Create Cargo.toml
cat > "$COMPONENT_PATH/Cargo.toml" << EOF
[package]
name = "taxtalk-$COMPONENT_NAME"
version.workspace = true
authors.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
leptos = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
console_error_panic_hook = { workspace = true }
wasm-bindgen = { workspace = true }
web-sys = { workspace = true }
uuid = { workspace = true }
chrono = { workspace = true }
taxtalk-ui-core = { workspace = true }

[dev-dependencies]
wasm-bindgen-test = { workspace = true }
pretty_assertions = { workspace = true }

[[bin]]
name = "taxtalk-$COMPONENT_NAME"
path = "src/main.rs"
EOF

# Create README.md
cat > "$COMPONENT_PATH/README.md" << EOF
# $PASCAL_NAME Component

## Overview
The $PASCAL_NAME component provides [brief description].

## Installation
\`\`\`toml
[dependencies]
taxtalk-$COMPONENT_NAME = { path = "../$COMPONENT_NAME" }
\`\`\`

## Basic Usage
\`\`\`rust
use taxtalk_${SNAKE_NAME}::$PASCAL_NAME;
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <$PASCAL_NAME
            // props here
        />
    }
}
\`\`\`

## Props
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| value | Signal<String> | Required | The component value |
| on_change | Callback<String> | Required | Called when value changes |

## Events
- \`on_change\`: Fired when the value changes
- \`on_submit\`: Fired on form submission

## Styling
The component uses CSS classes with \`tui-${COMPONENT_NAME}-\` prefix.

## Examples
See \`examples/\` directory for more usage patterns.

## Testing
\`\`\`bash
cargo test
cargo test --target wasm32-unknown-unknown
\`\`\`
EOF

# Create index.html
cat > "$COMPONENT_PATH/index.html" << EOF
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>$PASCAL_NAME Component Demo</title>
    <link data-trunk rel="css" href="/src/styles.css">
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }
        
        .demo-container {
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            border-radius: 12px;
            padding: 32px;
            box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
        }
        
        h1 {
            color: #1a202c;
            margin-top: 0;
            margin-bottom: 8px;
        }
        
        .subtitle {
            color: #718096;
            margin-bottom: 32px;
        }
        
        .demo-section {
            margin-bottom: 32px;
            padding: 24px;
            background: #f7fafc;
            border-radius: 8px;
        }
        
        h2 {
            color: #2d3748;
            margin-top: 0;
            margin-bottom: 16px;
            font-size: 1.25rem;
        }
    </style>
</head>
<body></body>
</html>
EOF

# Create Trunk.toml
cat > "$COMPONENT_PATH/Trunk.toml" << EOF
[build]
target = "index.html"
dist = "dist"

[serve]
address = "127.0.0.1"
port = 8080
open = false

[watch]
watch = ["src", "index.html", "Cargo.toml"]
EOF

# Create lib.rs
cat > "$COMPONENT_PATH/src/lib.rs" << EOF
pub mod component;
pub mod types;
pub mod state;
pub mod effects;

pub use component::$PASCAL_NAME;
pub use types::*;
pub use state::*;
EOF

# Create component.rs
cat > "$COMPONENT_PATH/src/component.rs" << EOF
use leptos::prelude::*;
use crate::types::*;
use crate::state::*;

/// The $PASCAL_NAME component
#[component]
pub fn $PASCAL_NAME(
    /// The current value
    #[prop(into)] value: Signal<String>,
    
    /// Callback when value changes
    on_change: Callback<String, ()>,
    
    /// Optional CSS class
    #[prop(optional)] class: Option<String>,
    
    /// Whether the component is disabled
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    // Component state management
    let (local_state, set_local_state) = signal(ComponentState::default());
    
    // Sync with external value
    Effect::new(move |_| {
        let new_value = value.get();
        set_local_state.update(|state| {
            state.value = new_value.clone();
        });
    });
    
    // Handle changes
    let handle_change = move |new_value: String| {
        set_local_state.update(|state| {
            state.value = new_value.clone();
        });
        on_change.run(new_value);
    };
    
    let css_class = format!(
        "tui-$COMPONENT_NAME {}",
        class.unwrap_or_default()
    );
    
    view! {
        <div class=css_class>
            <div class="tui-$COMPONENT_NAME-content">
                {move || local_state.get().value.clone()}
            </div>
            
            <Show
                when=move || !disabled
                fallback=|| ()
            >
                <button
                    class="tui-$COMPONENT_NAME-button"
                    on:click=move |_| handle_change("clicked".to_string())
                >
                    "Click Me"
                </button>
            </Show>
        </div>
    }
}
EOF

# Create types.rs
cat > "$COMPONENT_PATH/src/types.rs" << EOF
use serde::{Deserialize, Serialize};

/// Component-specific types
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ComponentData {
    pub value: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ComponentEvent {
    ValueChanged(String),
    Submitted,
    Cancelled,
}
EOF

# Create state.rs (with Crux integration)
cat > "$COMPONENT_PATH/src/state.rs" << EOF
use serde::{Deserialize, Serialize};
use crate::types::*;

/// Component state management
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ComponentState {
    pub value: String,
    pub loading: bool,
    pub error: Option<String>,
    pub data: Option<ComponentData>,
}

/// Crux App integration (placeholder for now)
#[derive(Clone, Debug)]
pub struct ComponentApp;

// Note: Full Crux integration will be implemented when Crux dependency is available
// impl crux::App for ComponentApp {
//     type Model = ComponentState;
//     type Event = ComponentEvent;
//     type ViewModel = ComponentViewModel;
//     type Capabilities = ();
//     
//     fn update(&self, event: Self::Event, model: &mut Self::Model, _caps: &()) {
//         match event {
//             ComponentEvent::ValueChanged(value) => {
//                 model.value = value;
//             }
//             ComponentEvent::Submitted => {
//                 // Handle submission
//             }
//             ComponentEvent::Cancelled => {
//                 // Handle cancellation
//             }
//         }
//     }
//     
//     fn view(&self, model: &Self::Model) -> Self::ViewModel {
//         ComponentViewModel {
//             value: model.value.clone(),
//             loading: model.loading,
//             error: model.error.clone(),
//         }
//     }
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentViewModel {
    pub value: String,
    pub loading: bool,
    pub error: Option<String>,
}
EOF

# Create effects.rs
cat > "$COMPONENT_PATH/src/effects.rs" << EOF
use leptos::prelude::*;

/// Side effects and async operations for the component
pub struct ComponentEffects;

impl ComponentEffects {
    /// Load data asynchronously
    pub async fn load_data(query: String) -> Result<Vec<String>, String> {
        // Simulate API call
        Ok(vec![
            format!("Result 1 for {}", query),
            format!("Result 2 for {}", query),
        ])
    }
    
    /// Validate input
    pub fn validate(value: &str) -> Result<(), String> {
        if value.is_empty() {
            Err("Value cannot be empty".to_string())
        } else {
            Ok(())
        }
    }
}
EOF

# Create styles.css
cat > "$COMPONENT_PATH/src/styles.css" << EOF
/* Component-specific styles with prefix */
.tui-$COMPONENT_NAME {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
    border: 1px solid #e2e8f0;
    border-radius: 0.5rem;
    background: white;
}

.tui-$COMPONENT_NAME-content {
    padding: 0.75rem;
    background: #f7fafc;
    border-radius: 0.375rem;
    font-size: 1rem;
    color: #2d3748;
}

.tui-$COMPONENT_NAME-button {
    padding: 0.5rem 1rem;
    background: #4299e1;
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
}

.tui-$COMPONENT_NAME-button:hover {
    background: #3182ce;
}

.tui-$COMPONENT_NAME-button:disabled {
    background: #cbd5e0;
    cursor: not-allowed;
}
EOF

# Create main.rs (demo app)
cat > "$COMPONENT_PATH/src/main.rs" << EOF
use leptos::prelude::*;
use taxtalk_${SNAKE_NAME}::$PASCAL_NAME;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (value, set_value) = signal("Initial value".to_string());
    let (log, set_log) = signal(Vec::<String>::new());
    
    let handle_change = move |new_value: String| {
        set_value.set(new_value.clone());
        set_log.update(|l| l.push(format!("Changed to: {}", new_value)));
    };
    
    view! {
        <div class="demo-container">
            <h1>"$PASCAL_NAME Component Demo"</h1>
            <p class="subtitle">"Interactive demonstration of the $PASCAL_NAME component"</p>
            
            <div class="demo-section">
                <h2>"Basic Usage"</h2>
                <$PASCAL_NAME
                    value=Signal::derive(move || value.get())
                    on_change=Callback::new(handle_change)
                />
            </div>
            
            <div class="demo-section">
                <h2>"Disabled State"</h2>
                <$PASCAL_NAME
                    value=Signal::derive(move || "Disabled component".to_string())
                    on_change=Callback::new(|_| {})
                    disabled=true
                />
            </div>
            
            <div class="demo-section">
                <h2>"Event Log"</h2>
                <div style="font-family: monospace; font-size: 0.875rem;">
                    <For
                        each=move || log.get()
                        key=|entry| entry.clone()
                        children=move |entry| {
                            view! {
                                <div style="padding: 4px; background: #f0f0f0; margin: 2px 0;">
                                    {entry}
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}
EOF

# Create unit test file
cat > "$COMPONENT_PATH/tests/unit.rs" << EOF
#[cfg(test)]
mod tests {
    use taxtalk_${SNAKE_NAME}::*;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn component_renders() {
        // Test that component renders without panicking
        let _result = leptos::ssr::render_to_string(|| {
            view! {
                <$PASCAL_NAME
                    value=Signal::derive(|| "test".to_string())
                    on_change=Callback::new(|_| {})
                />
            }
        });
    }
    
    #[test]
    fn test_component_state() {
        let mut state = ComponentState::default();
        assert_eq!(state.value, "");
        
        state.value = "new value".to_string();
        assert_eq!(state.value, "new value");
    }
    
    #[test]
    fn test_component_data() {
        let data = ComponentData {
            value: "test".to_string(),
            metadata: None,
        };
        
        assert_eq!(data.value, "test");
        assert!(data.metadata.is_none());
    }
}
EOF

# Create integration test
cat > "$COMPONENT_PATH/tests/integration.rs" << EOF
#[cfg(test)]
mod integration_tests {
    use taxtalk_${SNAKE_NAME}::*;
    
    #[test]
    fn test_component_integration() {
        // Integration tests will be added as needed
        assert!(true);
    }
}
EOF

# Create basic example
cat > "$COMPONENT_PATH/examples/basic.rs" << EOF
use leptos::prelude::*;
use taxtalk_${SNAKE_NAME}::$PASCAL_NAME;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(BasicExample);
}

#[component]
fn BasicExample() -> impl IntoView {
    let (value, set_value) = signal("Hello World".to_string());
    
    view! {
        <div style="padding: 20px;">
            <h1>"Basic $PASCAL_NAME Example"</h1>
            
            <$PASCAL_NAME
                value=Signal::derive(move || value.get())
                on_change=Callback::new(move |v| set_value.set(v))
            />
            
            <p>"Current value: " {move || value.get()}</p>
        </div>
    }
}
EOF

echo "✅ Component '$COMPONENT_NAME' created successfully!"
echo ""
echo "Next steps:"
echo "1. cd $COMPONENT_PATH"
echo "2. cargo build"
echo "3. trunk serve"
echo ""
echo "The component will be available at http://localhost:8080"