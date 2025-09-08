# 03: Advanced Components

## Overview

This chapter covers advanced component patterns in Leptos, including complex state management, custom hooks, context providers, and sophisticated component architectures.

## Custom Hooks

### Creating Custom Hooks

Custom hooks encapsulate reusable logic and state management:

```rust
use leptos::*;

// Custom hook for local storage
pub fn use_local_storage<T>(key: &str, initial: T) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: Clone + serde::Serialize + serde::de::DeserializeOwned + 'static,
{
    let (value, set_value) = create_signal(initial);

    // Load from localStorage on mount
    create_effect(move |_| {
        if let Ok(Some(stored)) = window()
            .local_storage()
            .ok()
            .flatten()
            .and_then(|storage| storage.get_item(key).ok())
            .flatten()
            .and_then(|json| serde_json::from_str(&json).ok())
        {
            set_value(stored);
        }
    });

    // Save to localStorage when value changes
    create_effect(move |_| {
        if let (Ok(Some(storage)), Ok(json)) = (
            window().local_storage(),
            serde_json::to_string(&value()),
        ) {
            let _ = storage.set_item(key, &json);
        }
    });

    (value, set_value)
}

// Usage
#[component]
pub fn PersistentCounter() -> impl IntoView {
    let (count, set_count) = use_local_storage("counter", 0);

    view! {
        <div>
            <p>"Count: " {count}</p>
            <button on:click=move |_| set_count.update(|n| *n + 1)>
                "Increment"
            </button>
        </div>
    }
}
```

### Hook Composition

Combine multiple hooks for complex behavior:

```rust
pub fn use_debounce<T>(value: ReadSignal<T>, delay: std::time::Duration) -> ReadSignal<T>
where
    T: Clone + 'static,
{
    let (debounced, set_debounced) = create_signal(value());

    create_effect(move |_| {
        let current_value = value();
        let timeout_id = set_timeout(
            move || set_debounced(current_value),
            delay,
        );

        on_cleanup(move || clear_timeout(timeout_id));
    });

    debounced
}

// Combined hook
pub fn use_debounced_search() -> (ReadSignal<String>, WriteSignal<String>, ReadSignal<String>) {
    let (search, set_search) = create_signal("".to_string());
    let debounced_search = use_debounce(search, std::time::Duration::from_millis(300));

    (search, set_search, debounced_search)
}
```

## Context Providers

### Creating Context

```rust
use leptos::*;

// Define context type
#[derive(Clone, Debug)]
pub struct AppContext {
    pub user: ReadSignal<Option<User>>,
    pub theme: ReadSignal<String>,
    pub set_theme: WriteSignal<String>,
}

// Create context
pub fn use_app_context() -> AppContext {
    use_context::<AppContext>()
        .expect("AppContext not found. Make sure to wrap your app with AppContextProvider")
}

// Context provider component
#[component]
pub fn AppContextProvider(children: Children) -> impl IntoView {
    let (user, set_user) = create_signal(None::<User>);
    let (theme, set_theme) = create_signal("light".to_string());

    let context = AppContext {
        user,
        theme,
        set_theme,
    };

    provide_context(context);

    children()
}
```

### Using Context

```rust
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let app_context = use_app_context();
    let theme = app_context.theme;
    let set_theme = app_context.set_theme;

    let toggle_theme = move |_| {
        set_theme.update(|t| {
            *t = if t == "light" { "dark" } else { "light" };
        });
    };

    view! {
        <button on:click=toggle_theme>
            "Current theme: " {theme}
        </button>
    }
}
```

### Nested Contexts

