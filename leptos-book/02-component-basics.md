# 02: Component Basics

## Overview

Components are the building blocks of Leptos applications. Understanding how to create, compose, and manage components is essential for building maintainable and reusable UI code. This chapter covers the fundamental patterns for component development in Leptos.

## Component Definition

### Basic Component Structure

Components in Leptos are functions that return `impl IntoView`. The simplest component looks like this:

```rust
use leptos::*;

// Basic component
#[component]
pub fn Welcome() -> impl IntoView {
    view! {
        <div>
            <h1>"Welcome to Leptos!"</h1>
            <p>"This is a basic component."</p>
        </div>
    }
}
```

**Key Points:**
- Use `#[component]` attribute macro
- Return type is `impl IntoView`
- Use `view!` macro for JSX-like syntax
- Components are functions, not structs

### Component with Props

Components can accept properties (props) to make them configurable:

```rust
use leptos::*;

// Component with props
#[component]
pub fn Greeting(name: String, age: Option<i32>) -> impl IntoView {
    view! {
        <div>
            <h2>"Hello, " {name} "!"</h2>
            {age.map(|a| view! { <p>"You are " {a} " years old."</p> })}
        </div>
    }
}

// Usage
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <Greeting name="Alice".to_string() age=Some(25) />
            <Greeting name="Bob".to_string() age=None />
        </div>
    }
}
```

**Prop Rules:**
- Props are function parameters
- Use `String` for text props (not `&str`)
- Use `Option<T>` for optional props
- Props are owned by the component

## Component Composition

### Parent-Child Relationships

Components can be composed together to build complex UIs:

```rust
use leptos::*;

// Child component
#[component]
pub fn UserCard(user: User) -> impl IntoView {
    view! {
        <div class="user-card">
            <h3>{user.name}</h3>
            <p>{user.email}</p>
            <p>"Role: " {user.role}</p>
        </div>
    }
}

// Parent component
#[component]
pub fn UserList(users: Vec<User>) -> impl IntoView {
    view! {
        <div class="user-list">
            <h2>"Users"</h2>
            {users.into_iter()
                .map(|user| view! { <UserCard user=user /> })
                .collect::<Vec<_>>()}
        </div>
    }
}
```

### Fragment Components

Components can return multiple elements using fragments:

```rust
use leptos::*;

// Component returning multiple elements
#[component]
pub fn ArticleHeader(title: String, author: String, date: String) -> impl IntoView {
    view! {
        <header>
            <h1>{title}</h1>
            <div class="meta">
                <span class="author">"By " {author}</span>
                <span class="date">{date}</span>
            </div>
        </header>
    }
}
```

## Component State

### Local Component State

Components can have their own internal state using signals:

```rust
use leptos::*;

// Component with local state
#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div>
            <p>"Count: " {count}</p>
            <button on:click=move |_| set_count.update(|n| *n + 1)>
                "Increment"
            </button>
            <button on:click=move |_| set_count.set(0)>
                "Reset"
            </button>
        </div>
    }
}
```

### State Sharing Between Components

Share state between parent and child components:

```rust
use leptos::*;

// Child component that modifies parent state
#[component]
pub fn CounterButton(count: ReadSignal<i32>, set_count: WriteSignal<i32>) -> impl IntoView {
    view! {
        <button on:click=move |_| set_count.update(|n| *n + 1)>
            "Count: " {count}
        </button>
    }
}

// Parent component managing shared state
#[component]
pub fn CounterApp() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div>
            <h1>"Shared Counter"</h1>
            <CounterButton count=count set_count=set_count />
            <CounterButton count=count set_count=set_count />
            <p>"Total clicks: " {count}</p>
        </div>
    }
}
```

## Component Lifecycle

### Component Creation and Cleanup

Components can perform setup and cleanup:

```rust
use leptos::*;

// Component with lifecycle management
#[component]
pub fn Timer() -> impl IntoView {
    let (seconds, set_seconds) = create_signal(0);

    // Set up interval
    let interval_id = set_interval_with_handle(
        move || set_seconds.update(|s| *s + 1),
        std::time::Duration::from_secs(1)
    ).unwrap();

    // Clean up on component destruction
    on_cleanup(move || {
        if let Ok(id) = interval_id {
            id.clear();
        }
    });

    view! {
        <div>
            <p>"Timer: " {seconds} " seconds"</p>
        </div>
    }
}
```

### Effect Dependencies

Effects run when their reactive dependencies change:

```rust
use leptos::*;

// Component with effects
#[component]
pub fn UserProfile(user_id: String) -> impl IntoView {
    let (user, set_user) = create_signal(None::<User>);
    let (loading, set_loading) = create_signal(false);

    // Effect to fetch user data
    create_effect(move |_| {
        let id = user_id.clone();
        set_loading.set(true);

        // Simulate API call
        spawn_local(async move {
            // In real app, this would be an HTTP request
            let fetched_user = fetch_user(&id).await;
            set_user.set(Some(fetched_user));
            set_loading.set(false);
        });
    });

    view! {
        <div>
            {move || if loading.get() {
                view! { <p>"Loading..."</p> }
            } else if let Some(user) = user.get() {
                view! {
                    <div>
                        <h2>{user.name}</h2>
                        <p>{user.email}</p>
                    </div>
                }
            } else {
                view! { <p>"User not found"</p> }
            }}
        </div>
    }
}
```

## Conditional Rendering

### If-Else Patterns

Render different content based on conditions:

```rust
use leptos::*;

// Conditional rendering
#[component]
pub fn StatusIndicator(status: ReadSignal<String>) -> impl IntoView {
    view! {
        <div>
            {move || match status.get().as_str() {
                "online" => view! { <span class="online">"🟢 Online"</span> },
                "away" => view! { <span class="away">"🟡 Away"</span> },
                "offline" => view! { <span class="offline">"🔴 Offline"</span> },
                _ => view! { <span class="unknown">"❓ Unknown"</span> }
            }}
        </div>
    }
}
```

### Show/Hide Components

Use signals to control component visibility:

```rust
use leptos::*;

// Show/hide component
#[component]
pub fn CollapsiblePanel(title: String, children: Children) -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);

    view! {
        <div>
            <button on:click=move |_| set_is_open.update(|open| !*open)>
                {title} " " {move || if is_open.get() { "▼" } else { "▶" }}
            </button>
            {move || if is_open.get() {
                view! { <div class="panel-content">{children()}</div> }
            } else {
                view! { <div></div> }
            }}
        </div>
    }
}

// Usage
#[component]
pub fn App() -> impl IntoView {
    view! {
        <CollapsiblePanel title="Click to expand">
            <p>"This content is hidden by default."</p>
            <p>"It can contain any child elements."</p>
        </CollapsiblePanel>
    }
}
```

## List Rendering

### Rendering Collections

Render lists of items efficiently:

```rust
use leptos::*;

// List rendering
#[component]
pub fn TodoList(todos: ReadSignal<Vec<TodoItem>>) -> impl IntoView {
    view! {
        <ul>
            {move || todos.get()
                .into_iter()
                .enumerate()
                .map(|(index, todo)| view! {
                    <li key=index>
                        <input
                            type="checkbox"
                            prop:checked=todo.completed
                            on:change=move |ev| {
                                // Handle checkbox change
                            }
                        />
                        <span class=move || if todo.completed { "completed" } else { "" }>
                            {todo.text}
                        </span>
                    </li>
                })
                .collect::<Vec<_>>()}
        </ul>
    }
}
```

### Keyed Lists

Use keys for efficient list updates:

