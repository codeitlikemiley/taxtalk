# Signals Cheatsheet

## Core Signal Types

### ReadSignal<T>
```rust
let (read, write) = create_signal(cx, initial_value);

// Reading
let current_value = read.get();

// Reactive reading in view
view! { cx, <p>{read}</p> }

// Conditional reading
if read.get() > 0 { /* ... */ }
```

### WriteSignal<T>
```rust
let (read, write) = create_signal(cx, initial_value);

// Writing
write.set(new_value);

// Updating based on current value
write.update(|current| *current += 1);

// Conditional update
write.set(if condition { value_a } else { value_b });
```

### RwSignal<T>
```rust
let signal = create_rw_signal(cx, initial_value);

// Reading
let current = signal.get();

// Writing
signal.set(new_value);

// Updating
signal.update(|current| *current += 1);

// Direct access (for complex mutations)
signal.with(|current| {
    // Access current value without cloning
    current.do_something();
});
```

## Signal Creation Patterns

### Basic Signal
```rust
let (count, set_count) = create_signal(cx, 0);
```

### Computed Signal (Memo)
```rust
let doubled = create_memo(cx, move |_| count.get() * 2);

// Or with dependencies
let computed = create_memo(cx, move |_| {
    let a = signal_a.get();
    let b = signal_b.get();
    expensive_computation(a, b)
});
```

### Effect
```rust
create_effect(cx, move |_| {
    let value = signal.get();
    log::info!("Signal changed to: {}", value);
    // Side effects here
});
```

### Resource (Async)
```rust
let resource = create_resource(cx,
    move || id.get(), // Source signal
    |id| async move { // Fetcher function
        fetch_data(id).await
    }
);

// Usage
match resource.read(cx) {
    Some(Ok(data)) => view! { cx, <div>{data}</div> },
    Some(Err(err)) => view! { cx, <div>{"Error: "}{err}</div> },
    None => view! { cx, <div>"Loading..."</div> },
}
```

## Signal Composition

### Signal from Signal
```rust
let derived = create_memo(cx, move |_| {
    let base = base_signal.get();
    transform(base)
});
```

### Multiple Signals
```rust
let combined = create_memo(cx, move |_| {
    let a = signal_a.get();
    let b = signal_b.get();
    let c = signal_c.get();
    combine(a, b, c)
});
```

### Signal with Cleanup
```rust
let (signal, set_signal) = create_signal(cx, initial);

create_effect(cx, move |_| {
    let value = signal.get();
    // Setup side effect

    on_cleanup(cx, || {
        // Cleanup side effect
    });
});
```

## Advanced Patterns

### Signal with Validation
```rust
let (input, set_input) = create_signal(cx, String::new());
let validated = create_memo(cx, move |_| {
    let value = input.get();
    if value.len() > 3 {
        Some(value)
    } else {
        None
    }
});
```

### Debounced Signal
```rust
let (input, set_input) = create_signal(cx, String::new());
let debounced = create_memo(cx, move |_| {
    let value = input.get();
    set_timeout(move || {}, 300); // Simple debounce
    value
});
```

### Signal with History
```rust
let (current, set_current) = create_signal(cx, initial);
let history = create_rw_signal(cx, Vec::new());

create_effect(cx, move |_| {
    let value = current.get();
    history.update(|h| h.push(value));
});
```

## Common Patterns

### Toggle Signal
```rust
let (is_open, set_is_open) = create_signal(cx, false);

// Toggle
set_is_open.update(|open| !*open);

// Or
let toggle = move |_| set_is_open.update(|open| !*open);
```

### Counter Pattern
```rust
let (count, set_count) = create_signal(cx, 0);

let increment = move |_| set_count.update(|c| *c += 1);
let decrement = move |_| set_count.update(|c| *c -= 1);
let reset = move |_| set_count.set(0);
```

### Form State
```rust
#[derive(Clone, Debug)]
struct FormData {
    name: String,
    email: String,
    age: u32,
}

let (form, set_form) = create_signal(cx, FormData {
    name: String::new(),
    email: String::new(),
    age: 0,
});

// Update individual fields
let update_name = move |name| {
    set_form.update(|f| f.name = name);
};
```

## Signal Best Practices

### ✅ Do
```rust
// Use memos for expensive computations
let expensive = create_memo(cx, move |_| {
    let data = signal.get();
    expensive_calculation(data)
});

// Use effects for side effects only
create_effect(cx, move |_| {
    let value = signal.get();
    update_dom(value);
});

// Clone signals when needed
let signal_clone = signal;
```

