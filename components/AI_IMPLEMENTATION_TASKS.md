# AI-Optimized Component Extraction Implementation Tasks

## Overview
This document provides step-by-step tasks for AI agents to implement the component extraction plan from the Enhanced PRD. Each task includes clear objectives, success criteria, and specific implementation details.

## Phase 1: Foundation Infrastructure (Weeks 1-2)

### Week 1: Core Infrastructure Setup

#### Task 1.1: Configure Cargo Workspace
**Objective**: Set up the workspace structure for component isolation

**Actions**:
```bash
# Update /components/Cargo.toml
[workspace]
resolver = "2"
members = [
    "core",
    "entity-tag",
    "token-autocomplete",
    "entity-combobox",
    "simple-form",
    "mixed-input",
    "smart-suggestions",
    "quick-actions",
    "command-palette",
    "inline-confirmation",
    # ... add all 27 components
]

[workspace.dependencies]
leptos = { version = "0.7.0", features = ["csr", "nightly"] }
crux = { version = "0.1.0", path = "../../core" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**Success Criteria**:
- ✅ Workspace compiles without errors
- ✅ All dependencies are properly shared
- ✅ Version consistency maintained

#### Task 1.2: Create Component Template
**Objective**: Establish standardized component structure

**Template Structure**:
```
components/{component-name}/
├── Cargo.toml
├── README.md
├── index.html
├── Trunk.toml
├── src/
│   ├── lib.rs       # Component export with Crux integration
│   ├── main.rs      # Standalone demo app
│   ├── component.rs # Main component implementation
│   ├── types.rs     # Component-specific types
│   ├── state.rs     # Crux state management
│   ├── effects.rs   # Side effects and API calls
│   └── styles.css   # Scoped CSS with prefix
├── tests/
│   ├── unit.rs      # Unit tests
│   ├── integration.rs # Integration tests
│   └── performance.rs # Performance benchmarks
└── examples/
    ├── basic.rs     # Basic usage
    └── advanced.rs  # Advanced patterns
```

**Create template generator script**:
```rust
// tools/create-component.rs
use std::fs;
use std::path::Path;

fn create_component(name: &str) {
    let component_path = format!("components/{}", name);
    
    // Create directory structure
    fs::create_dir_all(&format!("{}/src", component_path))?;
    fs::create_dir_all(&format!("{}/tests", component_path))?;
    fs::create_dir_all(&format!("{}/examples", component_path))?;
    
    // Generate Cargo.toml
    let cargo_toml = format!(r#"
[package]
name = "taxtalk-{}"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = {{ workspace = true }}
crux = {{ workspace = true }}
serde = {{ workspace = true }}
serde_json = {{ workspace = true }}
taxtalk-ui-core = {{ path = "../core" }}

[[bin]]
name = "taxtalk-{}"
path = "src/main.rs"
"#, name, name);
    
    fs::write(format!("{}/Cargo.toml", component_path), cargo_toml)?;
    
    // Generate component files...
}
```

#### Task 1.3: Setup Testing Framework
**Objective**: Establish comprehensive testing infrastructure

**Test Configuration**:
```rust
// components/{component}/tests/unit.rs
#[cfg(test)]
mod tests {
    use super::*;
    use leptos_test::*;
    use pretty_assertions::assert_eq;
    
    #[test]
    fn component_renders_correctly() {
        let result = render_component(|| view! { 
            <MyComponent value=Signal::derive(|| "test".to_string()) />
        });
        
        assert!(result.contains("expected-class"));
        assert!(result.contains("test"));
    }
    
    #[test]
    fn handles_crux_state_updates() {
        let app = ComponentApp::default();
        let mut model = ComponentState::default();
        
        let cmd = app.update(ComponentEvent::UserInput("test"), &mut model, &());
        
        assert_eq!(model.value, "test");
        assert!(cmd.has_effects());
    }
}
```

**Performance Benchmarks**:
```rust
// components/{component}/tests/performance.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_render(c: &mut Criterion) {
    c.bench_function("component_render", |b| {
        b.iter(|| {
            render_component(|| view! { 
                <MyComponent value=black_box("test") />
            })
        });
    });
}

