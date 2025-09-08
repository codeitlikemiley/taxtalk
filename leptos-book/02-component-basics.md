# 02: Component Basics

## Overview

Components are the building blocks of Leptos applications. This chapter covers the fundamentals of creating, composing, and managing components in Leptos.

## Component Definition

### Basic Component Structure

```rust
use leptos::*;

// Component function (must return impl IntoView)
#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <div>
            <h1>"Hello, World!"</h1>
            <p>"This is a Leptos component"</p>
        </div>
    }
}
```

### Component with Props

```rust
use leptos::*;

// Define props struct
#[derive(Clone, Debug)]
pub struct PersonProps {
    pub name: String,
    pub age: u32,
}

// Component with props
#[component]
pub fn PersonCard(props: PersonProps) -> impl IntoView {
    view! {
        <div class="person-card">
            <h2>{props.name}</h2>
            <p>"Age: " {props.age}</p>
        </div>
    }
}
```

### Using Components

```rust
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <MyComponent/>
            <PersonCard name="Alice".to_string() age=30/>
            <PersonCard name="Bob".to_string() age=25/>
        </div>
    }
}
```

## Props Patterns

### Optional Props

```rust
#[derive(Clone, Debug)]
pub struct ButtonProps {
    pub children: Children,
    pub on_click: Option<Callback<(), ()>>,
    pub disabled: Option<bool>,
    pub variant: Option<String>,
}

#[component]
pub fn Button(props: ButtonProps) -> impl IntoView {
    let disabled = props.disabled.unwrap_or(false);
    let variant = props.variant.unwrap_or_else(|| "primary".to_string());

    let on_click = move |_| {
        if let Some(callback) = &props.on_click {
            callback.call(());
        }
    };

    view! {
        <button
            class=move || format!("btn btn-{}", variant)
            disabled=disabled
            on:click=on_click
        >
            {props.children()}
        </button>
    }
}
```

### Children Props

```rust
#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="card">
            <div class="card-content">
                {children()}
            </div>
        </div>
    }
}

// Usage
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Card>
            <h1>"Card Title"</h1>
            <p>"Card content goes here"</p>
        </Card>
    }
}
```

### Generic Props

```rust
#[derive(Clone, Debug)]
pub struct ListProps<T: Clone + 'static> {
    pub items: Vec<T>,
    pub render_item: Callback<T, View>,
}

#[component]
pub fn List<T: Clone + 'static>(props: ListProps<T>) -> impl IntoView {
    view! {
        <ul>
            {props.items.into_iter()
                .map(|item| props.render_item.call(item))
                .collect::<Vec<_>>()}
        </ul>
    }
}
```

## Component State

### Local State

```rust
#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div>
            <p>"Count: " {count}</p>
            <button on:click=move |_| set_count.update(|n| *n + 1)>
                "Increment"
            </button>
            <button on:click=move |_| set_count.update(|n| *n - 1)>
                "Decrement"
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

    // Derived state: count of completed todos
    let completed_count = create_memo(move |_| {
        todos().iter().filter(|todo| todo.completed).count()
    });

    // Derived state: total todos
    let total_count = create_memo(move |_| todos().len());

    view! {
        <div>
            <h1>"Todo List"</h1>
            <p>"Completed: " {completed_count} " / " {total_count}</p>
            // ... todo items
        </div>
    }
}
```

## Event Handling

### Basic Events

```rust
#[component]
pub fn Form() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());

    view! {
        <div>
            <input
                type="text"
                value=name
                on:input=move |ev| set_name(event_target_value(&ev))
            />
            <p>"Hello, " {name} "!"</p>
        </div>
    }
}
```

### Custom Event Handlers

```rust
#[component]
pub fn CustomButton() -> impl IntoView {
    let (click_count, set_click_count) = create_signal(0);

    let handle_click = move |_| {
        set_click_count.update(|n| *n + 1);
        logging::log!("Button clicked! Count: {}", click_count());
    };

    view! {
        <button on:click=handle_click>
            "Clicked " {click_count} " times"
        </button>
    }
}
```

### Event Delegation

```rust
#[component]
pub fn TodoItem(
    todo: Todo,
    on_toggle: Callback<(), ()>,
    on_delete: Callback<(), ()>
) -> impl IntoView {
    view! {
        <div class="todo-item">
            <input
                type="checkbox"
                checked=todo.completed
                on:change=move |_| on_toggle.call(())
            />
            <span class=move || if todo.completed { "completed" } else { "" }>
                {todo.text}
            </span>
            <button on:click=move |_| on_delete.call(())>
                "Delete"
            </button>
        </div>
    }
}
```

## Component Lifecycle

### Component Creation

```rust
#[component]
pub fn LifecycleDemo() -> impl IntoView {
    // This runs when component is created
    logging::log!("Component created");

    // Cleanup when component is destroyed
    on_cleanup(|| {
        logging::log!("Component destroyed");
    });

    view! {
        <div>"Lifecycle Demo"</div>
    }
}
```

