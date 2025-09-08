# Component Patterns Cheatsheet

## Overview

This cheatsheet provides quick reference for common Leptos component patterns, including:

- Basic component structure
- Props and children patterns
- State management patterns
- Event handling patterns
- Lifecycle patterns
- Performance optimization patterns
- Testing patterns

## Basic Component Structure

### Simple Component

```rust
#[component]
pub fn SimpleButton() -> impl IntoView {
    view! {
        <button>"Click me"</button>
    }
}
```

### Component with Props

```rust
#[derive(Clone, Debug)]
pub struct ButtonProps {
    pub text: String,
    pub on_click: Callback<()>,
    pub disabled: bool,
}

#[component]
pub fn Button(props: ButtonProps) -> impl IntoView {
    view! {
        <button
            disabled=props.disabled
            on:click=move |_| props.on_click.call(())
        >
            {props.text}
        </button>
    }
}
```

### Component with Children

```rust
#[component]
pub fn Card(
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="card">
            <h3>{title}</h3>
            <div class="card-content">
                {children()}
            </div>
        </div>
    }
}
```

## Props Patterns

### Optional Props with Defaults

```rust
#[component]
pub fn Input(
    #[prop(into)] placeholder: String,
    #[prop(default = "text".to_string())] input_type: String,
    #[prop(default = false)] disabled: bool,
    #[prop(optional)] value: Option<String>,
) -> impl IntoView {
    view! {
        <input
            type=input_type
            placeholder=placeholder
            disabled=disabled
            prop:value=value.unwrap_or_default()
        />
    }
}
```

### Props with Into Trait

```rust
#[component]
pub fn Heading(
    #[prop(into)] level: u8,
    #[prop(into)] text: String,
) -> impl IntoView {
    match level {
        1 => view! { <h1>{text}</h1> },
        2 => view! { <h2>{text}</h2> },
        3 => view! { <h3>{text}</h3> },
        _ => view! { <h4>{text}</h4> },
    }
}

// Usage
<Heading level=1 text="Main Title" />
<Heading level="2" text=move || format!("Count: {}", count.get()) />
```

### Generic Props

```rust
#[component]
pub fn List<T: Clone + 'static>(
    items: Vec<T>,
    render_item: Callback<T, impl IntoView>,
) -> impl IntoView {
    view! {
        <ul>
            <For
                each=move || items.clone()
                key=|item| format!("{:?}", item)
                children=move |item| view! {
                    <li>{render_item.call(item)}</li>
                }
            />
        </ul>
    }
}
```

## State Management Patterns

### Local State with Signals

```rust
#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div>
            <p>"Count: " {count}</p>
            <button on:click=move |_| set_count.update(|n| *n + 1)>
                "+"
            </button>
            <button on:click=move |_| set_count.update(|n| *n - 1)>
                "-"
            </button>
        </div>
    }
}
```

### Derived State

```rust
#[component]
pub fn TodoList() -> impl IntoView {
    let (todos, set_todos) = create_signal(vec![
        Todo { id: 1, text: "Learn Leptos".to_string(), completed: false },
        Todo { id: 2, text: "Build app".to_string(), completed: true },
    ]);

    let completed_count = create_memo(move |_| {
        todos.get().iter().filter(|todo| todo.completed).count()
    });

    let total_count = create_memo(move |_| todos.get().len());

    view! {
        <div>
            <p>"Completed: " {completed_count} " / " {total_count}</p>
            <For
                each=move || todos.get()
                key=|todo| todo.id
                children=move |todo| view! {
                    <TodoItem todo=todo set_todos=set_todos />
                }
            />
        </div>
    }
}
```

### Shared State with Context

```rust
#[derive(Clone, Debug)]
pub struct AppState {
    pub user: RwSignal<Option<User>>,
    pub theme: RwSignal<String>,
}

pub type AppContext = RwSignal<AppState>;

#[component]
pub fn App() -> impl IntoView {
    let state = create_rw_signal(AppState {
        user: create_rw_signal(None),
        theme: create_rw_signal("light".to_string()),
    });

    provide_context(state);

    view! {
        <Router>
            <AppLayout>
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/profile" view=ProfilePage />
                </Routes>
            </AppLayout>
        </Router>
    }
}

#[component]
pub fn ProfilePage() -> impl IntoView {
    let state = use_context::<AppContext>()
        .expect("AppContext should be provided");

    let user = create_read_slice(state, |state| state.user.get());

    view! {
        <div>
            {move || user.get().map(|user| {
                view! { <p>"Welcome, " {user.name}</p> }
            }).unwrap_or_else(|| {
                view! { <p>"Please log in"</p> }
            })}
        </div>
    }
}
```

## Event Handling Patterns

### Click Events

```rust
#[component]
pub fn ClickableButton() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button on:click=move |_| set_count.update(|n| *n + 1)>
            "Clicked " {count} " times"
        </button>
    }
}
```

### Form Events