criterion_group!(benches, benchmark_render);
criterion_main!(benches);
```

#### Task 1.4: Setup CI/CD Pipeline
**Objective**: Automate testing and deployment

**GitHub Actions Workflow**:
```yaml
# .github/workflows/component-ci.yml
name: Component CI

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
        component: [entity-tag, token-autocomplete, entity-combobox]
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly
          override: true
      
      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Run tests
        run: |
          cd components/${{ matrix.component }}
          cargo test --all-features
      
      - name: Check formatting
        run: cargo fmt -- --check
      
      - name: Run clippy
        run: cargo clippy -- -D warnings
      
      - name: Build WASM
        run: |
          cd components/${{ matrix.component }}
          trunk build --release
      
      - name: Measure bundle size
        run: |
          SIZE=$(du -b dist/*.wasm | cut -f1)
          echo "Bundle size: $SIZE bytes"
          if [ $SIZE -gt 51200 ]; then
            echo "Bundle size exceeds 50KB limit!"
            exit 1
          fi
```

### Week 2: Core Component Extraction

#### Task 2.1: Extract EntityTag Component
**Objective**: Create the simplest component with full Crux integration

**Implementation Steps**:

1. **Create component structure**:
```rust
// components/entity-tag/src/types.rs
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityToken {
    pub id: String,
    pub entity_type: String,
    pub display_name: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EntityType {
    Client,
    Supplier,
    Product,
    Employee,
    Invoice,
    Payment,
}
```

2. **Implement Crux state management**:
```rust
// components/entity-tag/src/state.rs
use crux::prelude::*;

#[derive(Default, Clone, Debug)]
pub struct EntityTagState {
    pub token: Option<EntityToken>,
    pub is_removing: bool,
}

#[derive(Clone, Debug)]
pub enum EntityTagEvent {
    SetToken(EntityToken),
    Remove,
    RemovalConfirmed,
}

#[derive(Clone, Debug)]
pub struct EntityTagApp;

impl App for EntityTagApp {
    type Model = EntityTagState;
    type Event = EntityTagEvent;
    type ViewModel = EntityTagViewModel;
    type Capabilities = ();
    
    fn update(&self, event: Self::Event, model: &mut Self::Model, _caps: &()) {
        match event {
            EntityTagEvent::SetToken(token) => {
                model.token = Some(token);
            }
            EntityTagEvent::Remove => {
                model.is_removing = true;
            }
            EntityTagEvent::RemovalConfirmed => {
                model.token = None;
                model.is_removing = false;
            }
        }
    }
    
    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        EntityTagViewModel {
            token: model.token.clone(),
            is_removing: model.is_removing,
        }
    }
}
```

3. **Create Leptos component**:
```rust
// components/entity-tag/src/component.rs
use leptos::prelude::*;
use crate::types::*;
use crate::state::*;

#[component]
pub fn EntityTag(
    token: Signal<Option<EntityToken>>,
    on_remove: Callback<String, ()>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    // Connect to Crux state
    let (state, set_state) = signal(EntityTagState::default());
    let app = EntityTagApp;
    
    // Sync token changes
    Effect::new(move |_| {
        if let Some(token) = token.get() {
            let mut current_state = state.get();
            app.update(EntityTagEvent::SetToken(token), &mut current_state, &());
            set_state.set(current_state);
        }
    });
    
    // Determine styling based on entity type
    let get_color_class = move |entity_type: &str| {
        match entity_type {
            "client" => "tui-et-client",
            "supplier" => "tui-et-supplier",
            "product" => "tui-et-product",
            "employee" => "tui-et-employee",
            "invoice" => "tui-et-invoice",
            "payment" => "tui-et-payment",
            _ => "tui-et-default",
        }
    };
    
    view! {
        <Show
            when=move || state.get().token.is_some()
            fallback=|| ()
        >
            {
                let token = state.get().token.unwrap();
                let color_class = get_color_class(&token.entity_type);
                let custom_class = class.clone().unwrap_or_default();
                
                view! {
                    <span class=format!("tui-et-tag {} {}", color_class, custom_class)>
                        <span class="tui-et-label">{token.display_name.clone()}</span>
                        <button
                            class="tui-et-remove"
                            on:click=move |_| {
                                let token_id = token.id.clone();
                                on_remove.run(token_id);
                                
                                let mut current_state = state.get();
                                app.update(EntityTagEvent::Remove, &mut current_state, &());
                                set_state.set(current_state);
                            }
                            disabled=move || state.get().is_removing
                        >
                            "×"
                        </button>
                    </span>
                }
            }
        </Show>
    }
}
```

4. **Add scoped CSS**:
```css
/* components/entity-tag/src/styles.css */
.tui-et-tag {
    display: inline-flex;
    align-items: center;
    padding: 0.25rem 0.5rem;
    border-radius: 9999px;
    font-size: 0.875rem;
    font-weight: 500;
    border: 1px solid;
    transition: all 0.2s;
}

.tui-et-client {
    background-color: #dbeafe;
    color: #1e40af;
    border-color: #93c5fd;
}

.tui-et-supplier {
    background-color: #d1fae5;
    color: #065f46;
    border-color: #86efac;
}

.tui-et-product {
    background-color: #e9d5ff;
    color: #6b21a8;
    border-color: #c084fc;
}

.tui-et-label {
    margin-right: 0.25rem;
}

.tui-et-remove {
    background: none;
    border: none;
    color: currentColor;
    cursor: pointer;
    font-size: 1.25rem;
    line-height: 1;
    opacity: 0.7;
    padding: 0;
    margin-left: 0.25rem;
}

.tui-et-remove:hover {
    opacity: 1;
}

.tui-et-remove:disabled {
    cursor: not-allowed;
    opacity: 0.3;
}
```

5. **Create demo app**:
```rust
// components/entity-tag/src/main.rs
use leptos::prelude::*;
use taxtalk_entity_tag::{EntityTag, EntityToken};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let sample_tokens = vec![
        EntityToken {
            id: "1".to_string(),
            entity_type: "client".to_string(),
            display_name: "ABC Corporation".to_string(),
            metadata: None,
        },
        EntityToken {
            id: "2".to_string(),
            entity_type: "supplier".to_string(),
            display_name: "Tech Solutions Inc".to_string(),
            metadata: None,
        },
        EntityToken {
            id: "3".to_string(),
            entity_type: "product".to_string(),
            display_name: "Widget Pro 2000".to_string(),
            metadata: None,
        },
    ];
    
    let (tokens, set_tokens) = signal(sample_tokens);
    
    view! {
        <div class="demo-container">
            <h1>"EntityTag Component Demo"</h1>
            
            <div class="demo-section">
                <h2>"Active Tags"</h2>
                <div class="tag-container">
                    <For
                        each=move || tokens.get().into_iter().enumerate()
                        key=|(_, token)| token.id.clone()
                        children=move |(index, token)| {
                            let token_signal = Signal::derive(move || Some(token.clone()));
                            
                            view! {
                                <EntityTag
                                    token=token_signal
                                    on_remove=Callback::new(move |id: String| {
                                        let mut current_tokens = tokens.get();
                                        current_tokens.retain(|t| t.id != id);
                                        set_tokens.set(current_tokens);
                                        leptos::logging::log!("Removed token: {}", id);
                                    })
                                />
                            }
                        }
                    />
                </div>
            </div>
            
            <div class="demo-section">
                <h2>"Add New Tag"</h2>
                <button on:click=move |_| {
                    let new_token = EntityToken {
                        id: uuid::Uuid::new_v4().to_string(),
                        entity_type: "invoice".to_string(),
                        display_name: format!("INV-{}", rand::random::<u32>() % 1000),
                        metadata: None,
                    };
                    let mut current_tokens = tokens.get();
                    current_tokens.push(new_token);
                    set_tokens.set(current_tokens);
                }>
                    "Add Random Invoice Tag"
                </button>
            </div>
        </div>
    }
}
```

6. **Write comprehensive tests**:
```rust
// components/entity-tag/tests/unit.rs
#[cfg(test)]
mod tests {
    use super::*;
    use taxtalk_entity_tag::*;
    