```rust
use leptos::*;

// Keyed list for better performance
#[component]
pub fn ProductList(products: ReadSignal<Vec<Product>>) -> impl IntoView {
    view! {
        <div class="product-grid">
            <For
                each=move || products.get()
                key=|product| product.id
                children=move |product| view! {
                    <div class="product-card">
                        <h3>{product.name}</h3>
                        <p>{product.description}</p>
                        <p class="price">"$" {product.price}</p>
                    </div>
                }
            />
        </div>
    }
}
```

## Component Communication

### Callback Props

Pass functions as props for child-to-parent communication:

```rust
use leptos::*;

// Child component with callback
#[component]
pub fn TextInput(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    on_submit: Callback<String>
) -> impl IntoView {
    let input_ref = create_node_ref::<html::Input>();

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let input = input_ref.get().unwrap();
        let text = input.value();
        on_submit.call(text);
        set_value.set(String::new());
    };

    view! {
        <form on:submit=handle_submit>
            <input
                type="text"
                value=value
                on:input=move |ev| set_value.set(event_target_value(&ev))
                node_ref=input_ref
            />
            <button type="submit">"Submit"</button>
        </form>
    }
}

// Parent component
#[component]
pub fn MessageForm() -> impl IntoView {
    let (message, set_message) = create_signal(String::new());
    let (messages, set_messages) = create_signal(vec![]);

    let add_message = Callback::new(move |text: String| {
        set_messages.update(|msgs| msgs.push(text));
    });

    view! {
        <div>
            <TextInput
                value=message
                set_value=set_message
                on_submit=add_message
            />
            <ul>
                {move || messages.get()
                    .into_iter()
                    .map(|msg| view! { <li>{msg}</li> })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
```

## Component Patterns

### Container/Presentational Pattern

Separate logic from presentation:

```rust
use leptos::*;

// Presentational component
#[component]
pub fn UserProfileView(user: ReadSignal<Option<User>>) -> impl IntoView {
    view! {
        <div>
            {move || match user.get() {
                Some(user) => view! {
                    <div class="profile">
                        <img src=user.avatar alt="Avatar" />
                        <h2>{user.name}</h2>
                        <p>{user.bio}</p>
                    </div>
                },
                None => view! { <div>"Loading..."</div> }
            }}
        </div>
    }
}

// Container component
#[component]
pub fn UserProfileContainer(user_id: String) -> impl IntoView {
    let (user, set_user) = create_signal(None::<User>);

    // Fetch user data
    create_effect(move |_| {
        let id = user_id.clone();
        spawn_local(async move {
            let fetched_user = fetch_user(&id).await;
            set_user.set(Some(fetched_user));
        });
    });

    view! {
        <UserProfileView user=user />
    }
}
```

### Higher-Order Components

Create reusable component wrappers:

```rust
use leptos::*;

// HOC for loading state
#[component]
pub fn WithLoading<T, F>(
    data: ReadSignal<Option<T>>,
    children: F
) -> impl IntoView
where
    F: Fn(ReadSignal<T>) -> impl IntoView + 'static,
    T: Clone + 'static,
{
    view! {
        <div>
            {move || match data.get() {
                Some(data) => children(ReadSignal::from(data)),
                None => view! { <div class="loading">"Loading..."</div> }
            }}
        </div>
    }
}

// Usage
#[component]
pub fn UserDashboard(user_id: String) -> impl IntoView {
    let (user, set_user) = create_signal(None::<User>);

    // Fetch logic here...

    view! {
        <WithLoading data=user>
            {move |user_signal| view! {
                <div>
                    <h1>"Welcome, " {move || user_signal.get().name}</h1>
                    <UserStats user=user_signal />
                </div>
            }}
        </WithLoading>
    }
}
```

## Component Best Practices

### Naming Conventions

**Component Names:**
- Use PascalCase for component names
- Be descriptive and specific
- Avoid generic names like `Component`, `Item`, `Card`

```rust
// ✅ Good
#[component]
pub fn UserProfile() -> impl IntoView { ... }

#[component]
pub fn ProductCard() -> impl IntoView { ... }

#[component]
pub fn NavigationMenu() -> impl IntoView { ... }

// ❌ Bad
#[component]
pub fn Component() -> impl IntoView { ... }

#[component]
pub fn Item() -> impl IntoView { ... }
```

