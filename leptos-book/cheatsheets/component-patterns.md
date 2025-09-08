# Leptos Component Patterns Cheatsheet

## Core Component Patterns

### Basic Component Structure
```rust
#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <div>
            <h1>"Hello World"</h1>
        </div>
    }
}
```

### Component with Props
```rust
#[component]
pub fn Greeting(name: String) -> impl IntoView {
    view! {
        <p>"Hello, " {name} "!"</p>
    }
}

// Usage
<Greeting name="Alice"/>
```

### Component with Children
```rust
#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="card">
            {children()}
        </div>
    }
}

// Usage
<Card>
    <h2>"Card Title"</h2>
    <p>"Card content"</p>
</Card>
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
            <button on:click=move |_| set_count.update(|n| *n += 1)>
                "+"
            </button>
        </div>
    }
}
```

### Derived Signals
```rust
#[component]
pub fn UserDisplay(user: ReadSignal<User>) -> impl IntoView {
    let display_name = create_memo(move |_| {
        format!("{} {}", user().first_name, user().last_name)
    });

    view! {
        <div>
            <h3>{display_name}</h3>
            <p>{user().email}</p>
        </div>
    }
}
```

### Effect Pattern
```rust
#[component]
pub fn DataFetcher() -> impl IntoView {
    let (data, set_data) = create_signal(None::<Vec<Item>>);
    let (loading, set_loading) = create_signal(false);

    create_effect(move |_| {
        set_loading.set(true);
        spawn_local(async move {
            let result = fetch_data().await;
            set_data.set(Some(result));
            set_loading.set(false);
        });
    });

    view! {
        <div>
            {move || loading().then(|| view! { <p>"Loading..."</p> })}
            {move || data().map(|items| view! {
                <ul>
                    {items.into_iter().map(|item| view! {
                        <li>{item.name}</li>
                    }).collect_view()}
                </ul>
            })}
        </div>
    }
}
```

## Control Flow Patterns

### Conditional Rendering
```rust
#[component]
pub fn StatusDisplay(status: ReadSignal<Status>) -> impl IntoView {
    view! {
        <div>
            {move || match status() {
                Status::Loading => view! { <p>"Loading..."</p> },
                Status::Success(data) => view! { <p>"Success: " {data}</p> },
                Status::Error(err) => view! { <p>"Error: " {err}</p> },
            }}
        </div>
    }
}
```

### List Rendering
```rust
#[component]
pub fn ItemList(items: ReadSignal<Vec<Item>>) -> impl IntoView {
    view! {
        <ul>
            {move || items().into_iter().enumerate().map(|(i, item)| view! {
                <li key=i>{item.name}</li>
            }).collect_view()}
        </ul>
    }
}
```

### Optional Rendering
```rust
#[component]
pub fn OptionalContent(show: ReadSignal<bool>) -> impl IntoView {
    view! {
        <div>
            {move || show().then(|| view! {
                <p>"This content is optional"</p>
            })}
        </div>
    }
}
```

## Event Handling Patterns

### Click Handler
```rust
#[component]
pub fn Button(on_click: Action<(), ()>) -> impl IntoView {
    view! {
        <button on:click=move |_| on_click.dispatch(())>
            "Click me"
        </button>
    }
}
```

### Form Input
```rust
#[component]
pub fn TextInput(value: RwSignal<String>) -> impl IntoView {
    view! {
        <input
            type="text"
            prop:value={move || value.get()}
            on:input=move |ev| value.set(event_target_value(&ev))
        />
    }
}
```

### Custom Events
```rust
#[component]
pub fn CustomButton(
    #[prop(into)] on_custom_event: Callback<CustomEvent>
) -> impl IntoView {
    view! {
        <button on:click=move |_| {
            on_custom_event.call(CustomEvent { data: "clicked".to_string() })
        }>
            "Custom Event"
        </button>
    }
}
```

## Advanced Patterns

