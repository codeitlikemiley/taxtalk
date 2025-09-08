# 01: Reactive Fundamentals

## Overview

Leptos is built on a fine-grained reactive system that automatically tracks dependencies and updates the UI when data changes. This chapter covers the core reactive primitives: signals, effects, and memos.

## Signals

Signals are the fundamental reactive primitive in Leptos. They hold values that can change over time and automatically notify dependents when updated.

### Creating Signals

```rust
use leptos::*;

// Create a signal with initial value
let (count, set_count) = create_signal(0);

// Read the current value
let current_count = count();

// Update the value
set_count(5);

// Update based on current value
set_count.update(|n| *n + 1);
```

### Signal Types

#### Read Signals
```rust
let (count, _) = create_signal(0);
// count is ReadSignal<i32>
let value = count(); // Read current value
```

#### Write Signals
```rust
let (_, set_count) = create_signal(0);
// set_count is WriteSignal<i32>
set_count(5); // Set new value
```

#### Combined Signals
```rust
let (count, set_count) = create_signal(0);
// count: ReadSignal<i32>
// set_count: WriteSignal<i32>
```

### Signal Updates

#### Direct Assignment
```rust
set_count(10);
```

#### Functional Updates
```rust
set_count.update(|n| *n + 1);
```

#### Conditional Updates
```rust
if some_condition {
    set_count(0);
}
```

## Effects

Effects run when their reactive dependencies change. They're used for side effects like logging, DOM manipulation, or API calls.

### Creating Effects

```rust
use leptos::*;

// Simple effect
create_effect(move |_| {
    logging::log!("Count changed to: {}", count());
});

// Effect with cleanup
create_effect(move |_| {
    let cleanup = setup_some_resource();
    on_cleanup(cleanup);
});
```

### Effect Dependencies

Effects automatically track which signals they read:

```rust
let (count, set_count) = create_signal(0);
let (name, set_name) = create_signal("Alice".to_string());

create_effect(move |_| {
    // This effect depends on both count and name
    logging::log!("{} has count: {}", name(), count());
});

// Only triggers when count changes
create_effect(move |_| {
    logging::log!("Count: {}", count());
});

// Only triggers when name changes
create_effect(move |_| {
    logging::log!("Name: {}", name());
});
```

### Cleanup Functions

```rust
create_effect(move |_| {
    // Setup
    let interval_id = set_interval(|| {
        logging::log!("Tick");
    }, 1000);

    // Cleanup when effect re-runs or component unmounts
    on_cleanup(move || {
        clear_interval(interval_id);
    });
});
```

## Memos

Memos are computed values that cache expensive calculations and only re-compute when dependencies change.

### Creating Memos

```rust
use leptos::*;

// Simple memo
let doubled = create_memo(move |_| count() * 2);

// Memo with complex computation
let expensive_value = create_memo(move |_| {
    // Expensive calculation here
    fibonacci(count())
});
```

### Memo Usage

```rust
#[component]
pub fn CounterDisplay() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let doubled = create_memo(move |_| count() * 2);

    view! {
        <div>
            <p>"Count: " {count}</p>
            <p>"Doubled: " {doubled}</p>
            <button on:click=move |_| set_count.update(|n| *n + 1)>
                "Increment"
            </button>
        </div>
    }
}
```

## Reactive Patterns

### Derived State

```rust
let (first_name, set_first_name) = create_signal("John".to_string());
let (last_name, set_last_name) = create_signal("Doe".to_string());

// Derived signal
let full_name = create_memo(move |_| {
    format!("{} {}", first_name(), last_name())
});
```

### Conditional Reactivity

```rust
let (is_logged_in, set_is_logged_in) = create_signal(false);
let (user_data, set_user_data) = create_signal(None::<User>);

// Only fetch when logged in
create_effect(move |_| {
    if is_logged_in() {
        // Fetch user data
        spawn_local(async move {
            let user = fetch_user().await;
            set_user_data(Some(user));
        });
    } else {
        set_user_data(None);
    }
});
```

### Reactive Collections

```rust
let (items, set_items) = create_signal(vec![1, 2, 3]);

// Add item
let add_item = move |new_item| {
    set_items.update(|items| items.push(new_item));
};

// Remove item
let remove_item = move |index| {
    set_items.update(|items| {
        items.remove(index);
    });
};
```

## Advanced Signal Patterns

### Signal Mapping

```rust
let (count, set_count) = create_signal(0);

// Create a derived signal that transforms the value
let count_string = create_memo(move |_| count().to_string());

// Create a signal that filters values
let even_count = create_memo(move |_| {
    let c = count();
    if c % 2 == 0 { Some(c) } else { None }
});
```

### Signal Composition

