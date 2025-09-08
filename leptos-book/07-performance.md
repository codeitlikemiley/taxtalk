# 07: Performance

## Overview

Building performant Leptos applications requires understanding both the framework's reactive system and general web performance best practices. This chapter covers optimization techniques, performance monitoring, and strategies for maintaining fast, responsive applications. We'll explore reactive optimizations, rendering strategies, memory management, and performance measurement tools.

## Reactive System Optimization

### Memoization

Memoization is crucial for expensive computations that depend on reactive values:

```rust
use leptos::*;

// Basic memoization
#[component]
pub fn ExpensiveCalculation() -> impl IntoView {
    let (input, set_input) = create_signal(1);
    
    // Memoize expensive computation
    let result = create_memo(move |_| {
        // Simulate expensive calculation
        let value = input.get();
        (0..1000).map(|i| (i * value) % 100).sum::<i32>()
    });
    
    view! {
        <div>
            <input
                type="number"
                prop:value=input
                on:input=move |ev| set_input.set(event_target_value(&ev).parse().unwrap_or(1))
            />
            <p>"Result: " {result}</p>
        </div>
    }
}
```

### Selective Re-rendering

Use signals to control when components re-render:

```rust
use leptos::*;

// Selective re-rendering with derived signals
#[component]
pub fn SelectiveUpdate() -> impl IntoView {
    let (data, set_data) = create_signal(LargeData {
        items: (0..1000).collect(),
        metadata: "Initial".to_string(),
        timestamp: chrono::Utc::now(),
    });
    
    // Only re-compute when items change
    let item_count = create_memo(move |_| data.get().items.len());
    
    // Only re-compute when metadata changes
    let display_metadata = create_memo(move |_| data.get().metadata);
    
    view! {
        <div>
            <button on:click=move |_| {
                set_data.update(|d| {
                    d.items.push(d.items.len());
                });
            }>
                "Add Item (affects count)"
            </button>
            
            <button on:click=move |_| {
                set_data.update(|d| {
                    d.metadata = format!("Updated at {}", chrono::Utc::now());
                });
            }>
                "Update Metadata (affects display)"
            </button>
            
            <p>"Item count: " {item_count} " (re-renders when items change)"</p>
            <p>"Metadata: " {display_metadata} " (re-renders when metadata changes)"</p>
        </div>
    }
}

#[derive(Clone, Debug)]
pub struct LargeData {
    pub items: Vec<usize>,
    pub metadata: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
```

### Batching Updates

Batch multiple state updates to reduce re-renders:

```rust
use leptos::*;

// Batching updates
#[component]
pub fn BatchUpdates() -> impl IntoView {
    let (count1, set_count1) = create_signal(0);
    let (count2, set_count2) = create_signal(0);
    let (count3, set_count3) = create_signal(0);
    
    let total = create_memo(move |_| count1.get() + count2.get() + count3.get());
    
    let increment_all = move |_| {
        // Batch updates - only one re-render
        batch(move || {
            set_count1.update(|c| *c + 1);
            set_count2.update(|c| *c + 1);
            set_count3.update(|c| *c + 1);
        });
    };
    
    view! {
        <div>
            <button on:click=increment_all>"Increment All"</button>
            <p>"Count 1: " {count1}</p>
            <p>"Count 2: " {count2}</p>
            <p>"Count 3: " {count3}</p>
            <p>"Total: " {total}</p>
        </div>
    }
}
```

## Component Optimization

### Component Splitting

Split large components into smaller, focused components:

```rust
use leptos::*;

// Before: Large monolithic component
#[component]
pub fn LargeComponent() -> impl IntoView {
    let (user, set_user) = create_signal(User::default());
    let (posts, set_posts) = create_signal(vec![]);
    let (comments, set_comments) = create_signal(vec![]);
    
    view! {
        <div class="large-component">
            <UserProfile user=user set_user=set_user />
            <PostList posts=posts set_posts=set_posts />
            <CommentSection comments=comments set_comments=set_comments />
        </div>
    }
}

// After: Split into focused components
#[component]
pub fn UserProfile(user: Signal<User>, set_user: WriteSignal<User>) -> impl IntoView {
    view! {
        <div class="user-profile">
            <h2>{move || user.get().name}</h2>
            <p>{move || user.get().email}</p>
            <button on:click=move |_| {
                set_user.update(|u| u.name = "Updated Name".to_string());
            }>"Update Name"</button>
        </div>
    }
}

#[component]
pub fn PostList(posts: Signal<Vec<Post>>, set_posts: WriteSignal<Vec<Post>>) -> impl IntoView {
    view! {
        <div class="post-list">
            <h3>"Posts"</h3>
            {move || posts.get().into_iter().map(|post| view! {
                <PostItem post />
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
pub fn PostItem(post: Post) -> impl IntoView {
    view! {
        <div class="post-item">
            <h4>{post.title}</h4>
            <p>{post.content}</p>
        </div>
    }
}

#[component]
pub fn CommentSection(comments: Signal<Vec<Comment>>, set_comments: WriteSignal<Vec<Comment>>) -> impl IntoView {
    view! {
        <div class="comment-section">
            <h3>"Comments"</h3>
            {move || comments.get().into_iter().map(|comment| view! {
                <CommentItem comment />
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
pub fn CommentItem(comment: Comment) -> impl IntoView {
    view! {
        <div class="comment-item">
            <p>{comment.text}</p>
            <small>{comment.author}</small>
        </div>
    }
}

#[derive(Clone, Default, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(Clone, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
}

#[derive(Clone, Debug)]
pub struct Comment {
    pub id: u32,
    pub text: String,
    pub author: String,
}
```

### Lazy Loading Components

Load components only when needed:

```rust
use leptos::*;

// Lazy loading with Suspense
#[component]
pub fn LazyLoadedContent() -> impl IntoView {
    let (show_content, set_show_content) = create_signal(false);
    
    view! {
        <div>
            <button on:click=move |_| set_show_content.set(true)>
                "Load Heavy Content"
            </button>
            
            {move || show_content.get().then(|| view! {
                <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                    <HeavyComponent />
                </Suspense>
            })}
        </div>
    }
}

#[component]
pub fn HeavyComponent() -> impl IntoView {
    // Simulate heavy computation
    let data = create_resource(|| (), |_| async move {
        // Heavy async operation
        gloo::timers::future::TimeoutFuture::new(2_000).await;
        "Heavy data loaded!".to_string()
    });
    
    view! {
        <div class="heavy-content">
            {move || data.get().map(|data| view! {
                <p>{data}</p>
            }).unwrap_or_else(|| view! {
                <p>"Loading heavy content..."</p>
            })}
        </div>
    }
}
```

## Memory Management

### Resource Cleanup

Properly clean up resources to prevent memory leaks:

```rust
use leptos::*;
use std::rc::Rc;

// Resource cleanup with on_cleanup
#[component]
pub fn ResourceManager() -> impl IntoView {
    let (interval_id, set_interval_id) = create_signal(None);
    
    create_effect(move |_| {
        let id = set_interval_with_handle(
            move || {
                logging::log!("Interval tick");
            },
            std::time::Duration::from_secs(1)
        );
        
        set_interval_id.set(Some(id));
        
        // Cleanup when component unmounts
        on_cleanup(move || {
            if let Some(id) = interval_id.get() {
                id.clear();
            }
        });
    });
    
    view! {
        <div>
            <p>"Resource manager with cleanup"</p>
        </div>
    }
}
```

### Signal Memory Optimization

Be mindful of signal memory usage:

```rust
use leptos::*;

// Memory-efficient signal usage
#[component]
pub fn MemoryEfficient() -> impl IntoView {
    // Use RwSignal for complex state that needs both read and write
    let complex_state = create_rw_signal(ComplexState::default());
    
    // Use ReadSignal when you only need to read
    let derived_value = create_memo(move |_| {
        let state = complex_state.read();
        state.value * 2
    });
    
    view! {
        <div>
            <button on:click=move |_| {
                complex_state.update(|state| state.value += 1);
            }>"Increment"</button>
            <p>"Value: " {move || complex_state.read().value}</p>
            <p>"Derived: " {derived_value}</p>
        </div>
    }
}

#[derive(Clone, Default)]
pub struct ComplexState {
    pub value: i32,
    pub metadata: std::collections::HashMap<String, String>,
}
```

## Rendering Optimization

### Virtual Scrolling

Implement virtual scrolling for large lists:

```rust
use leptos::*;

// Virtual scrolling implementation
#[component]
pub fn VirtualList() -> impl IntoView {
    let items = create_signal((0..10000).collect::<Vec<_>>());
    let (scroll_top, set_scroll_top) = create_signal(0);
    let (container_height, set_container_height) = create_signal(400);
    
    const ITEM_HEIGHT: f64 = 50.0;
    let visible_count = create_memo(move |_| {
        (container_height.get() as f64 / ITEM_HEIGHT).ceil() as usize + 2 // +2 for buffer
    });
    
    let start_index = create_memo(move |_| {
        (scroll_top.get() as f64 / ITEM_HEIGHT).floor() as usize
    });
    
    let end_index = create_memo(move |_| {
        (start_index.get() + visible_count.get()).min(items.get().len())
    });
    
    let visible_items = create_memo(move |_| {
        let start = start_index.get();
        let end = end_index.get();
        items.get()[start..end].to_vec()
    });
    
    let total_height = create_memo(move |_| {
        items.get().len() as f64 * ITEM_HEIGHT
    });
    
    let offset_y = create_memo(move |_| {
        start_index.get() as f64 * ITEM_HEIGHT
    });
    
    view! {
        <div 
            class="virtual-list-container"
            style:height=move || format!("{}px", container_height.get())
            on:scroll=move |ev| {
                let target = event_target::<web_sys::Element>(&ev);
                set_scroll_top.set(target.scroll_top());
            }
        >
            <div 
                class="virtual-list-content"
                style:height=move || format!("{}px", total_height.get())
            >
                <div 
                    class="virtual-list-items"
                    style:transform=move || format!("translateY({}px)", offset_y.get())
                >
                    {move || visible_items.get().into_iter().enumerate().map(|(i, item)| {
                        let global_index = start_index.get() + i;
                        view! {
                            <div 
                                class="virtual-list-item"
                                style:height=move || format!("{}px", ITEM_HEIGHT)
                            >
                                "Item " {global_index} ": " {item}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
```

### Debouncing and Throttling

Control the frequency of expensive operations:

```rust
use leptos::*;
use std::time::Duration;

// Debounced search
#[component]
pub fn DebouncedSearch() -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let (debounced_query, set_debounced_query) = create_signal(String::new());
    
    // Debounce the search query
    create_effect(move |_| {
        let current_query = query.get();
        set_timeout_with_handle(
            move || {
                set_debounced_query.set(current_query);
            },
            Duration::from_millis(300)
        );
    });
    
    // Simulate search results
    let search_results = create_memo(move |_| {
        let q = debounced_query.get();
        if q.is_empty() {
            vec![]
        } else {
            (0..10).map(|i| format!("Result {} for '{}'", i, q)).collect()
        }
    });
    
    view! {
        <div class="search-container">
            <input
                type="text"
                placeholder="Search..."
                prop:value=query
                on:input=move |ev| set_query.set(event_target_value(&ev))
            />
            <div class="search-results">
                {move || search_results.get().into_iter().map(|result| view! {
                    <div class="search-result">{result}</div>
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
```

## Performance Monitoring

### Performance Hooks

Create custom hooks for performance monitoring:

```rust
use leptos::*;
use std::time::Instant;

// Performance monitoring hook
pub fn use_performance_monitor(name: &'static str) -> (Signal<PerformanceMetrics>, impl Fn()) {
    let (metrics, set_metrics) = create_signal(PerformanceMetrics::default());
    let start_time = create_rw_signal(None);
    
    let start = move || {
        start_time.set(Some(Instant::now()));
    };
    
    let end = move || {
        if let Some(start) = start_time.get() {
            let duration = start.elapsed();
            set_metrics.update(|m| {
                m.render_count += 1;
                m.total_render_time += duration;
                m.average_render_time = m.total_render_time / m.render_count as u32;
                m.last_render_time = Some(duration);
            });
        }
    };
    
    // Track renders
    create_effect(move |_| {
        end();
        start();
    });
    
    (metrics.into(), start)
}

#[derive(Clone, Default, Debug)]
pub struct PerformanceMetrics {
    pub render_count: u32,
    pub total_render_time: std::time::Duration,
    pub average_render_time: std::time::Duration,
    pub last_render_time: Option<std::time::Duration>,
}

// Usage in component
#[component]
pub fn MonitoredComponent() -> impl IntoView {
    let (metrics, _) = use_performance_monitor("MonitoredComponent");
    
    view! {
        <div>
            <h3>"Performance Metrics"</h3>
            <p>"Render count: " {move || metrics.get().render_count}</p>
            <p>"Average render time: " {move || format!("{:?}", metrics.get().average_render_time)}</p>
            <p>"Last render time: " {move || metrics.get().last_render_time.map(|d| format!("{:?}", d)).unwrap_or_else(|| "N/A".to_string())}</p>
        </div>
    }
}
```

