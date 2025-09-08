# Performance Tips Cheatsheet

## Reactive System Optimization

### Use Memos for Expensive Computations
```rust
// ✅ Good: Memoized expensive calculation
let expensive_result = create_memo(cx, move |_| {
    data.get().iter()
        .filter(|item| item.is_valid())
        .map(|item| item.compute_expensive_value())
        .sum::<f64>()
});

// ❌ Bad: Recalculated on every render
let expensive_result = move || {
    data.get().iter()
        .filter(|item| item.is_valid())
        .map(|item| item.compute_expensive_value())
        .sum::<f64>()
};
```

### Batch State Updates
```rust
// ✅ Good: Batch multiple updates
let update_multiple = move || {
    set_count.update(|c| *c += 1);
    set_total.update(|t| *t += amount);
    set_loading.set(false);
};

// ❌ Bad: Individual signal updates
let increment = move |_| set_count.update(|c| *c += 1);
let add_amount = move |_| set_total.update(|t| *t += amount);
let stop_loading = move |_| set_loading.set(false);
```

### Use Selectors for Derived State
```rust
// ✅ Good: Selector for specific derived state
let is_complete = create_selector(cx, move || {
    let todos = todos.get();
    todos.iter().all(|todo| todo.completed)
});

// ❌ Bad: Recalculating in multiple places
let is_complete = move || todos.get().iter().all(|todo| todo.completed);
```

## Component Optimization

### Avoid Unnecessary Re-renders
```rust
// ✅ Good: Extract static content
#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <header>
            <h1>"My App"</h1>
            <nav>
                <a href="/">"Home"</a>
                <a href="/about">"About"</a>
            </nav>
        </header>
    }
}

// ❌ Bad: Dynamic content for static elements
#[component]
pub fn Header(cx: Scope, title: String) -> impl IntoView {
    view! { cx,
        <header>
            <h1>{title}</h1> // Unnecessary reactivity
            <nav>
                <a href="/">"Home"</a>
                <a href="/about">"About"</a>
            </nav>
        </header>
    }
}
```

### Use Children Props Efficiently
```rust
// ✅ Good: Accept children as parameter
#[component]
pub fn Card<T>(cx: Scope, children: T) -> impl IntoView
where
    T: IntoView + 'static,
{
    view! { cx,
        <div class="card">
            {children}
        </div>
    }
}

// ❌ Bad: Clone children unnecessarily
#[component]
pub fn Card(cx: Scope, children: Children) -> impl IntoView {
    let children = children.clone(); // Unnecessary clone
    view! { cx,
        <div class="card">
            {children(cx)}
        </div>
    }
}
```

## List Rendering Optimization

### Use Keys for List Items
```rust
// ✅ Good: Proper keys for list items
view! { cx,
    <For
        each=items
        key=|item| item.id
        children=move |cx, item| view! { cx,
            <div key=item.id>
                {item.name}
            </div>
        }
    />
}

// ❌ Bad: No keys or wrong keys
view! { cx,
    <For
        each=items
        key=|_| 0 // Wrong key
        children=move |cx, item| view! { cx,
            <div>
                {item.name}
            </div>
        }
    />
}
```

### Virtual Scrolling for Large Lists
```rust
#[component]
pub fn VirtualList(cx: Scope, items: ReadSignal<Vec<Item>>) -> impl IntoView {
    let scroll_top = create_signal(cx, 0.0);
    let item_height = 50.0;
    let container_height = 400.0;

    let visible_range = create_memo(cx, move |_| {
        let start = (scroll_top.get() / item_height) as usize;
        let visible_count = (container_height / item_height) as usize;
        let end = (start + visible_count + 1).min(items.get().len());
        (start, end)
    });

    let visible_items = create_memo(cx, move |_| {
        let (start, end) = visible_range.get();
        items.get()[start..end].to_vec()
    });

    view! { cx,
        <div
            style=format!("height: {}px; overflow: auto", container_height)
            on:scroll=move |ev| {
                let target = event_target(&ev);
                scroll_top.set(target.scroll_top());
            }
        >
            <div style=format!("height: {}px", items.get().len() as f64 * item_height)>
                <div style=format!("transform: translateY({}px)", visible_range.get().0 as f64 * item_height)>
                    {move || visible_items.get().iter().map(|item| {
                        view! { cx, <div style=format!("height: {}px", item_height)>{&item.name}</div> }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
```