```rust
#[component]
pub fn LoginForm() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        // Handle login logic
        log::info!("Login attempt: {} / {}", email.get(), password.get());
    };

    view! {
        <form on:submit=on_submit>
            <input
                type="email"
                placeholder="Email"
                prop:value=email
                on:input=move |ev| {
                    let target = event_target::<web_sys::HtmlInputElement>(&ev);
                    if let Some(input) = target {
                        set_email.set(input.value());
                    }
                }
            />
            <input
                type="password"
                placeholder="Password"
                prop:value=password
                on:input=move |ev| {
                    let target = event_target::<web_sys::HtmlInputElement>(&ev);
                    if let Some(input) = target {
                        set_password.set(input.value());
                    }
                }
            />
            <button type="submit">"Login"</button>
        </form>
    }
}
```

### Custom Events

```rust
#[derive(Clone, Debug)]
pub struct CustomEvent {
    pub event_type: String,
    pub data: serde_json::Value,
}

#[component]
pub fn EventEmitter() -> impl IntoView {
    let on_custom_event = move |event: CustomEvent| {
        log::info!("Custom event: {} - {:?}", event.event_type, event.data);
    };

    view! {
        <CustomButton on_custom_event=on_custom_event />
    }
}

#[component]
pub fn CustomButton(
    on_custom_event: Callback<CustomEvent>,
) -> impl IntoView {
    let emit_event = move |_| {
        let event = CustomEvent {
            event_type: "button_clicked".to_string(),
            data: serde_json::json!({ "timestamp": js_sys::Date::now() }),
        };
        on_custom_event.call(event);
    };

    view! {
        <button on:click=emit_event>"Emit Custom Event"</button>
    }
}
```

### Keyboard Events

```rust
#[component]
pub fn SearchInput() -> impl IntoView {
    let (query, set_query) = create_signal(String::new());

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            // Perform search
            log::info!("Searching for: {}", query.get());
        } else if ev.key() == "Escape" {
            set_query.set(String::new());
        }
    };

    view! {
        <input
            type="text"
            placeholder="Search..."
            prop:value=query
            on:input=move |ev| {
                let target = event_target::<web_sys::HtmlInputElement>(&ev);
                if let Some(input) = target {
                    set_query.set(input.value());
                }
            }
            on:keydown=on_keydown
        />
    }
}
```

## Lifecycle Patterns

### Component with Effects

```rust
#[component]
pub fn DataFetcher() -> impl IntoView {
    let (data, set_data) = create_signal(None::<Vec<String>>);
    let (loading, set_loading) = create_signal(true);

    // Effect runs when component mounts
    create_effect(move |_| {
        spawn_local(async move {
            // Simulate API call
            gloo::timers::future::TimeoutFuture::new(1000).await;
            set_data.set(Some(vec!["Item 1".to_string(), "Item 2".to_string()]));
            set_loading.set(false);
        });
    });

    view! {
        <div>
            {move || if loading.get() {
                view! { <p>"Loading..."</p> }
            } else {
                view! {
                    <ul>
                        <For
                            each=move || data.get().unwrap_or_default()
                            key=|item| item.clone()
                            children=move |item| view! {
                                <li>{item}</li>
                            }
                        />
                    </ul>
                }
            }}
        </div>
    }
}
```

### Cleanup with on_cleanup

```rust
#[component]
pub fn Timer() -> impl IntoView {
    let (seconds, set_seconds) = create_signal(0);

    // Effect with cleanup
    create_effect(move |_| {
        let interval_id = gloo::timers::callback::Interval::new(1000, move || {
            set_seconds.update(|s| *s + 1);
        });

        // Cleanup when component unmounts
        on_cleanup(move || {
            interval_id.cancel();
        });
    });

    view! {
        <div>
            <p>"Timer: " {seconds} " seconds"</p>
        </div>
    }
}
```

## Performance Optimization Patterns

### Memoization

```rust
#[component]
pub fn ExpensiveList(
    items: Vec<String>,
    filter: String,
) -> impl IntoView {
    // Memoize filtered items
    let filtered_items = create_memo(move |_| {
        items.iter()
            .filter(|item| item.to_lowercase().contains(&filter.to_lowercase()))
            .cloned()
            .collect::<Vec<_>>()
    });

    view! {
        <ul>
            <For
                each=move || filtered_items.get()
                key=|item| item.clone()
                children=move |item| view! {
                    <li>{item}</li>
                }
            />
        </ul>
    }
}
```

### Callback Memoization

```rust
#[component]
pub fn TodoItem(
    todo: Todo,
    on_toggle: Callback<i32>,
) -> impl IntoView {
    // Memoize the callback to prevent unnecessary re-renders
    let toggle_callback = create_memo(move |_| {
        let id = todo.id;
        move |_| on_toggle.call(id)
    });

    view! {
        <div class="todo-item">
            <input
                type="checkbox"
                prop:checked=todo.completed
                on:change=move |_| toggle_callback.get()()
            />
            <span class=move || if todo.completed { "completed" } else { "" }>
                {todo.text}
            </span>
        </div>
    }
}
```

### Selective Re-rendering