    #[test]
    fn test_entity_tag_renders() {
        let token = EntityToken {
            id: "test-1".to_string(),
            entity_type: "client".to_string(),
            display_name: "Test Client".to_string(),
            metadata: None,
        };
        
        let html = render_to_string(|| view! {
            <EntityTag
                token=Signal::derive(|| Some(token.clone()))
                on_remove=Callback::new(|_| {})
            />
        });
        
        assert!(html.contains("Test Client"));
        assert!(html.contains("tui-et-client"));
    }
    
    #[test]
    fn test_crux_state_management() {
        let app = EntityTagApp;
        let mut state = EntityTagState::default();
        
        let token = EntityToken {
            id: "1".to_string(),
            entity_type: "product".to_string(),
            display_name: "Product A".to_string(),
            metadata: None,
        };
        
        app.update(EntityTagEvent::SetToken(token.clone()), &mut state, &());
        assert_eq!(state.token, Some(token));
        
        app.update(EntityTagEvent::Remove, &mut state, &());
        assert!(state.is_removing);
        
        app.update(EntityTagEvent::RemovalConfirmed, &mut state, &());
        assert!(state.token.is_none());
        assert!(!state.is_removing);
    }
}
```

#### Task 2.2: Extract TokenAutocomplete Component
**Objective**: Build autocomplete with performance optimization

**Implementation with Performance Focus**:
```rust
// components/token-autocomplete/src/component.rs
use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn TokenAutocomplete(
    input_value: Signal<String>,
    on_select: Callback<TokenHint, ()>,
    #[prop(default = 300)] debounce_ms: u64,
    #[prop(default = 5)] max_suggestions: usize,
) -> impl IntoView {
    // Debounced search with performance optimization
    let (suggestions, set_suggestions) = signal(Vec::<TokenHint>::new());
    let (is_loading, set_is_loading) = signal(false);
    
    // Performance: Debounce input to reduce computation
    let debounced_value = use_debounced_value(input_value, Duration::from_millis(debounce_ms));
    
    // Performance: Memoize filtered suggestions
    let filtered_suggestions = Memo::new(move |_| {
        let query = debounced_value.get().to_lowercase();
        if query.is_empty() {
            return vec![];
        }
        
        let all_hints = get_available_hints();
        all_hints.into_iter()
            .filter(|hint| {
                hint.token.to_lowercase().contains(&query) ||
                hint.description.to_lowercase().contains(&query)
            })
            .take(max_suggestions)
            .collect()
    });
    
    // Update suggestions when filtered list changes
    Effect::new(move |_| {
        set_suggestions.set(filtered_suggestions.get());
    });
    
    view! {
        <div class="tui-ta-container">
            <Show
                when=move || !suggestions.get().is_empty()
                fallback=|| ()
            >
                <div class="tui-ta-dropdown">
                    <For
                        each=move || suggestions.get()
                        key=|hint| hint.token.clone()
                        children=move |hint| {
                            let hint_clone = hint.clone();
                            view! {
                                <div
                                    class="tui-ta-suggestion"
                                    on:click=move |_| on_select.run(hint_clone.clone())
                                >
                                    <span class="tui-ta-token">{hint.token}</span>
                                    <span class="tui-ta-description">{hint.description}</span>
                                </div>
                            }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}

// Performance optimization: Debounced value hook
fn use_debounced_value(value: Signal<String>, delay: Duration) -> Signal<String> {
    let (debounced, set_debounced) = signal(value.get());
    
    Effect::new(move |_| {
        let current = value.get();
        set_timeout(
            move || set_debounced.set(current),
            delay,
        );
    });
    
    Signal::derive(move || debounced.get())
}
```

## Phase 2: Form & Input Components (Weeks 3-6)

### Weeks 3-4: Core Form Components

#### Task 3.1: Extract EntityCombobox with Advanced Features
**Objective**: Create high-performance entity selector with caching

**Implementation with API Integration**:
```rust
// components/entity-combobox/src/component.rs
use leptos::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[component]
pub fn EntityCombobox(
    entity_type: String,
    on_select: Callback<Entity, ()>,
    #[prop(default = true)] enable_cache: bool,
    #[prop(default = 500)] search_debounce_ms: u64,
    #[prop(default = 10)] page_size: usize,
) -> impl IntoView {
    // Performance: Entity cache
    let cache = use_context::<Arc<EntityCache>>()
        .unwrap_or_else(|| Arc::new(EntityCache::new()));
    
    // State management
    let (search_query, set_search_query) = signal(String::new());
    let (results, set_results) = signal(Vec::<Entity>::new());
    let (is_loading, set_is_loading) = signal(false);
    let (selected_index, set_selected_index) = signal(0);
    let (has_more, set_has_more) = signal(false);
    let (page, set_page) = signal(0);
    
    // Performance: Virtual scrolling for large lists
    let visible_range = use_virtual_list(results, page_size);
    
    // Debounced search
    let debounced_query = use_debounced_value(search_query, Duration::from_millis(search_debounce_ms));
    
    // Search effect with caching
    Effect::new(move |_| {
        let query = debounced_query.get();
        let entity_type = entity_type.clone();
        
        spawn_local(async move {
            // Check cache first
            if enable_cache {
                if let Some(cached) = cache.get(&entity_type, &query).await {
                    set_results.set(cached);
                    return;
                }
            }
            
            set_is_loading.set(true);
            
            match search_entities(entity_type.clone(), query.clone(), page.get(), page_size).await {
                Ok(response) => {
                    set_results.set(response.results.clone());
                    set_has_more.set(response.has_more);
                    
                    // Update cache
                    if enable_cache {
                        cache.set(&entity_type, &query, response.results).await;
                    }
                }
                Err(err) => {
                    leptos::logging::error!("Search failed: {:?}", err);
                }
            }
            
            set_is_loading.set(false);
        });
    });
    
    // Keyboard navigation
    let handle_keydown = move |e: KeyboardEvent| {
        match e.key().as_str() {
            "ArrowDown" => {
                let max = results.get().len().saturating_sub(1);
                set_selected_index.update(|i| *i = (*i + 1).min(max));
            }
            "ArrowUp" => {
                set_selected_index.update(|i| *i = i.saturating_sub(1));
            }
            "Enter" => {
                if let Some(entity) = results.get().get(selected_index.get()) {
                    on_select.run(entity.clone());
                }
            }
            _ => {}
        }
    };
    
    view! {
        <div class="tui-ec-container">
            <input
                type="text"
                class="tui-ec-input"
                placeholder=format!("Search {}...", entity_type)
                on:input=move |e| set_search_query.set(event_target_value(&e))
                on:keydown=handle_keydown
            />
            
            <Show
                when=move || is_loading.get()
                fallback=|| ()
            >
                <div class="tui-ec-loading">
                    <div class="tui-ec-spinner"></div>
                    "Loading..."
                </div>
            </Show>
            
            <Show
                when=move || !results.get().is_empty()
                fallback=|| ()
            >
                <div class="tui-ec-dropdown">
                    <For
                        each=move || visible_range.get()
                        key=|entity| entity.id.clone()
                        children=move |entity| {
                            let is_selected = move || selected_index.get() == index;
                            let entity_clone = entity.clone();
                            
                            view! {
                                <div
                                    class=move || {
                                        if is_selected() {
                                            "tui-ec-item tui-ec-item-selected"
                                        } else {
                                            "tui-ec-item"
                                        }
                                    }
                                    on:click=move |_| on_select.run(entity_clone.clone())
                                >
                                    <span class="tui-ec-name">{entity.display_name}</span>
                                    <span class="tui-ec-type">{entity.entity_type}</span>
                                </div>
                            }
                        }
                    />
                    
                    <Show
                        when=move || has_more.get()
                        fallback=|| ()
                    >
                        <button
                            class="tui-ec-load-more"
                            on:click=move |_| set_page.update(|p| *p += 1)
                        >
                            "Load More"
                        </button>
                    </Show>
                </div>
            </Show>
        </div>
    }
}
```

## Phase 3: Smart Components (Weeks 7-10)

### Task 4.1: Extract SmartSuggestions with ML Integration
**Objective**: Context-aware suggestions with machine learning

**Implementation**:
```rust
// components/smart-suggestions/src/ml.rs
use candle_core::{Device, Tensor};
use candle_transformers::models::bert::{BertModel, Config};

pub struct SuggestionEngine {
    model: BertModel,
    tokenizer: Tokenizer,
    context_embeddings: HashMap<String, Tensor>,
}

impl SuggestionEngine {
    pub async fn generate_suggestions(&self, context: &str) -> Vec<Suggestion> {
        // Tokenize context
        let tokens = self.tokenizer.encode(context, true).unwrap();
        
        // Get embeddings
        let input_ids = Tensor::new(&tokens.get_ids(), &Device::Cpu).unwrap();
        let embeddings = self.model.forward(&input_ids).unwrap();
        
        // Find similar contexts
        let similarities = self.compute_similarities(&embeddings);
        
        // Generate ranked suggestions
        self.rank_suggestions(similarities)
    }
    
    fn compute_similarities(&self, query: &Tensor) -> Vec<(String, f32)> {
        self.context_embeddings
            .iter()
            .map(|(context, embedding)| {
                let similarity = cosine_similarity(query, embedding);
                (context.clone(), similarity)
            })
            .collect()
    }
}
```

## Phase 4: Performance & Production (Weeks 11-12)

### Task 5.1: Component Health Dashboard
**Objective**: Real-time monitoring of component health

**Implementation**:
```rust
// components/core/src/health.rs
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct ComponentHealth {
    pub name: String,
    pub render_time: Duration,
    pub memory_usage: usize,
    pub error_count: usize,
    pub last_updated: Instant,
}

#[component]
pub fn HealthDashboard() -> impl IntoView {
    let (components, set_components) = signal(Vec::<ComponentHealth>::new());
    
    // Poll health data every second
    set_interval(
        move || {
            let health_data = collect_component_health();
            set_components.set(health_data);
        },
        Duration::from_secs(1),
    );
    
    view! {
        <div class="health-dashboard">
            <h2>"Component Health Monitor"</h2>
            <table>
                <thead>
                    <tr>
                        <th>"Component"</th>
                        <th>"Render Time"</th>
                        <th>"Memory"</th>
                        <th>"Errors"</th>
                        <th>"Status"</th>
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || components.get()
                        key=|c| c.name.clone()
                        children=move |component| {
                            let status = get_health_status(&component);
                            let status_class = match status {
                                HealthStatus::Healthy => "status-healthy",
                                HealthStatus::Warning => "status-warning",
                                HealthStatus::Critical => "status-critical",
                            };
                            
                            view! {
                                <tr>
                                    <td>{component.name}</td>
                                    <td>{format!("{:?}", component.render_time)}</td>
                                    <td>{format_bytes(component.memory_usage)}</td>
                                    <td>{component.error_count}</td>
                                    <td class=status_class>{format!("{:?}", status)}</td>
                                </tr>
                            }
                        }
                    />
                </tbody>
            </table>
        </div>
    }
}
```

## Testing & Validation Tasks

### Task 6.1: Integration Test Suite
**Objective**: Ensure components work together

```rust
// tests/integration/token_flow.rs
#[test]
async fn test_complete_token_resolution_flow() {
    // 1. User types "@client"
    let input = simulate_input("@client");
    
    // 2. TokenAutocomplete shows suggestions
    let suggestions = TokenAutocomplete::get_suggestions(&input).await;
    assert!(!suggestions.is_empty());
    
    // 3. EntityCombobox appears
    let combobox = EntityCombobox::new("client");
    let entities = combobox.search("ABC").await;
    assert!(!entities.is_empty());
    
    // 4. User selects entity
    let selected = entities[0].clone();
    
    // 5. EntityTag is created
    let tag = EntityTag::new(selected.into());
    assert_eq!(tag.display_name, "ABC Corporation");
    
    // 6. Validate complete flow
    assert!(validate_token_flow(&input, &tag));
}
```

## Rollback Procedures

### Task 7.1: Feature Flag Implementation
**Objective**: Enable gradual rollout and rollback

```rust
// core/src/feature_flags.rs
pub struct FeatureFlags {
    flags: HashMap<String, bool>,
}

impl FeatureFlags {
    pub fn is_enabled(&self, component: &str) -> bool {
        self.flags.get(component).copied().unwrap_or(false)
    }
    
    pub fn use_new_component<T>(&self, 
        component_name: &str,
        new_component: T,
        old_component: T,
    ) -> T {
        if self.is_enabled(component_name) {
            new_component
        } else {
            old_component
        }
    }
}

// Usage in app
#[component]
fn App() -> impl IntoView {
    let flags = use_context::<FeatureFlags>().unwrap();
    
    view! {
        {if flags.is_enabled("entity-tag") {
            view! { <NewEntityTag /> }
        } else {
            view! { <OldEntityTag /> }
        }}
    }
}
```

## Documentation Tasks

### Task 8.1: Component Documentation Template
**Objective**: Standardize documentation

```markdown
# Component Name

## Overview
Brief description of what the component does.

## Installation
```toml
[dependencies]
taxtalk-component-name = { path = "../component-name" }
```

## Basic Usage
```rust
use taxtalk_component::ComponentName;

#[component]
fn App() -> impl IntoView {
    view! {
        <ComponentName
            value=signal("test")
            on_change=|v| println!("Changed: {}", v)
        />
    }
}
```

## Props
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| value | Signal<T> | Required | The component value |
| on_change | Callback<T> | Required | Called when value changes |

## Events
- `on_change`: Fired when the value changes
- `on_submit`: Fired on form submission

## Styling
The component uses CSS classes with `tui-xx-` prefix.

## Examples
See `examples/` directory for more usage patterns.

## Performance
- Bundle size: XXkb gzipped
- Initial render: < 100ms
- Re-render: < 16ms

## Testing
```bash
cargo test
cargo bench
```
```

## Success Validation

### Final Checklist
- [ ] All 27 components extracted
- [ ] Crux integration working
- [ ] Performance targets met (<200KB total, <100ms render)
- [ ] 90%+ test coverage
- [ ] Documentation complete
- [ ] CI/CD pipeline running
- [ ] Health dashboard operational
- [ ] Rollback procedures tested
- [ ] Team trained on new architecture
- [ ] Production deployment successful

This comprehensive task list provides clear, actionable items that can be executed by AI agents or developers to successfully implement the component extraction plan according to the Enhanced PRD requirements.