## Signal Optimization

### Use Read/Write Signals Appropriately
```rust
// ✅ Good: Separate read/write signals
let (count, set_count) = create_signal(cx, 0);
let double_count = create_memo(cx, move |_| count.get() * 2);

// ✅ Good: Read-only signal for derived state
let is_even = create_memo(cx, move |_| count.get() % 2 == 0);

// ❌ Bad: Unnecessary writes
let (count, set_count) = create_signal(cx, 0);
let display_count = create_memo(cx, move |_| {
    set_count.update(|c| *c); // Unnecessary write
    count.get().to_string()
});
```

### Avoid Signal Dependencies in Loops
```rust
// ✅ Good: Pre-calculate outside loop
let items = create_memo(cx, move |_| {
    let filter = filter.get();
    data.get().iter()
        .filter(|item| item.matches(&filter))
        .cloned()
        .collect::<Vec<_>>()
});

view! { cx,
    <For
        each=items
        key=|item| item.id
        children=move |cx, item| view! { cx,
            <div>{item.name}</div>
        }
    />
}

// ❌ Bad: Signal access inside loop
view! { cx,
    <For
        each=data
        key=|item| item.id
        children=move |cx, item| view! { cx,
            <Show
                when=move || item.matches(&filter.get()) // Signal access in loop
                fallback=|| ()
            >
                <div>{item.name}</div>
            </Show>
        }
    />
}
```

## Effect Optimization

### Cleanup Effects Properly
```rust
// ✅ Good: Proper cleanup
create_effect(cx, move |_| {
    let interval = set_interval(move || {
        // Do something
    }, 1000);

    on_cleanup(cx, move || {
        clear_interval(interval);
    });
});

// ❌ Bad: No cleanup
create_effect(cx, move |_| {
    let interval = set_interval(move || {
        // Do something
    }, 1000);
    // Memory leak!
});
```

### Use Effects Sparingly
```rust
// ✅ Good: Effect for side effects only
create_effect(cx, move |_| {
    let data = api_data.get();
    if !data.is_empty() {
        log::info!("Data loaded: {}", data.len());
    }
});

// ❌ Bad: Effect for derived state
create_effect(cx, move |_| {
    let result = expensive_computation(data.get());
    set_result.set(result); // Should use memo instead
});
```

## Resource Optimization

### Resource Caching
```rust
// ✅ Good: Cached resource
let user_resource = create_resource(cx,
    move || user_id.get(),
    move |id| async move {
        if let Some(cached) = cache.get(&id) {
            return cached;
        }
        let user = fetch_user(id).await?;
        cache.insert(id, user.clone());
        Ok(user)
    }
);

// ❌ Bad: No caching
let user_resource = create_resource(cx,
    move || user_id.get(),
    move |id| fetch_user(id) // Always fetches
);
```

### Resource Dependencies
```rust
// ✅ Good: Proper resource dependencies
let user_resource = create_resource(cx, move || user_id.get(), fetch_user);
let posts_resource = create_resource(cx,
    move || user_resource.read(cx).as_ref().map(|u| u.id),
    move |user_id| async move {
        match user_id {
            Some(id) => fetch_posts(id).await,
            None => Ok(vec![]),
        }
    }
);

// ❌ Bad: Unnecessary resource recreation
let user_resource = create_resource(cx, move || user_id.get(), fetch_user);
let posts_resource = create_resource(cx, move || user_id.get(), fetch_posts); // Wrong dependency
```

