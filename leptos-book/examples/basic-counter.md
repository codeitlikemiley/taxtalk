# Basic Counter Example

## Overview

This example demonstrates a simple counter component in Leptos, showcasing the fundamental concepts of reactive state management, event handling, and component composition.

## Complete Implementation

```rust
use leptos::*;

#[component]
pub fn Counter(cx: Scope) -> impl IntoView {
    // Create reactive state
    let (count, set_count) = create_signal(cx, 0);
    
    // Create a derived signal for display
    let count_display = move || {
        if count.get() == 0 {
            "Zero".to_string()
        } else {
            count.get().to_string()
        }
    };

    view! { cx,
        <div class="counter-container">
            <h2>"Counter Example"</h2>
            
            // Display the current count
            <div class="count-display">
                "Count: " {count_display}
            </div>
            
            // Control buttons
            <div class="button-group">
                <button 
                    on:click=move |_| set_count.update(|n| *n -= 1)
                    class="btn btn-secondary"
                >
                    "-"
                </button>
                
                <button 
                    on:click=move |_| set_count.set(0)
                    class="btn btn-warning"
                >
                    "Reset"
                </button>
                
                <button 
                    on:click=move |_| set_count.update(|n| *n += 1)
                    class="btn btn-primary"
                >
                    "+"
                </button>
            </div>
            
            // Status indicator
            <div class="status">
                {move || if count.get() > 0 {
                    view! { cx, <span class="positive">"Positive"</span> }
                } else if count.get() < 0 {
                    view! { cx, <span class="negative">"Negative"</span> }
                } else {
                    view! { cx, <span class="zero">"Zero"</span> }
                }}
            </div>
        </div>
    }
}

// Main App component
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="app">
            <header>
                <h1>"Leptos Counter Demo"</h1>
            </header>
            
            <main>
                <Counter />
            </main>
        </div>
    }
}
```

## Key Concepts Demonstrated

### 1. Reactive State Management

```rust
// Create a signal with initial value
let (count, set_count) = create_signal(cx, 0);
```

- `create_signal` returns a getter and setter pair
- The getter (`count`) is reactive and tracks dependencies
- The setter (`set_count`) updates the value and triggers re-renders

### 2. Derived State

```rust
// Create a computed value based on the count
let count_display = move || {
    if count.get() == 0 {
        "Zero".to_string()
    } else {
        count.get().to_string()
    }
};
```

- Uses a closure that captures the reactive signal
- Automatically re-computes when dependencies change
- Returns a value that can be displayed in the view

### 3. Event Handling

```rust
<button on:click=move |_| set_count.update(|n| *n += 1)>
    "+"
</button>
```

- `on:click` attaches a click event handler
- `move |_|` creates a closure that moves the setter into the handler
- `set_count.update()` modifies the value using a closure

### 4. Conditional Rendering

```rust
{move || if count.get() > 0 {
    view! { cx, <span class="positive">"Positive"</span> }
} else if count.get() < 0 {
    view! { cx, <span class="negative">"Negative"</span> }
} else {
    view! { cx, <span class="zero">"Zero"</span> }
}}
```

- Uses reactive conditionals to show different content
- Each branch returns a `View` that gets rendered
- Automatically updates when the condition changes

## Alternative Implementations

### Using `create_memo` for Derived State

```rust
// More efficient for complex computations
let count_display = create_memo(cx, move |_| {
    match count.get() {
        0 => "Zero".to_string(),
        n if n > 0 => format!("+{}", n),
        n => n.to_string(),
    }
});
```

### Using `create_selector` for Specific Changes

```rust
// Only triggers when count crosses zero
let is_positive = create_selector(cx, move || count.get() > 0);

view! { cx,
    <div class=move || if is_positive.get() { "positive" } else { "non-positive" }>
        {count}
    </div>
}
```

### Batch Updates

```rust
// For multiple related updates
let increment_twice = move || {
    set_count.update(|n| *n += 1);
    set_count.update(|n| *n += 1);
};

// Or use batch
let increment_twice_batch = move || {
    batch(|| {
        set_count.update(|n| *n += 1);
        set_count.update(|n| *n += 1);
    });
};
```

## CSS Styling

```css
.counter-container {
    max-width: 400px;
    margin: 2rem auto;
    padding: 2rem;
    border: 2px solid #e5e7eb;
    border-radius: 8px;
    text-align: center;
}

.count-display {
    font-size: 2rem;
    font-weight: bold;
    margin: 1rem 0;
    color: #1f2937;
}

.button-group {
    display: flex;
    gap: 0.5rem;
    justify-content: center;
    margin: 1rem 0;
}

.btn {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1.2rem;
    transition: background-color 0.2s;
}

.btn-primary {
    background-color: #3b82f6;
    color: white;
}

.btn-primary:hover {
    background-color: #2563eb;
}

.btn-secondary {
    background-color: #6b7280;
    color: white;
}

.btn-secondary:hover {
    background-color: #4b5563;
}

.btn-warning {
    background-color: #f59e0b;
    color: white;
}

.btn-warning:hover {
    background-color: #d97706;
}

.status {
    margin-top: 1rem;
    font-weight: bold;
}

.positive {
    color: #10b981;
}

.negative {
    color: #ef4444;
}

.zero {
    color: #6b7280;
}
```

