# 01: Reactive Fundamentals

## Overview

Leptos is built on fine-grained reactive programming, where the framework automatically tracks dependencies between reactive values and updates the UI when those values change. Understanding signals, effects, and memos is crucial for effective Leptos development.

## Core Reactive Concepts

### Signals: Reactive Values

Signals are the fundamental building blocks of reactivity in Leptos. They represent values that can change over time and automatically notify dependent computations when they update.

**Basic Signal Creation:**
```rust
use leptos::*;

// Create a signal with initial value
let (count, set_count) = create_signal(0);

// Read the current value
let current_value = count.get();

// Update the value
set_count.set(5);

// Update based on current value
set_count.update(|n| *n + 1);
```

**Signal Types:**
- **Read Signal**: `count` - provides read-only access to the value
- **Write Signal**: `set_count` - provides write-only access to update the value
- **Combined**: `(count, set_count)` - tuple providing both read and write access

### Effects: Reactive Side Effects

Effects run whenever their reactive dependencies change. They're perfect for side effects like logging, DOM manipulation, or API calls.

**Basic Effect:**
```rust
use leptos::*;

// Effect that runs when count changes
create_effect(move |_| {
    let current_count = count.get();
    log::info!("Count changed to: {}", current_count);
});
```

**Effect with Multiple Dependencies:**
```rust
let (name, set_name) = create_signal("Alice".to_string());
let (age, set_age) = create_signal(25);

create_effect(move |_| {
    let current_name = name.get();
    let current_age = age.get();
    log::info!("{} is {} years old", current_name, current_age);
});
```

### Memos: Cached Computations

Memos are computed values that cache their result and only recalculate when their dependencies change. They're essential for performance optimization.

**Basic Memo:**
```rust
use leptos::*;

// Memo that computes double the count
let double_count = create_memo(move |_| {
    let count_val = count.get();
    count_val * 2
});

// Use the memo in effects or views
create_effect(move |_| {
    log::info!("Double count: {}", double_count.get());
});
```

**Memo with Complex Computation:**
```rust
let (items, set_items) = create_signal(vec![1, 2, 3, 4, 5]);

// Memo that computes expensive operation
let sum = create_memo(move |_| {
    let items_val = items.get();
    items_val.iter().sum::<i32>() // Expensive operation
});

// Memo that depends on another memo
let average = create_memo(move |_| {
    let total = sum.get();
    let count = items.get().len();
    if count > 0 {
        total as f64 / count as f64
    } else {
        0.0
    }
});
```

## Reactive Patterns

### Derived State

Create new reactive values derived from existing signals:

```rust
let (first_name, set_first_name) = create_signal("John".to_string());
let (last_name, set_last_name) = create_signal("Doe".to_string());

// Derived signal for full name
let full_name = create_memo(move |_| {
    format!("{} {}", first_name.get(), last_name.get())
});

// Use in component
view! {
    <div>
        <p>"Full Name: " {full_name}</p>
    </div>
}
```

### Conditional Reactivity

Use signals to control when effects run:

```rust
let (enabled, set_enabled) = create_signal(true);
let (count, set_count) = create_signal(0);

// Effect that only runs when enabled
create_effect(move |_| {
    if enabled.get() {
        let current_count = count.get();
        log::info!("Count: {}", current_count);
    }
});
```

### Reactive Collections

Handle reactive arrays and collections:

```rust
let (todos, set_todos) = create_signal(vec![
    Todo { id: 1, text: "Learn Leptos".to_string(), completed: false },
    Todo { id: 2, text: "Build app".to_string(), completed: false },
]);

// Memo for completed todos
let completed_todos = create_memo(move |_| {
    todos.get()
        .into_iter()
        .filter(|todo| todo.completed)
        .collect::<Vec<_>>()
});

// Memo for completion percentage
let completion_percentage = create_memo(move |_| {
    let all = todos.get().len();
    let completed = completed_todos.get().len();
    if all > 0 {
        (completed as f64 / all as f64) * 100.0
    } else {
        0.0
    }
});
```

## Advanced Reactive Patterns

### Signal Batching

Batch multiple signal updates to avoid unnecessary re-renders:

```rust
// Without batching - causes multiple re-renders
set_count.set(1);
set_name.set("Alice".to_string());
set_age.set(30);

// With batching - single re-render
batch(|| {
    set_count.set(1);
    set_name.set("Alice".to_string());
    set_age.set(30);
});
```

### Selective Updates

Use `update` method for conditional updates:

```rust
let (count, set_count) = create_signal(0);

// Only increment if count is even
set_count.update(|n| {
    if *n % 2 == 0 {
        *n + 1
    } else {
        *n
    }
});
```

### Reactive Cleanup

Clean up resources when signals are no longer needed:

```rust
let (interval_id, set_interval_id) = create_signal(None);

create_effect(move |_| {
    let current_count = count.get();

    // Clean up previous interval
    if let Some(id) = interval_id.get() {
        // Clean up logic here
    }

    // Set up new interval
    let new_id = set_interval(move || {
        log::info!("Count is: {}", current_count);
    }, 1000);

    set_interval_id.set(Some(new_id));
});
```

## Reactive Gotchas

### Common Mistakes

**1. Reading signals in wrong context:**
```rust
// ❌ Wrong - reading signal outside reactive context
let current_count = count.get();
create_effect(move |_| {
    // This will always log the same value
    log::info!("Count: {}", current_count);
});

// ✅ Correct - read signal inside effect
create_effect(move |_| {
    let current_count = count.get();
    log::info!("Count: {}", current_count);
});
```