## Styling Performance

### Use CSS Classes Over Inline Styles
```rust
// ✅ Good: CSS classes
view! { cx,
    <div class="card shadow rounded">
        "Content"
    </div>
}

// ❌ Bad: Inline styles for static properties
view! { cx,
    <div style="box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1); border-radius: 0.5rem;">
        "Content"
    </div>
}
```

### Minimize Style Recalculations
```rust
// ✅ Good: Memoized styles
let styles = create_memo(cx, move |_| {
    format!("color: {}; font-size: {}px;", color.get(), size.get())
});

// ❌ Bad: Style concatenation on every render
view! { cx,
    <div style=format!("color: {}; font-size: {}px;", color.get(), size.get())>
        "Content"
    </div>
}
```

## Bundle Optimization

### Code Splitting
```rust
// ✅ Good: Lazy load components
let LazyComponent = lazy!(cx, || async move {
    load_component().await
});

view! { cx,
    <Suspense fallback=|| view! { cx, <div>"Loading..."</div> }>
        <LazyComponent />
    </Suspense>
}
```

### Tree Shaking
```rust
// ✅ Good: Import only what you need
use leptos::prelude::{create_signal, view};

// ❌ Bad: Import everything
use leptos::*;
```

## Memory Management

### Avoid Memory Leaks
```rust
// ✅ Good: Proper cleanup
#[component]
pub fn Timer(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    create_effect(cx, move |_| {
        let interval = set_interval(move || {
            set_count.update(|c| *c += 1);
        }, 1000);

        on_cleanup(cx, move || {
            clear_interval(interval);
        });
    });

    on_cleanup(cx, move || {
        // Additional cleanup
    });
}

// ❌ Bad: No cleanup
#[component]
pub fn Timer(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    create_effect(cx, move |_| {
        set_interval(move || {
            set_count.update(|c| *c += 1);
        }, 1000);
        // Memory leak!
    });
}
```

### Component Disposal
```rust
// ✅ Good: Manual disposal when needed
let component_ref = create_node_ref(cx);

create_effect(cx, move |_| {
    if should_dispose.get() {
        if let Some(element) = component_ref.get() {
            // Clean up
        }
    }
});
```

## Network Optimization

### Debounce API Calls
```rust
// ✅ Good: Debounced search
let search_query = create_signal(cx, String::new());
let debounced_query = create_memo(cx, move |_| {
    let query = search_query.get();
    if query.len() > 2 {
        Some(query)
    } else {
        None
    }
});

let search_resource = create_resource(cx, debounced_query, search_api);

// ❌ Bad: Search on every keystroke
let search_resource = create_resource(cx, move || search_query.get(), search_api);
```

### Request Deduplication
```rust
// ✅ Good: Request deduplication
let user_resource = create_resource(cx, move || user_id.get(), move |id| async move {
    static CACHE: once_cell::sync::Lazy<Mutex<HashMap<String, User>>> = 
        once_cell::sync::Lazy::new(|| Mutex::new(HashMap::new()));
    
    let mut cache = CACHE.lock().unwrap();
    if let Some(user) = cache.get(&id) {
        return Ok(user.clone());
    }
    
    let user = fetch_user(&id).await?;
    cache.insert(id, user.clone());
    Ok(user)
});
```

## Profiling and Debugging

### Performance Monitoring
```rust
// ✅ Good: Performance monitoring
create_effect(cx, move |_| {
    let start = instant::Instant::now();
    // Expensive operation
    let result = expensive_computation(data.get());
    let duration = start.elapsed();
    
    if duration > Duration::from_millis(100) {
        log::warn!("Slow computation: {:?}", duration);
    }
    
    set_result.set(result);
});
```

### Development Performance Tips
```rust
// Use in development only
#[cfg(debug_assertions)]
create_effect(cx, move |_| {
    log::debug!("Component re-rendered");
    log::debug!("Data: {:?}", data.get());
});
```