### ❌ Don't
```rust
// Don't create signals in loops
for i in 0..10 {
    let (signal, _) = create_signal(cx, i); // Bad!
}

// Don't read signals unnecessarily
let value = signal.get(); // Only read when needed
let same_value = signal.get(); // Redundant

// Don't mutate signals in render
view! { cx,
    <button on:click=move |_| {
        signal.set(signal.get() + 1); // OK
        signal.update(|x| *x += 1); // Better
    }>
        {signal.get()} // Reading in render is OK
    </button>
}
```

## Performance Tips

### Minimize Signal Reads
```rust
// ✅ Good: Read once
let value = signal.get();
view! { cx,
    <div>{value}</div>
    <span>{value}</span>
}

// ❌ Bad: Read multiple times
view! { cx,
    <div>{signal.get()}</div>
    <span>{signal.get()}</span>
}
```

### Use Memos for Derived State
```rust
// ✅ Good: Memoized computation
let display_name = create_memo(cx, move |_| {
    let user = user_signal.get();
    format!("{} {}", user.first_name, user.last_name)
});

// ❌ Bad: Recomputing in render
view! { cx,
    <div>
        {format!("{} {}", user_signal.get().first_name, user_signal.get().last_name)}
    </div>
}
```

### Batch Updates
```rust
// ✅ Good: Batch related updates
set_form.update(|form| {
    form.name = "New Name".to_string();
    form.email = "new@email.com".to_string();
});

// ❌ Bad: Multiple separate updates
set_name.set("New Name".to_string());
set_email.set("new@email.com".to_string());
```

## Debugging Signals

### Logging Signal Changes
```rust
create_effect(cx, move |_| {
    let value = signal.get();
    log::debug!("Signal changed: {:?}", value);
});
```

### Signal Inspector (Development)
```rust
#[cfg(debug_assertions)]
create_effect(cx, move |_| {
    let value = signal.get();
    leptos::console_log(&format!("Signal: {:?}", value));
});
```

## Signal Types Comparison

| Type | Read | Write | Use Case |
|------|------|-------|----------|
| `ReadSignal<T>` | ✅ | ❌ | Derived/computed values |
| `WriteSignal<T>` | ❌ | ✅ | Input controls |
| `RwSignal<T>` | ✅ | ✅ | General state |
| `Memo<T>` | ✅ | ❌ | Expensive computations |
| `Resource<T>` | ✅ | ❌ | Async data |

## Common Signal Mistakes

### 1. Reading in Render
```rust
// ❌ Bad: Reading multiple times
view! { cx,
    <div class={if signal.get() > 5 { "large" } else { "small" }}>
        {signal.get()}
    </div>
}

// ✅ Good: Read once
let value = signal.get();
view! { cx,
    <div class={if value > 5 { "large" } else { "small" }}>
        {value}
    </div>
}
```

### 2. Unnecessary Effects
```rust
// ❌ Bad: Effect just to update another signal
create_effect(cx, move |_| {
    let value = signal_a.get();
    signal_b.set(value * 2);
});

// ✅ Good: Use memo
let doubled = create_memo(cx, move |_| signal_a.get() * 2);
```

### 3. Missing Dependencies
```rust
// ❌ Bad: Missing dependency
let combined = create_memo(cx, move |_| {
    let a = signal_a.get();
    // Forgot signal_b!
    a * 2
});

// ✅ Good: Include all dependencies
let combined = create_memo(cx, move |_| {
    let a = signal_a.get();
    let b = signal_b.get();
    a * b
});
```

## Migration from Other Frameworks

### From React useState
```rust
// React
const [count, setCount] = useState(0);

// Leptos
let (count, set_count) = create_signal(cx, 0);
```

### From Vue ref
```rust
// Vue
const count = ref(0);

// Leptos
let count = create_rw_signal(cx, 0);
```

### From Svelte store
```rust
// Svelte
const store = writable(initial);

// Leptos
let store = create_rw_signal(cx, initial);
```

## Quick Reference

### Creating Signals
- `create_signal(cx, initial)` → `(ReadSignal<T>, WriteSignal<T>)`
- `create_rw_signal(cx, initial)` → `RwSignal<T>`
- `create_memo(cx, computation)` → `Memo<T>`
- `create_resource(cx, source, fetcher)` → `Resource<T>`

### Reading Signals
- `signal.get()` → Current value
- `signal.with(|value| ...)` → Access without cloning
- `signal()` → In view context (auto `.get()`)

### Writing Signals
- `signal.set(new_value)` → Replace value
- `signal.update(|current| ...)` → Modify current value

### Reactive Primitives
- `create_effect(cx, callback)` → Side effects
- `create_memo(cx, computation)` → Derived values
- `on_cleanup(cx, cleanup_fn)` → Cleanup effects