```rust
#[component]
pub fn OptimizedCounter() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (name, set_name) = create_signal("Counter".to_string());

    view! {
        <div>
            <h2>{name}</h2>
            <p>"Count: " {count}</p>
            <button on:click=move |_| set_count.update(|n| *n + 1)>
                "Increment"
            </button>
            <button on:click=move |_| set_name.set("Updated Counter".to_string())>
                "Change Name"
            </button>
        </div>
    }
}
```

## Testing Patterns

### Component Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_counter_increment() {
        let mut app = Counter::new();

        // Initial state
        assert_eq!(app.view().count, 0);

        // Simulate click
        app.update(Event::Increment);

        // Check updated state
        assert_eq!(app.view().count, 1);
    }

    #[test]
    fn test_button_props() {
        let (clicked, set_clicked) = create_signal(false);

        let button = Button(ButtonProps {
            text: "Test".to_string(),
            on_click: Callback::new(move |_| set_clicked.set(true)),
            disabled: false,
        });

        // Test initial state
        assert!(!clicked.get());

        // Simulate click (this would be done through DOM testing)
        // button.click();

        // In a real test, you'd use something like:
        // let rendered = render_to_string(|| button);
        // assert!(rendered.contains("Test"));
    }
}
```

### Effect Testing

```rust
#[test]
fn test_data_fetcher() {
    let fetcher = DataFetcher::new();

    // Initially loading
    assert!(fetcher.view().loading);

    // After some time (in real test, you'd mock the async call)
    // assert!(!fetcher.view().loading);
    // assert_eq!(fetcher.view().data.len(), 2);
}
```

## Advanced Patterns

### Higher-Order Components

```rust
pub fn with_loading<T: IntoView>(
    loading: ReadSignal<bool>,
    component: impl Fn() -> T,
) -> impl IntoView {
    move || {
        if loading.get() {
            view! { <div class="loading">"Loading..."</div> }
        } else {
            component().into_view()
        }
    }
}

#[component]
pub fn UserProfile() -> impl IntoView {
    let (user, set_user) = create_signal(None::<User>);
    let (loading, set_loading) = create_signal(true);

    // Load user data...

    with_loading(loading, move || {
        view! {
            <div>
                {move || user.get().map(|user| {
                    view! { <p>"Name: " {user.name}</p> }
                })}
            </div>
        }
    })
}
```

### Render Props

```rust
#[component]
pub fn DataProvider<T: Clone + 'static>(
    data: T,
    children: ChildrenFn,
) -> impl IntoView {
    provide_context(data);
    children()
}

#[component]
pub fn Consumer() -> impl IntoView {
    let data = use_context::<String>().unwrap();

    view! {
        <p>"Data: " {data}</p>
    }
}

// Usage
<DataProvider data="Hello World".to_string()>
    <Consumer />
</DataProvider>
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

#[component]
pub fn Tab(
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    let context = use_context::<TabContext>().unwrap();
    let tab_index = create_memo(move |_| {
        // Calculate tab index (simplified)
        0
    });

    let is_active = create_memo(move |_| {
        context.active_tab.get() == tab_index.get()
    });

    view! {
        <div class=move || if is_active.get() { "tab active" } else { "tab" }>
            {move || if is_active.get() {
                children()
            } else {
                view! { <div></div> }
            }}
        </div>
    }
}

#[component]
pub fn TabHeader(
    #[prop(into)] title: String,
) -> impl IntoView {
    let context = use_context::<TabContext>().unwrap();

    view! {
        <button on:click=move |_| context.set_active_tab.set(0)>
            {title}
        </button>
    }
}

// Usage
<Tabs>
    <TabHeader title="Tab 1" />
    <TabHeader title="Tab 2" />
    <Tab title="Tab 1">
        <p>"Content 1"</p>
    </Tab>
    <Tab title="Tab 2">
        <p>"Content 2"</p>
    </Tab>
</Tabs>
```

## Common Patterns Quick Reference

### Signal Patterns
- `create_signal(initial)` - Basic reactive value
- `create_rw_signal(initial)` - Read-write signal
- `create_memo(closure)` - Computed/derived value
- `create_effect(closure)` - Side effect
- `create_resource(fetcher, source)` - Async data fetching

### Event Patterns
- `on:click=move |_| ...` - Click handler
- `on:input=move |ev| ...` - Input change handler
- `on:submit=move |ev| ...` - Form submission
- `on:keydown=move |ev| ...` - Keyboard events

### Props Patterns
- `#[prop(into)]` - Convert prop to target type
- `#[prop(default = value)]` - Default value
- `#[prop(optional)]` - Optional prop
- `#[prop(into)] children: Children` - Child components

### Performance Patterns
- Use `create_memo` for expensive computations
- Memoize callbacks with `create_memo`
- Use `For` component for lists
- Avoid unnecessary re-renders

### State Patterns
- Local state: `create_signal`
- Shared state: `create_rw_signal` + `provide_context`
- Global state: Context + `use_context`
- Async state: `create_resource`

This cheatsheet covers the most common patterns you'll encounter when building Leptos applications. Each pattern includes the essential code structure and usage examples to help you quickly implement common functionality.