### Effect Lifecycle

```rust
#[component]
pub fn EffectDemo() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    // Effect runs when count changes
    create_effect(move |_| {
        logging::log!("Count changed to: {}", count());
    });

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

## Component Composition

### Container/Presentational Pattern

```rust
// Presentational component
#[component]
pub fn TodoListView(todos: Vec<Todo>, on_toggle: Callback<usize, ()>) -> impl IntoView {
    view! {
        <ul>
            {todos.into_iter().enumerate().map(|(i, todo)| {
                view! {
                    <li>
                        <input
                            type="checkbox"
                            checked=todo.completed
                            on:change=move |_| on_toggle.call(i)
                        />
                        <span>{todo.text}</span>
                    </li>
                }
            }).collect::<Vec<_>>()}
        </ul>
    }
}

// Container component
#[component]
pub fn TodoApp() -> impl IntoView {
    let (todos, set_todos) = create_signal(vec![
        Todo { id: 1, text: "Learn Leptos".to_string(), completed: false },
    ]);

    let toggle_todo = move |index| {
        set_todos.update(|todos| {
            if let Some(todo) = todos.get_mut(index) {
                todo.completed = !todo.completed;
            }
        });
    };

    view! {
        <div>
            <h1>"Todo App"</h1>
            <TodoListView todos=todos() on_toggle=toggle_todo/>
        </div>
    }
}
```

### Higher-Order Components

```rust
#[component]
pub fn WithLoading<T: 'static>(
    children: Children,
    loading: ReadSignal<bool>
) -> impl IntoView {
    view! {
        <Show
            when=move || !loading()
            fallback=|| view! { <div>"Loading..."</div> }
        >
            {children()}
        </Show>
    }
}

// Usage
#[component]
pub fn App() -> impl IntoView {
    let (loading, set_loading) = create_signal(true);

    // Simulate loading
    set_timeout(move || set_loading(false), 2000);

    view! {
        <WithLoading loading=loading>
            <div>"Content loaded!"</div>
        </WithLoading>
    }
}
```

## Conditional Rendering

### Show Component

```rust
#[component]
pub fn UserProfile(user: Option<User>) -> impl IntoView {
    view! {
        <Show
            when=move || user.is_some()
            fallback=|| view! { <div>"No user selected"</div> }
        >
            {move || user.as_ref().map(|u| {
                view! {
                    <div>
                        <h1>{&u.name}</h1>
                        <p>{&u.email}</p>
                    </div>
                }
            })}
        </Show>
    }
}
```

### Conditional Classes

```rust
#[component]
pub fn Button(
    #[prop(default = false)] disabled: bool,
    children: Children
) -> impl IntoView {
    view! {
        <button
            class=move || if disabled { "btn btn-disabled" } else { "btn btn-primary" }
            disabled=disabled
        >
            {children()}
        </button>
    }
}
```

## Lists and Keys

### Basic Lists

```rust
#[component]
pub fn ItemList() -> impl IntoView {
    let (items, set_items) = create_signal(vec!["Apple", "Banana", "Cherry"]);

    view! {
        <ul>
            {move || items().into_iter().map(|item| {
                view! { <li>{item}</li> }
            }).collect::<Vec<_>>()}
        </ul>
    }
}
```

### Lists with Keys

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Todo {
    pub id: usize,
    pub text: String,
    pub completed: bool,
}

#[component]
pub fn TodoList() -> impl IntoView {
    let (todos, set_todos) = create_signal(vec![
        Todo { id: 1, text: "Learn Leptos".to_string(), completed: false },
        Todo { id: 2, text: "Build app".to_string(), completed: false },
    ]);

    view! {
        <ul>
            <For
                each=move || todos()
                key=|todo| todo.id
                children=move |todo| {
                    view! {
                        <li>
                            <input
                                type="checkbox"
                                checked=todo.completed
                                on:change=move |_| {
                                    set_todos.update(|todos| {
                                        if let Some(t) = todos.iter_mut().find(|t| t.id == todo.id) {
                                            t.completed = !t.completed;
                                        }
                                    });
                                }
                            />
                            <span>{todo.text}</span>
                        </li>
                    }
                }
            />
        </ul>
    }
}
```

## Component Communication

### Parent to Child (Props)

```rust
#[derive(Clone, Debug)]
pub struct ThemeProps {
    pub theme: String,
    pub children: Children,
}

#[component]
pub fn ThemeProvider(props: ThemeProps) -> impl IntoView {
    provide_context(props.theme);

    props.children()
}

#[component]
pub fn ThemedButton() -> impl IntoView {
    let theme = use_context::<String>().unwrap_or_else(|| "light".to_string());

    view! {
        <button class=move || format!("btn btn-{}", theme)>
            "Themed Button"
        </button>
    }
}
```

### Child to Parent (Callbacks)