## Testing the Component

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_counter_increment() {
        // This would require leptos_test or similar testing utilities
        // For now, this shows the testing pattern
        
        leptos::test(|cx| {
            let (count, set_count) = create_signal(cx, 0);
            
            // Test initial state
            assert_eq!(count.get(), 0);
            
            // Test increment
            set_count.set(1);
            assert_eq!(count.get(), 1);
            
            // Test derived state
            let is_positive = move || count.get() > 0;
            assert!(is_positive());
        });
    }
}
```

## Common Patterns and Best Practices

### 1. Signal Naming Convention
```rust
// Good: descriptive names
let (user_name, set_user_name) = create_signal(cx, String::new());
let (is_loading, set_is_loading) = create_signal(cx, false);

// Bad: unclear names
let (value, set_value) = create_signal(cx, 0);
let (flag, set_flag) = create_signal(cx, false);
```

### 2. Event Handler Organization
```rust
// Good: separate handlers for clarity
let handle_increment = move |_| {
    set_count.update(|n| *n += 1);
};

let handle_reset = move |_| {
    set_count.set(0);
};

// Bad: inline complex logic
<button on:click=move |_| {
    set_count.update(|n| *n += 1);
    set_total.update(|t| *t += 1);
    set_last_action.set("increment".to_string());
}>
```

### 3. Component Props
```rust
#[component]
pub fn CounterWithProps(
    cx: Scope,
    initial: i32,
    step: i32,
    #[prop(optional)] max: Option<i32>,
) -> impl IntoView {
    let (count, set_count) = create_signal(cx, initial);
    
    let increment = move |_| {
        set_count.update(|n| {
            let new_val = *n + step;
            if let Some(max_val) = max {
                new_val.min(max_val)
            } else {
                new_val
            }
        });
    };

    // ... rest of component
}
```

### 4. Effect Usage
```rust
// Good: use effects for side effects
create_effect(cx, move |_| {
    let current_count = count.get();
    log::info!("Count changed to: {}", current_count);
    
    // Persist to localStorage
    if let Ok(Some(storage)) = window().local_storage() {
        let _ = storage.set_item("counter", &current_count.to_string());
    }
});

// Bad: don't use effects for derived state
create_effect(cx, move |_| {
    let doubled = count.get() * 2;
    set_doubled.set(doubled); // Use memo instead
});
```

## Performance Considerations

### 1. Minimize Signal Updates
```rust
// Good: single update
let update_user = move || {
    set_name.set(new_name);
    set_email.set(new_email);
    set_age.set(new_age);
};

// Bad: multiple separate updates
let update_name = move |_| set_name.set(new_name);
let update_email = move |_| set_email.set(new_email);
let update_age = move |_| set_age.set(new_age);
```

### 2. Use Memos for Expensive Computations
```rust
// Good: memoized expensive operation
let expensive_result = create_memo(cx, move |_| {
    data.get().iter()
        .filter(|item| item.is_valid())
        .map(|item| item.compute_expensive_value())
        .sum::<f64>()
});
```

### 3. Avoid Unnecessary Re-renders
```rust
// Good: stable references
let items = create_memo(cx, move |_| {
    data.get().into_iter()
        .filter(|item| item.active)
        .collect::<Vec<_>>()
});

// Bad: new vector on every render
let items = move || {
    data.get().into_iter()
        .filter(|item| item.active)
        .collect::<Vec<_>>()
};
```

## Error Handling

```rust
#[component]
pub fn CounterWithErrorHandling(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let (error, set_error) = create_signal(cx, None::<String>);
    
    let increment = move |_| {
        set_count.update(|n| {
            if *n >= i32::MAX - 1 {
                set_error.set(Some("Counter would overflow!".to_string()));
                *n
            } else {
                set_error.set(None);
                *n + 1
            }
        });
    };

    view! { cx,
        <div>
            <div>"Count: " {count}</div>
            
            {move || error.get().map(|err| {
                view! { cx, <div class="error">{err}</div> }
            })}
            
            <button on:click=increment disabled=move || error.get().is_some()>
                "Increment"
            </button>
        </div>
    }
}
```

## Integration with Other Frameworks

### Using with Trunk (Build Tool)
```toml
# Cargo.toml
[package]
name = "counter-example"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = { version = "0.5", features = ["csr"] }
console_log = "1"
log = "0.4"
wasm-bindgen = "0.2"

[features]
hydrate = ["leptos/hydrate"]
ssr = ["leptos/ssr"]
```

### Trunk Configuration
```toml
# Trunk.toml
[build]
target = "index.html"
dist = "dist"
public_url = "/"

[[hooks]]
stage = "pre_build"
command = "echo"
command_arguments = ["Building counter example..."]
```

### HTML Template
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1"/>
    <title>Counter Example</title>
    <link rel="stylesheet" href="/style.css"/>
</head>
<body>
    <div id="leptos"></div>
</body>
</html>
```

## Summary

This basic counter example demonstrates:

- **Reactive State**: Using `create_signal` for state management
- **Derived State**: Creating computed values with closures and memos
- **Event Handling**: Attaching event handlers to DOM elements
- **Conditional Rendering**: Showing different content based on state
- **Component Composition**: Building reusable UI components
- **Styling**: Integrating CSS with Leptos components
- **Performance**: Best practices for efficient updates
- **Error Handling**: Managing edge cases and user feedback

The counter is a fundamental pattern that appears in many applications and serves as an excellent starting point for understanding Leptos' reactive system.