```rust
// Combine multiple signals
let combined = create_memo(move |_| {
    format!("{}: {}", name(), count())
});

// Chain computations
let processed = create_memo(move |_| {
    let base = count();
    let doubled = base * 2;
    let squared = doubled * doubled;
    squared
});
```

### Batching Updates

```rust
// Batch multiple updates
batch(|| {
    set_count(1);
    set_name("Alice".to_string());
    set_active(true);
});
```

## Error Handling in Reactive Code

### Handling Errors in Effects

```rust
create_effect(move |_| {
    match fetch_data().await {
        Ok(data) => set_data(data),
        Err(err) => set_error(err.to_string()),
    }
});
```

### Fallback Values

```rust
let data = create_resource(|| (), |()| async move {
    fetch_data().await.unwrap_or_default()
});
```

## Performance Considerations

### Minimizing Re-computations

```rust
// Good: Memo caches the expensive computation
let expensive = create_memo(move |_| {
    expensive_calculation(count())
});

// Avoid: Re-computes on every render
let expensive = move || expensive_calculation(count());
```

### Effect Cleanup

```rust
create_effect(move |_| {
    let subscription = subscribe_to_updates();
    on_cleanup(|| {
        subscription.unsubscribe();
    });
});
```

### Signal Granularity

```rust
// Good: Separate signals for different concerns
let (name, set_name) = create_signal("John".to_string());
let (age, set_age) = create_signal(25);

// Avoid: Single signal with complex object
let (user, set_user) = create_signal(User {
    name: "John".to_string(),
    age: 25,
});
```

## Common Patterns

### Loading States

```rust
let (is_loading, set_is_loading) = create_signal(false);
let (data, set_data) = create_signal(None);

create_effect(move |_| {
    if is_loading() {
        // Show loading spinner
    } else if let Some(data) = data() {
        // Show data
    } else {
        // Show empty state
    }
});
```

### Debounced Input

```rust
let (input, set_input) = create_signal("".to_string());
let debounced_input = create_memo(move |_| {
    let value = input();
    // Debounce logic here
    value
});

create_effect(move |_| {
    let search_term = debounced_input();
    // Perform search
});
```

### Optimistic Updates

```rust
let (count, set_count) = create_signal(0);

let increment = move |_| {
    // Optimistic update
    set_count.update(|n| *n + 1);

    // Send to server
    spawn_local(async move {
        match update_server(count()).await {
            Ok(_) => {}, // Success
            Err(_) => {
                // Revert on error
                set_count.update(|n| *n - 1);
            }
        }
    });
};
```

## Testing Reactive Code

### Testing Signals

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_signal_updates() {
        let (count, set_count) = create_signal(0);

        assert_eq!(count(), 0);

        set_count(5);
        assert_eq!(count(), 5);

        set_count.update(|n| *n + 1);
        assert_eq!(count(), 6);
    }
}
```

### Testing Effects

```rust
#[test]
    fn test_effect_runs() {
        let (count, set_count) = create_signal(0);
        let mut effect_ran = false;

        create_effect(move |_| {
            let _ = count(); // Read signal
            effect_ran = true;
        });

        // Trigger effect
        set_count(1);

        // In real testing, you'd wait for effect to run
        assert!(effect_ran);
    }
```

## Best Practices

1. **Keep signals focused**: Each signal should represent one piece of state
2. **Use memos for expensive computations**: Cache results to avoid re-computation
3. **Clean up effects**: Always use `on_cleanup` for resources
4. **Batch updates**: Use `batch()` for multiple related updates
5. **Handle errors gracefully**: Don't let errors crash your reactive system
6. **Test reactive logic**: Unit test your signals and effects

## Migration from Other Frameworks

### From React

```javascript
// React
const [count, setCount] = useState(0);
const doubled = useMemo(() => count * 2, [count]);

// Leptos
let (count, set_count) = create_signal(0);
let doubled = create_memo(move |_| count() * 2);
```

### From Vue

```javascript
// Vue
const count = ref(0);
const doubled = computed(() => count.value * 2);

// Leptos
let (count, set_count) = create_signal(0);
let doubled = create_memo(move |_| count() * 2);
```

### From Svelte

```javascript
// Svelte
let count = 0;
$: doubled = count * 2;

// Leptos
let (count, set_count) = create_signal(0);
let doubled = create_memo(move |_| count() * 2);
```

## Summary

Leptos' reactive system provides:
- **Signals**: Reactive values that notify dependents
- **Effects**: Side effects that run when dependencies change
- **Memos**: Cached computed values
- **Automatic dependency tracking**: No manual dependency arrays
- **Fine-grained reactivity**: Only re-compute what's necessary

Mastering these primitives is essential for building efficient, reactive Leptos applications.