### Provider Pattern
```rust
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    provide_context(Theme::default());

    children()
}

#[component]
pub fn ThemedComponent() -> impl IntoView {
    let theme = use_context::<Theme>().unwrap();

    view! {
        <div style=move || format!("color: {}", theme.text_color)>
            "Themed content"
        </div>
    }
}
```

### Slot Pattern
```rust
#[component]
pub fn Modal(
    header: Children,
    body: Children,
    footer: Children
) -> impl IntoView {
    view! {
        <div class="modal">
            <div class="modal-header">{header()}</div>
            <div class="modal-body">{body()}</div>
            <div class="modal-footer">{footer()}</div>
        </div>
    }
}

// Usage
<Modal
    header=|| view! { <h2>"Modal Title"</h2> }
    body=|| view! { <p>"Modal content"</p> }
    footer=|| view! { <button>"Close"</button> }
/>
```

### Render Prop Pattern
```rust
#[component]
pub fn DataList<T, F>(
    data: ReadSignal<Vec<T>>,
    render_item: F
) -> impl IntoView
where
    F: Fn(&T) -> View + 'static,
    T: 'static,
{
    view! {
        <ul>
            {move || data().iter().map(|item| render_item(item)).collect_view()}
        </ul>
    }
}

// Usage
<DataList
    data=items
    render_item=|item| view! { <li>{item.name}</li> }
/>
```

## Performance Patterns

### Memoization
```rust
#[component]
pub fn ExpensiveComponent(data: ReadSignal<Vec<ComplexData>>) -> impl IntoView {
    let processed_data = create_memo(move |_| {
        data().iter().map(|item| expensive_computation(item)).collect::<Vec<_>>()
    });

    view! {
        <div>
            {move || processed_data().iter().map(|result| view! {
                <p>{result}</p>
            }).collect_view()}
        </div>
    }
}
```

### Selective Updates
```rust
#[component]
pub fn SelectiveUpdate(
    data: ReadSignal<LargeData>,
    filter: ReadSignal<String>
) -> impl IntoView {
    let filtered_data = create_memo(move |_| {
        let data = data();
        let filter = filter();
        data.items.into_iter()
            .filter(|item| item.name.contains(&filter))
            .collect::<Vec<_>>()
    });

    view! {
        <div>
            <input
                prop:value=filter
                on:input=move |ev| filter.set(event_target_value(&ev))
            />
            <ul>
                {move || filtered_data().iter().map(|item| view! {
                    <li>{item.name}</li>
                }).collect_view()}
            </ul>
        </div>
    }
}
```

## Common Patterns

### Loading States
```rust
#[component]
pub fn AsyncContent<T: 'static>(
    data: Resource<(), T>,
    children: ChildrenFn<T>
) -> impl IntoView {
    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            {move || data.get().map(|result| children(result))}
        </Suspense>
    }
}
```

### Error Boundaries
```rust
#[component]
pub fn ErrorBoundary(children: Children) -> impl IntoView {
    let (error, set_error) = create_signal(None::<String>);

    view! {
        <ErrorBoundary fallback=move |errors| view! {
            <div class="error">
                <h3>"Something went wrong"</h3>
                <p>{errors}</p>
            </div>
        }>
            {children()}
        </ErrorBoundary>
    }
}
```

### Controlled Components
```rust
#[component]
pub fn ControlledInput(
    value: RwSignal<String>,
    #[prop(into)] on_change: Callback<String>
) -> impl IntoView {
    view! {
        <input
            type="text"
            prop:value={move || value.get()}
            on:input=move |ev| {
                let new_value = event_target_value(&ev);
                value.set(new_value.clone());
                on_change.call(new_value);
            }
        />
    }
}
```

## Best Practices

1. **Use signals for reactive state**
2. **Memoize expensive computations**
3. **Use proper keys for list items**
4. **Handle loading and error states**
5. **Keep components focused on single responsibility**
6. **Use children and slots for flexibility**
7. **Prefer RwSignal for two-way binding**
8. **Use Actions for complex async operations**
9. **Implement proper error boundaries**
10. **Profile and optimize performance bottlenecks**