## Best Practices Summary

### ✅ Performance Do's
- Use memos for expensive computations
- Batch state updates
- Use selectors for derived state
- Implement virtual scrolling for large lists
- Cache resources and API responses
- Use CSS classes over inline styles
- Clean up effects and intervals
- Debounce user input
- Code splitting for large applications
- Profile and monitor performance

### ❌ Performance Don'ts
- Don't access signals in loops
- Don't create effects for derived state
- Don't forget cleanup functions
- Don't make unnecessary re-renders
- Don't use inline styles for static properties
- Don't create resources without proper dependencies
- Don't forget to dispose components when needed
- Don't make API calls on every keystroke
- Don't bundle everything into one chunk
- Don't ignore performance in development

### Key Metrics to Monitor
- Initial bundle size
- Time to interactive
- Memory usage
- Network requests
- Frame rate
- Component render count
- Effect execution time

### Tools for Performance Analysis
- Browser DevTools Performance tab
- React DevTools Profiler (adapted for Leptos)
- `console.time()` and `console.timeEnd()`
- Custom performance monitoring
- Bundle analyzer tools
- Memory leak detection tools

### Progressive Enhancement
```rust
// Start with basic functionality
#[component]
pub fn DataTable(cx: Scope, data: ReadSignal<Vec<Item>>) -> impl IntoView {
    view! { cx,
        <table>
            <For
                each=data
                key=|item| item.id
                children=move |cx, item| view! { cx,
                    <tr>
                        <td>{item.name}</td>
                        <td>{item.value}</td>
                    </tr>
                }
            />
        </table>
    }
}

// Add performance optimizations incrementally
// 1. Virtual scrolling for large datasets
// 2. Memoized computations
// 3. Debounced search
// 4. Caching layers
// 5. Code splitting
```

## Quick Reference

### Performance Checklist
- [ ] Are you using memos for expensive computations?
- [ ] Are state updates batched?
- [ ] Are you using keys in lists?
- [ ] Are effects properly cleaned up?
- [ ] Are resources cached?
- [ ] Are API calls debounced?
- [ ] Are large lists virtualized?
- [ ] Are styles using CSS classes?
- [ ] Is code split appropriately?
- [ ] Are memory leaks prevented?

### Common Performance Patterns
```rust
// Expensive computation
let result = create_memo(cx, move |_| expensive_fn(data.get()));

// State batching
let update = move || {
    set_a.set(new_a);
    set_b.set(new_b);
    set_c.set(new_c);
};

// Resource caching
let cached_resource = create_resource(cx, key, move |k| async {
    if let Some(cached) = cache.get(&k) {
        return cached;
    }
    let result = fetch(k).await?;
    cache.insert(k, result.clone());
    result
});

// Virtual scrolling
let visible_items = create_memo(cx, move |_| {
    let start = (scroll_top.get() / item_height) as usize;
    let end = (start + visible_count).min(items.get().len());
    items.get()[start..end].to_vec()
});
```

### Performance Monitoring
```rust
// Component render tracking
#[cfg(debug_assertions)]
static RENDER_COUNT: AtomicUsize = AtomicUsize::new(0);

#[component]
pub fn TrackedComponent(cx: Scope) -> impl IntoView {
    let count = RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
    
    #[cfg(debug_assertions)]
    create_effect(cx, move |_| {
        log::debug!("Component rendered {} times", count);
    });
    
    // Component implementation
}
```

### Bundle Size Optimization
```rust
// Feature flags for conditional compilation
#[cfg(feature = "heavy_feature")]
pub fn heavy_function() {
    // Large dependency only included when needed
}

// Dynamic imports
let HeavyComponent = lazy!(cx, || async {
    // Load heavy component on demand
    load_heavy_component().await
});
```

Remember: Performance optimization is iterative. Start with the basics, measure, identify bottlenecks, and optimize incrementally. Always measure the impact of your optimizations to ensure they're effective.