```rust
#[component]
pub fn CounterWithCallback(
    initial: i32,
    on_count_change: Callback<i32, ()>
) -> impl IntoView {
    let (count, set_count) = create_signal(initial);

    let increment = move |_| {
        set_count.update(|n| *n + 1);
        on_count_change.call(count());
    };

    view! {
        <div>
            <p>"Count: " {count}</p>
            <button on:click=increment>"Increment"</button>
        </div>
    }
}
```

## Error Boundaries

### Basic Error Handling

```rust
#[component]
pub fn ErrorBoundary(children: Children) -> impl IntoView {
    let (error, set_error) = create_signal(None::<String>);

    // Catch panics in child components
    view! {
        <ErrorBoundary error=error>
            <div>
                {children()}
            </div>
        </ErrorBoundary>
    }
}
```

## Performance Patterns

### Memoization

```rust
#[component]
pub fn ExpensiveList(items: Vec<String>) -> impl IntoView {
    // Only re-compute when items change
    let processed_items = create_memo(move |_| {
        items.iter()
            .map(|item| item.to_uppercase())
            .collect::<Vec<_>>()
    });

    view! {
        <ul>
            {move || processed_items().into_iter().map(|item| {
                view! { <li>{item}</li> }
            }).collect::<Vec<_>>()}
        </ul>
    }
}
```

### Avoiding Unnecessary Re-renders

```rust
#[component]
pub fn OptimizedComponent() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (name, set_name) = create_signal("Alice".to_string());

    // This will only re-render when count changes
    let display_text = create_memo(move |_| {
        format!("{}: {}", name(), count())
    });

    view! {
        <div>
            <p>{display_text}</p>
            <button on:click=move |_| set_count.update(|n| *n + 1)>
                "Increment"
            </button>
            <input
                value=name
                on:input=move |ev| set_name(event_target_value(&ev))
            />
        </div>
    }
}
```

## Testing Components

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_counter_increments() {
        let (count, set_count) = create_signal(0);

        set_count.update(|n| *n + 1);

        assert_eq!(count(), 1);
    }

    #[test]
    fn test_component_renders() {
        let result = Counter();

        // Test that component renders without panicking
        assert!(true); // Placeholder
    }
}
```

### Integration Testing

```rust
#[test]
fn test_button_click() {
    let mut container = mount_to_body(|| {
        view! { <Counter/> }
    });

    // Find button and click it
    let button = container.find(By::Role(Role::Button)).unwrap();
    button.click();

    // Check that count increased
    let count_text = container.find(By::Text("Count: 1")).unwrap();
    assert!(count_text.exists());
}
```

## Best Practices

1. **Keep components small and focused**: Each component should have a single responsibility
2. **Use descriptive prop names**: Make component APIs clear and self-documenting
3. **Provide sensible defaults**: Use `#[prop(default = ...)]` for optional props
4. **Handle loading and error states**: Always consider edge cases
5. **Use callbacks for child-to-parent communication**: Avoid tight coupling
6. **Memoize expensive computations**: Use `create_memo` for derived state
7. **Use keys with dynamic lists**: Help Leptos optimize re-renders
8. **Clean up resources**: Use `on_cleanup` for subscriptions and timers

## Common Patterns

### Controlled Components

```rust
#[component]
pub fn ControlledInput() -> impl IntoView {
    let (value, set_value) = create_signal("".to_string());

    view! {
        <input
            type="text"
            value=value
            on:input=move |ev| set_value(event_target_value(&ev))
        />
    }
}
```

### Uncontrolled Components

```rust
#[component]
pub fn UncontrolledInput() -> impl IntoView {
    let input_ref = create_node_ref::<html::Input>();

    let get_value = move || {
        input_ref.get().unwrap().value()
    };

    view! {
        <input type="text" node_ref=input_ref/>
        <button on:click=move |_| logging::log!("Value: {}", get_value())>
            "Log Value"
        </button>
    }
}
```

### Form Components

```rust
#[component]
pub fn LoginForm() -> impl IntoView {
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    let submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        // Handle form submission
        logging::log!("Login: {} / {}", email(), password());
    };

    view! {
        <form on:submit=submit>
            <input
                type="email"
                placeholder="Email"
                value=email
                on:input=move |ev| set_email(event_target_value(&ev))
            />
            <input
                type="password"
                placeholder="Password"
                value=password
                on:input=move |ev| set_password(event_target_value(&ev))
            />
            <button type="submit">"Login"</button>
        </form>
    }
}
```

## Summary

Leptos components are functions that return `impl IntoView`. Key concepts include:

- **Props**: Data passed from parent to child components
- **State**: Local component state using signals
- **Events**: User interactions handled with event listeners
- **Composition**: Building complex UIs from smaller components
- **Lifecycle**: Component creation, updates, and cleanup
- **Performance**: Memoization and efficient re-rendering

Mastering these patterns will enable you to build maintainable, performant Leptos applications.