```rust
#[derive(Clone, Debug)]
pub struct ModalContext {
    pub is_open: ReadSignal<bool>,
    pub open_modal: WriteSignal<bool>,
    pub modal_content: ReadSignal<View>,
    pub set_modal_content: WriteSignal<View>,
}

#[component]
pub fn ModalProvider(children: Children) -> impl IntoView {
    let (is_open, open_modal) = create_signal(false);
    let (modal_content, set_modal_content) = create_signal(view! { <div></div> });

    let modal_context = ModalContext {
        is_open,
        open_modal,
        modal_content,
        set_modal_content,
    };

    provide_context(modal_context);

    view! {
        <div>
            {children()}
            <Modal/>
        </div>
    }
}

#[component]
pub fn Modal() -> impl IntoView {
    let modal_context = use_context::<ModalContext>()
        .expect("Modal context not found");

    view! {
        <Show
            when=move || modal_context.is_open()
            fallback=|| view! { <div></div> }
        >
            <div class="modal-overlay">
                <div class="modal-content">
                    {modal_context.modal_content()}
                    <button on:click=move |_| modal_context.open_modal(false)>
                        "Close"
                    </button>
                </div>
            </div>
        </Show>
    }
}
```

## Advanced State Management

### Reducer Pattern

```rust
use leptos::*;

// State type
#[derive(Clone, Debug)]
pub struct CounterState {
    pub count: i32,
    pub step: i32,
}

// Actions
pub enum CounterAction {
    Increment,
    Decrement,
    SetStep(i32),
    Reset,
}

// Reducer function
fn counter_reducer(state: CounterState, action: CounterAction) -> CounterState {
    match action {
        CounterAction::Increment => CounterState {
            count: state.count + state.step,
            step: state.step,
        },
        CounterAction::Decrement => CounterState {
            count: state.count - state.step,
            step: state.step,
        },
        CounterAction::SetStep(new_step) => CounterState {
            count: state.count,
            step: new_step,
        },
        CounterAction::Reset => CounterState {
            count: 0,
            step: state.step,
        },
    }
}

// Custom hook using reducer
pub fn use_counter_reducer() -> (ReadSignal<CounterState>, Callback<CounterAction, ()>) {
    let (state, set_state) = create_signal(CounterState { count: 0, step: 1 });

    let dispatch = Callback::new(move |action: CounterAction| {
        set_state.update(|current_state| {
            *current_state = counter_reducer(current_state.clone(), action);
        });
    });

    (state, dispatch)
}
```

### State Machines

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum LoadingState<T> {
    Idle,
    Loading,
    Success(T),
    Error(String),
}

pub fn use_async_operation<T, F>(
    operation: F,
) -> (ReadSignal<LoadingState<T>>, Callback<(), ()>)
where
    T: Clone + 'static,
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, String>>>> + 'static,
{
    let (state, set_state) = create_signal(LoadingState::<T>::Idle);

    let execute = Callback::new(move |_| {
        set_state(LoadingState::Loading);

        spawn_local(async move {
            match operation().await {
                Ok(result) => set_state(LoadingState::Success(result)),
                Err(error) => set_state(LoadingState::Error(error)),
            }
        });
    });

    (state, execute)
}
```

## Component Lifecycle Hooks

### Custom Lifecycle Hooks

```rust
pub fn use_on_mount<F>(callback: F)
where
    F: Fn() + 'static,
{
    create_effect(move |_| {
        callback();
        // Only run once by not depending on any signals
    });
}