### Browser Performance API

Use browser performance APIs for detailed metrics:

```rust
use leptos::*;
use wasm_bindgen::prelude::*;

// Browser performance monitoring
#[component]
pub fn BrowserPerformanceMonitor() -> impl IntoView {
    let (performance_data, set_performance_data) = create_signal(BrowserPerformance::default());
    
    // Update performance data periodically
    create_effect(move |_| {
        set_timeout_with_handle(
            move || {
                if let Ok(perf) = get_browser_performance() {
                    set_performance_data.set(perf);
                }
            },
            std::time::Duration::from_secs(1)
        );
    });
    
    view! {
        <div class="performance-monitor">
            <h3>"Browser Performance"</h3>
            <p>"Memory Usage: " {move || format!("{:.2} MB", performance_data.get().memory_usage_mb)}</p>
            <p>"DOM Nodes: " {move || performance_data.get().dom_nodes}</p>
            <p>"Layout Time: " {move || format!("{:.2}ms", performance_data.get().layout_time_ms)}</p>
        </div>
    }
}

#[derive(Clone, Default, Debug)]
pub struct BrowserPerformance {
    pub memory_usage_mb: f64,
    pub dom_nodes: u32,
    pub layout_time_ms: f64,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance)]
    fn memory() -> JsValue;
    
    #[wasm_bindgen(js_namespace = document)]
    fn getElementsByTagName(tag: &str) -> JsValue;
}

fn get_browser_performance() -> Result<BrowserPerformance, JsValue> {
    let mut perf = BrowserPerformance::default();
    
    // Get memory usage
    if let Ok(memory) = memory() {
        if let Ok(memory_obj) = js_sys::Object::from(memory) {
            if let Ok(used_js_heap_size) = js_sys::Reflect::get(&memory_obj, &"usedJSHeapSize".into()) {
                if let Ok(heap_size) = used_js_heap_size.as_f64() {
                    perf.memory_usage_mb = heap_size / (1024.0 * 1024.0);
                }
            }
        }
    }
    
    // Count DOM nodes
    if let Ok(elements) = getElementsByTagName("*") {
        if let Ok(html_collection) = elements.dyn_into::<web_sys::HtmlCollection>() {
            perf.dom_nodes = html_collection.length();
        }
    }
    
    Ok(perf)
}
```

## Bundle Optimization

### Code Splitting

Split your application into smaller chunks:

```rust
use leptos::*;
use leptos_router::*;

// Code splitting with dynamic imports
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage />
                <Route path="/dashboard" view=DashboardPage />
                <Route path="/admin" view=AdminPage />
                <Route path="/reports" view=ReportsPage />
            </Routes>
        </Router>
    }
}

// Lazy-loaded components
#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <Suspense fallback=move || view! { <p>"Loading dashboard..."</p> }>
            <Dashboard />
        </Suspense>
    }
}

#[component]
pub fn AdminPage() -> impl IntoView {
    view! {
        <Suspense fallback=move || view! { <p>"Loading admin panel..."</p> }>
            <AdminPanel />
        </Suspense>
    }
}

#[component]
pub fn ReportsPage() -> impl IntoView {
    view! {
        <Suspense fallback=move || view! { <p>"Loading reports..."</p> }>
            <Reports />
        </Suspense>
    }
}

// These components would be in separate modules/files
#[component]
pub fn Dashboard() -> impl IntoView {
    view! { <div>"Dashboard Content"</div> }
}

#[component]
pub fn AdminPanel() -> impl IntoView {
    view! { <div>"Admin Panel Content"</div> }
}

#[component]
pub fn Reports() -> impl IntoView {
    view! { <div>"Reports Content"</div> }
}
```

## Server-Side Rendering Optimization

### Hydration Optimization

Optimize hydration for better performance:

```rust
use leptos::*;

// Optimize hydration with selective client-side rendering
#[component]
pub fn OptimizedApp() -> impl IntoView {
    view! {
        <div>
            // Server-rendered content
            <Header />
            
            // Client-side only content
            <ClientOnly fallback=move || view! { <div>"Loading..."</div> }>
                <InteractiveContent />
            </ClientOnly>
            
            // Server-rendered content
            <Footer />
        </div>
    }
}

#[component]
pub fn ClientOnly(children: Children, fallback: Children) -> impl IntoView {
    let (mounted, set_mounted) = create_signal(false);
    
    create_effect(move |_| {
        set_mounted.set(true);
    });
    
    move || if mounted.get() {
        children()
    } else {
        fallback()
    }
}

#[component]
pub fn InteractiveContent() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    
    view! {
        <div>
            <button on:click=move |_| set_count.update(|c| *c + 1)>
                "Count: " {count}
            </button>
        </div>
    }
}
```

## Performance Testing

### Automated Performance Tests

Create performance tests for your components:

```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use leptos::*;
    use std::time::Instant;
    
    #[test]
    fn test_component_render_performance() {
        let mut app = create_runtime();
        
        let start = Instant::now();
        
        // Create a component with many items
        let items = (0..1000).collect::<Vec<_>>();
        let (items_signal, _) = create_signal(items);
        
        let view = ListComponent(items_signal.into());
        
        let render_time = start.elapsed();
        
        println!("Render time for 1000 items: {:?}", render_time);
        
        // Assert performance requirements
        assert!(render_time < std::time::Duration::from_millis(100), 
                "Render should complete in under 100ms");
        
        app.dispose();
    }
    
    #[test]
    fn test_memo_performance() {
        let mut app = create_runtime();
        
        let (input, set_input) = create_signal(1);
        
        let expensive_computation = create_memo(move |_| {
            let start = Instant::now();
            let result = (0..100000).map(|i| (i * input.get()) % 100).sum::<i32>();
            let computation_time = start.elapsed();
            println!("Computation time: {:?}", computation_time);
            result
        });
        
        // First computation
        let _ = expensive_computation.get();
        let first_time = Instant::now();
        
        // Second computation with same input (should be memoized)
        let _ = expensive_computation.get();
        let second_time = first_time.elapsed();
        
        // Memoized computation should be very fast
        assert!(second_time < std::time::Duration::from_micros(100), 
                "Memoized computation should be very fast");
        
        app.dispose();
    }
}

#[component]
pub fn ListComponent(items: Signal<Vec<i32>>) -> impl IntoView {
    view! {
        <ul>
            {move || items.get().into_iter().map(|item| view! {
                <li>{item}</li>
            }).collect::<Vec<_>>()}
        </ul>
    }
}
```

## Performance Best Practices

### General Guidelines

1. **Use memoization** for expensive computations
2. **Batch state updates** to reduce re-renders
3. **Split large components** into smaller focused components
4. **Implement lazy loading** for heavy components
5. **Clean up resources** properly to prevent memory leaks
6. **Use virtual scrolling** for large lists
7. **Debounce user input** for search and form fields
8. **Monitor performance** with custom hooks and browser APIs
9. **Implement code splitting** to reduce bundle size
10. **Optimize hydration** for SSR applications

### Common Performance Pitfalls

```rust
use leptos::*;

// ❌ Bad: Creating signals in render function
#[component]
pub fn BadComponent() -> impl IntoView {
    view! {
        <div>
            {move || {
                let (count, set_count) = create_signal(0); // Created on every render!
                view! {
                    <button on:click=move |_| set_count.update(|c| *c + 1)>
                        {count}
                    </button>
                }
            }}
        </div>
    }
}

// ✅ Good: Create signals once at component level
#[component]
pub fn GoodComponent() -> impl IntoView {
    let (count, set_count) = create_signal(0); // Created once
    
    view! {
        <div>
            <button on:click=move |_| set_count.update(|c| *c + 1)>
                {count}
            </button>
        </div>
    }
}
```

### Performance Checklist

- [ ] Use `create_memo` for expensive computations
- [ ] Batch related state updates with `batch()`
- [ ] Split large components into smaller ones
- [ ] Use `Suspense` for lazy loading
- [ ] Implement proper cleanup with `on_cleanup`
- [ ] Use virtual scrolling for large lists
- [ ] Debounce/throttle user input
- [ ] Monitor performance with custom hooks
- [ ] Implement code splitting
- [ ] Optimize bundle size
- [ ] Use performance budgets in CI/CD

---

**Next:** [08: SSR & Hydration](08-ssr-hydration.md) - Learn about server-side rendering and hydration strategies.