**2. Creating effects in loops:**
```rust
// ❌ Wrong - creates effect on every render
let items = vec![1, 2, 3];
for item in items {
    create_effect(move |_| {
        log::info!("Item: {}", item);
    });
}

// ✅ Correct - use single effect with iteration
create_effect(move |_| {
    for item in &items {
        log::info!("Item: {}", item);
    }
});
```

**3. Forgetting to move closures:**
```rust
// ❌ Wrong - closure doesn't capture variables
create_effect(|_| {
    let value = count.get(); // Won't work
    log::info!("Value: {}", value);
});

// ✅ Correct - use move closure
create_effect(move |_| {
    let value = count.get();
    log::info!("Value: {}", value);
});
```

### Performance Considerations

**1. Memoization Strategy:**
```rust
// ✅ Good - memoize expensive computations
let expensive_result = create_memo(move |_| {
    let data = large_dataset.get();
    // Expensive computation
    data.iter().map(|x| x * 2).sum()
});

// ❌ Bad - recompute on every access
fn get_expensive_result() -> i32 {
    let data = large_dataset.get();
    data.iter().map(|x| x * 2).sum()
}
```

**2. Effect Dependencies:**
```rust
// ✅ Good - effect only runs when needed
create_effect(move |_| {
    let search_term = search.get();
    let results = perform_search(&search_term);
    set_results.set(results);
});

// ❌ Bad - effect runs on every change
create_effect(move |_| {
    let _ = count.get(); // Unnecessary dependency
    let _ = name.get();  // Unnecessary dependency
    let search_term = search.get();
    let results = perform_search(&search_term);
    set_results.set(results);
});
```

## Reactive Testing

### Testing Reactive Code

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_signal_updates() {
        let (count, set_count) = create_signal(0);

        // Test initial value
        assert_eq!(count.get(), 0);

        // Test update
        set_count.set(5);
        assert_eq!(count.get(), 5);

        // Test update function
        set_count.update(|n| *n + 1);
        assert_eq!(count.get(), 6);
    }

    #[test]
    fn test_memo_computation() {
        let (count, set_count) = create_signal(2);
        let double = create_memo(move |_| count.get() * 2);

        assert_eq!(double.get(), 4);

        set_count.set(5);
        assert_eq!(double.get(), 10);
    }
}
```

### Testing Effects

```rust
#[test]
fn test_effect_execution() {
    let (count, set_count) = create_signal(0);
    let mut effect_runs = 0;

    create_effect(move |_| {
        let _ = count.get(); // Read to create dependency
        effect_runs += 1;
    });

    // Effect should run once initially
    assert_eq!(effect_runs, 1);

    // Effect should run again when count changes
    set_count.set(1);
    assert_eq!(effect_runs, 2);
}
```

## Best Practices

### Signal Organization

**1. Group related signals:**
```rust
// ✅ Good - related signals together
struct UserState {
    name: ReadSignal<String>,
    email: ReadSignal<String>,
    age: ReadSignal<i32>,
}

// ❌ Bad - scattered signals
let (name, set_name) = create_signal(String::new());
let (email, set_email) = create_signal(String::new());
let (age, set_age) = create_signal(0);
```

**2. Use meaningful names:**
```rust
// ✅ Good
let (is_loading, set_is_loading) = create_signal(false);
let (error_message, set_error_message) = create_signal(None::<String>);

// ❌ Bad
let (flag, set_flag) = create_signal(false);
let (msg, set_msg) = create_signal(None::<String>);
```

### Effect Management

**1. Clean up effects:**
```rust
create_effect(move |_| {
    let cleanup = setup_event_listener();

    on_cleanup(move || {
        cleanup(); // Clean up when effect re-runs
    });
});
```

**2. Avoid effect chains:**
```rust
// ✅ Good - single effect handles multiple dependencies
create_effect(move |_| {
    let count = count_signal.get();
    let name = name_signal.get();
    update_display(count, &name);
});

// ❌ Bad - effect triggers another effect
create_effect(move |_| {
    let count = count_signal.get();
    set_display_count.set(count);
});

create_effect(move |_| {
    let display_count = display_count_signal.get();
    let name = name_signal.get();
    update_display(display_count, &name);
});
```

### Performance Optimization

**1. Use memos for expensive computations:**
```rust
let expensive_result = create_memo(move |_| {
    let data = large_dataset.get();
    // Expensive computation that should be cached
    process_data(&data)
});
```

**2. Debounce rapid updates:**
```rust
let (search_term, set_search_term) = create_signal(String::new());

let debounced_search = create_memo(move |_| {
    let term = search_term.get();
    // Debounce logic here
    term
});
```

## Summary

Reactive programming in Leptos revolves around three core concepts:

1. **Signals** - Reactive values that notify dependents of changes
2. **Effects** - Side effects that run when dependencies change
3. **Memos** - Cached computed values for performance

Key principles:
- Always read signals inside reactive contexts (effects, memos, components)
- Use `move` closures to capture variables properly
- Memoize expensive computations
- Batch related updates
- Clean up resources when no longer needed

Understanding these fundamentals is essential for building efficient, maintainable Leptos applications. The reactive system handles most optimization automatically, but following these patterns ensures optimal performance and correct behavior.

---

**Next:** [02: Component Basics](02-component-basics.md) - Learn how to create and compose Leptos components.