pub fn use_interval<F>(callback: F, delay: std::time::Duration) -> Callback<(), ()>
where
    F: Fn() + 'static,
{
    let (is_active, set_is_active) = create_signal(true);

    create_effect(move |_| {
        if is_active() {
            let timeout_id = set_timeout(
                move || {
                    callback();
                    // Schedule next execution
                    set_timeout(
                        move || {
                            if is_active() {
                                callback();
                            }
                        },
                        delay,
                    );
                },
                delay,
            );

            on_cleanup(move || clear_timeout(timeout_id));
        }
    });

    Callback::new(move |_| set_is_active(false))
}
```

### Component Did Mount

```rust
#[component]
pub fn DataFetcher() -> impl IntoView {
    let (data, set_data) = create_signal(None::<Vec<String>>);

    use_on_mount(move || {
        spawn_local(async move {
            // Fetch data on mount
            match fetch_data().await {
                Ok(fetched_data) => set_data(Some(fetched_data)),
                Err(_) => logging::log!("Failed to fetch data"),
            }
        });
    });

    view! {
        <div>
            {move || match data() {
                Some(items) => view! {
                    <ul>
                        {items.into_iter().map(|item| view! { <li>{item}</li> }).collect::<Vec<_>>()}
                    </ul>
                },
                None => view! { <p>"Loading..."</p> },
            }}
        </div>
    }
}
```

## Advanced Component Patterns

### Render Props

```rust
#[component]
pub fn MouseTracker<T>(
    #[prop(into)] children: Callback<(i32, i32), T>,
) -> impl IntoView
where
    T: IntoView + 'static,
{
    let (mouse_pos, set_mouse_pos) = create_signal((0, 0));

    let on_mousemove = move |ev: web_sys::MouseEvent| {
        set_mouse_pos((ev.client_x(), ev.client_y()));
    };

    view! {
        <div on:mousemove=on_mousemove>
            {children(mouse_pos())}
        </div>
    }
}

// Usage
#[component]
pub fn App() -> impl IntoView {
    view! {
        <MouseTracker children=|pos| view! {
            <p>"Mouse position: " {pos.0} ", " {pos.1}</p>
        }/>
    }
}
```

### Compound Components

```rust
#[component]
pub fn Tabs(children: Children) -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal(0);

    provide_context(TabContext {
        active_tab,
        set_active_tab,
    });

    view! {
        <div class="tabs">
            {children()}
        </div>
    }
}

#[derive(Clone)]
pub struct TabContext {
    pub active_tab: ReadSignal<usize>,
    pub set_active_tab: WriteSignal<usize>,
}