### Prop Design

**Prop Guidelines:**
- Use meaningful prop names
- Prefer concrete types over generics when possible
- Use `Option<T>` for optional props
- Consider using builder pattern for complex props

```rust
// ✅ Good
#[component]
pub fn Button(
    children: Children,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    on_click: Callback<()>
) -> impl IntoView { ... }

// ❌ Bad - too many boolean props
#[component]
pub fn Button(
    children: Children,
    primary: bool,
    secondary: bool,
    large: bool,
    small: bool,
    disabled: bool,
    on_click: Callback<()>
) -> impl IntoView { ... }
```

### Component Size

**Keep Components Focused:**
- Each component should have a single responsibility
- Break large components into smaller ones
- Aim for components that fit on one screen

```rust
// ✅ Good - focused components
#[component]
pub fn UserForm() -> impl IntoView {
    view! {
        <div>
            <UserNameInput />
            <UserEmailInput />
            <UserAvatarUpload />
            <SubmitButton />
        </div>
    }
}

// ❌ Bad - monolithic component
#[component]
pub fn UserManagementPage() -> impl IntoView {
    // 500+ lines of mixed concerns
}
```

### Error Handling

**Handle Errors Gracefully:**
- Use `Result` types for fallible operations
- Provide fallback UI for error states
- Log errors appropriately
- Don't let component errors crash the app

```rust
use leptos::*;

// Component with error handling
#[component]
pub fn DataFetcher<T, F>(
    fetch_fn: F,
    children: ChildrenFn
) -> impl IntoView
where
    F: Fn() -> Result<T, String> + 'static,
    T: Clone + 'static,
{
    let (data, set_data) = create_signal(None::<Result<T, String>>);

    create_effect(move |_| {
        match fetch_fn() {
            Ok(result) => set_data.set(Some(Ok(result))),
            Err(error) => {
                log::error!("Failed to fetch data: {}", error);
                set_data.set(Some(Err(error)));
            }
        }
    });

    view! {
        <div>
            {move || match data.get() {
                Some(Ok(data)) => children(data).into_view(),
                Some(Err(error)) => view! {
                    <div class="error">
                        "Error: " {error}
                        <button on:click=move |_| set_data.set(None)>
                            "Retry"
                        </button>
                    </div>
                },
                None => view! { <div>"Loading..."</div> }
            }}
        </div>
    }
}
```

## Testing Components

### Component Testing Patterns

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_button_renders_correctly() {
        let mut app = leptos::create_runtime();

        let button = view! {
            <Button variant=ButtonVariant::Primary>
                "Click me"
            </Button>
        };

        app.dispose();
        // Test assertions here
    }

    #[test]
    fn test_counter_increments() {
        let mut app = leptos::create_runtime();

        let (count, set_count) = create_signal(0);
        let counter = Counter();

        // Simulate button click
        // Test that count increases

        app.dispose();
    }
}
```

## Summary

Leptos components are functions that return `impl IntoView`. Key concepts covered:

1. **Component Definition**: Use `#[component]` macro with `impl IntoView` return type
2. **Props**: Function parameters for component configuration
3. **Composition**: Build complex UIs by combining smaller components
4. **State**: Local state with signals, shared state via props
5. **Lifecycle**: Setup with effects, cleanup with `on_cleanup`
6. **Conditional Rendering**: Use signals and match expressions
7. **List Rendering**: Map over collections with proper keying
8. **Communication**: Callbacks for child-to-parent communication

Best practices:
- Keep components focused and small
- Use meaningful names and prop designs
- Handle errors gracefully
- Test components thoroughly
- Follow consistent patterns

---

**Next:** [03: Advanced Components](03-advanced-components.md) - Learn advanced component patterns like context, portals, and suspense.