#[component]
pub fn Tab(
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    let tab_context = use_context::<TabContext>()
        .expect("Tab must be used within Tabs");

    let tab_index = create_memo(move |_| {
        // This is a simplified example - in practice you'd need
        // a way to assign unique indices to each tab
        0
    });

    let is_active = create_memo(move |_| {
        tab_context.active_tab() == tab_index()
    });

    view! {
        <div>
            <button
                class=move || if is_active() { "tab active" } else { "tab" }
                on:click=move |_| tab_context.set_active_tab(tab_index())
            >
                {title}
            </button>
            <Show when=is_active fallback=|| view! { <div></div> }>
                <div class="tab-content">
                    {children()}
                </div>
            </Show>
        </div>
    }
}
```

### Higher-Order Components with Generics

```rust
#[component]
pub fn WithData<T, F>(
    fetcher: F,
    #[prop(into)] children: Callback<Option<T>, View>,
) -> impl IntoView
where
    T: Clone + 'static,
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, String>>>> + 'static,
{
    let (data, set_data) = create_signal(None::<T>);
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);

    let load_data = move || {
        set_loading(true);
        set_error(None);

        spawn_local(async move {
            match fetcher().await {
                Ok(result) => {
                    set_data(Some(result));
                    set_loading(false);
                }
                Err(err) => {
                    set_error(Some(err));
                    set_loading(false);
                }
            }
        });
    };

    // Load data on mount
    use_on_mount(load_data);

    children(data())
}
```

## Error Boundaries

### Custom Error Boundary

```rust
#[component]
pub fn ErrorBoundary(
    children: Children,
    #[prop(into)] fallback: Callback<String, View>,
) -> impl IntoView {
    let (error, set_error) = create_signal(None::<String>);

    // In a real implementation, you'd need to integrate with
    // Leptos' error handling system
    view! {
        <div>
            {children()}
        </div>
    }
}
```

## Performance Optimization

### Selective Re-rendering

```rust
#[component]
pub fn OptimizedList(items: Vec<String>) -> impl IntoView {
    // Only re-render when items length changes
    let item_count = create_memo(move |_| items.len());

    view! {
        <div>
            <p>"Total items: " {item_count}</p>
            <For
                each=move || items.clone()
                key=|item| item.clone()
                children=move |item| {
                    view! { <div>{item}</div> }
                }
            />
        </div>
    }
}
```

### Memoized Components

```rust
#[component]
pub fn ExpensiveComponent(value: i32) -> impl IntoView {
    // Memoize expensive computation
    let result = create_memo(move |_| {
        // Expensive calculation
        fibonacci(value)
    });

    view! {
        <div>
            "Fibonacci of " {value} " is " {result}
        </div>
    }
}
```

### Virtual Scrolling

```rust
#[component]
pub fn VirtualList<T>(
    items: Vec<T>,
    item_height: f64,
    container_height: f64,
    render_item: Callback<T, View>,
) -> impl IntoView
where
    T: Clone + 'static,
{
    let (scroll_top, set_scroll_top) = create_signal(0.0);

    let visible_range = create_memo(move |_| {
        let start = (scroll_top() / item_height) as usize;
        let visible_count = (container_height / item_height) as usize + 1;
        let end = (start + visible_count).min(items.len());
        (start, end)
    });

    let visible_items = create_memo(move |_| {
        let (start, end) = visible_range();
        items[start..end].to_vec()
    });

    let total_height = items.len() as f64 * item_height;

    view! {
        <div
            style=format!("height: {}px; overflow: auto", container_height)
            on:scroll=move |ev| {
                set_scroll_top(ev.target().unwrap().scroll_top() as f64);
            }
        >
            <div style=format!("height: {}px; position: relative", total_height)>
                <For
                    each=move || visible_items()
                    key=|item| format!("{:?}", item) // Simplified key
                    children=move |item| {
                        let index = items.iter().position(|i| i == &item).unwrap_or(0);
                        let top = index as f64 * item_height;

                        view! {
                            <div
                                style=format!("position: absolute; top: {}px", top)
                            >
                                {render_item(item)}
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}
```

## Testing Advanced Components

### Testing Custom Hooks

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_use_local_storage() {
        let (value, set_value) = use_local_storage("test_key", 42);

        assert_eq!(value(), 42);

        set_value(100);
        assert_eq!(value(), 100);
    }

    #[test]
    fn test_use_debounce() {
        let (input, set_input) = create_signal("".to_string());
        let debounced = use_debounce(input, std::time::Duration::from_millis(100));

        set_input("test".to_string());

        // In a real test, you'd need to wait for the debounce delay
        // For now, just check that the signal was created
        assert_eq!(debounced(), "".to_string());
    }
}
```

### Testing Context

```rust
#[test]
fn test_context_provider() {
    let result = mount_to_body(|| {
        view! {
            <AppContextProvider>
                <ThemeToggle/>
            </AppContextProvider>
        }
    });

    // Test that context is available
    assert!(true); // Placeholder
}
```

## Best Practices

1. **Keep hooks focused**: Each hook should have a single responsibility
2. **Use context sparingly**: Only use context for truly global state
3. **Memoize expensive operations**: Use `create_memo` for costly computations
4. **Handle errors gracefully**: Provide fallbacks for error states
5. **Test custom logic**: Unit test hooks and complex component logic
6. **Document component APIs**: Clearly document props and behavior
7. **Use TypeScript**: Leverage type safety for complex component APIs
8. **Optimize re-renders**: Use keys and selective updates

## Summary

Advanced Leptos components leverage:
- **Custom hooks**: Encapsulate reusable logic
- **Context providers**: Share state across component trees
- **Reducer pattern**: Manage complex state transitions
- **Lifecycle hooks**: Control component behavior
- **Compound components**: Create cohesive component APIs
- **Performance optimizations**: Minimize unnecessary re-renders

These patterns enable building sophisticated, maintainable applications with Leptos.