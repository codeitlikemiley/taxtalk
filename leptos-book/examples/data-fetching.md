# Data Fetching Example

## Overview

This example demonstrates comprehensive data fetching patterns in Leptos, including async operations, loading states, error handling, caching, and real-time updates using Server-Sent Events (SSE).

## Complete Code

```rust
use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub author_id: u32,
    pub created_at: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum FetchState<T> {
    Idle,
    Loading,
    Success(T),
    Error(String),
}

impl<T> Default for FetchState<T> {
    fn default() -> Self {
        Self::Idle
    }
}

#[component]
fn UserProfile(cx: Scope, user_id: u32) -> impl IntoView {
    // State for user data
    let (user_state, set_user_state) = create_signal(cx, FetchState::<User>::Idle);
    let (posts_state, set_posts_state) = create_signal(cx, FetchState::<Vec<Post>>::Idle);

    // Fetch user data on mount
    create_effect(cx, move |_| {
        set_user_state.set(FetchState::Loading);

        // Simulate API call
        set_timeout(move || {
            // Mock user data
            let user = User {
                id: user_id,
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
                avatar_url: Some("https://via.placeholder.com/150".to_string()),
            };
            set_user_state.set(FetchState::Success(user));
        }, 1000);
    });

    // Fetch user's posts when user data is loaded
    create_effect(cx, move |_| {
        if let FetchState::Success(user) = user_state.get() {
            set_posts_state.set(FetchState::Loading);

            set_timeout(move || {
                // Mock posts data
                let posts = vec![
                    Post {
                        id: 1,
                        title: "My First Post".to_string(),
                        content: "This is my first blog post!".to_string(),
                        author_id: user.id,
                        created_at: "2024-01-15T10:00:00Z".to_string(),
                        tags: vec!["introduction".to_string(), "blog".to_string()],
                    },
                    Post {
                        id: 2,
                        title: "Learning Leptos".to_string(),
                        content: "Leptos is a great web framework!".to_string(),
                        author_id: user.id,
                        created_at: "2024-01-20T14:30:00Z".to_string(),
                        tags: vec!["leptos".to_string(), "rust".to_string()],
                    },
                ];
                set_posts_state.set(FetchState::Success(posts));
            }, 800);
        }
    });

    view! { cx,
        div(class="user-profile") {
            // User info section
            (match user_state.get() {
                FetchState::Idle => view! { cx,
                    div(class="loading") { "Initializing..." }
                }.into_view(cx),
                FetchState::Loading => view! { cx,
                    div(class="loading") {
                        div(class="spinner") </div>
                        "Loading user..."
                    }
                }.into_view(cx),
                FetchState::Success(user) => view! { cx,
                    div(class="user-info") {
                        (if let Some(avatar_url) = &user.avatar_url {
                            view! { cx,
                                img(src=avatar_url, alt="User avatar", class="avatar")
                            }.into_view(cx)
                        } else {
                            view! { cx,
                                div(class="avatar-placeholder") {
                                    (user.name.chars().next().unwrap_or('?').to_uppercase())
                                }
                            }.into_view(cx)
                        })

                        div(class="user-details") {
                            h2 { (user.name) }
                            p { (user.email) }
                        }
                    }
                }.into_view(cx),
                FetchState::Error(error) => view! { cx,
                    div(class="error") {
                        "Error loading user: " (error)
                        button(on:click=move |_| set_user_state.set(FetchState::Loading)) {
                            "Retry"
                        }
                    }
                }.into_view(cx),
            })

            // Posts section
            (match posts_state.get() {
                FetchState::Idle => view! { cx, }.into_view(cx),
                FetchState::Loading => view! { cx,
                    div(class="loading") { "Loading posts..." }
                }.into_view(cx),
                FetchState::Success(posts) => view! { cx,
                    div(class="posts-section") {
                        h3 { "Recent Posts" }
                        ul(class="posts-list") {
                            (posts.into_iter().map(|post| {
                                view! { cx,
                                    li(class="post-item") {
                                        h4 { (post.title) }
                                        p { (post.content) }
                                        div(class="post-meta") {
                                            "Posted on " (post.created_at.split('T').next().unwrap_or("Unknown"))
                                            " • Tags: " (post.tags.join(", "))
                                        }
                                    }
                                }
                            }).collect::<Vec<_>>())
                        }
                    }
                }.into_view(cx),
                FetchState::Error(error) => view! { cx,
                    div(class="error") { "Error loading posts: " (error) }
                }.into_view(cx),
            })
        }
    }
}

#[component]
fn DataFetchingWithCache(cx: Scope) -> impl IntoView {
    // Simple in-memory cache
    let cache: StoredValue<HashMap<String, (FetchState<Vec<User>>, u64)>> =
        store_value(cx, HashMap::new());

    let (users_state, set_users_state) = create_signal(cx, FetchState::<Vec<User>>::Idle);
    let (cache_hit, set_cache_hit) = create_signal(cx, false);

    let fetch_users = move || {
        let cache_key = "users".to_string();
        let now = js_sys::Date::now() as u64;
        let cache_duration = 30000; // 30 seconds

        // Check cache first
        let cached_data = cache.with_value(|cache| {
            cache.get(&cache_key).cloned()
        });

        if let Some((cached_state, timestamp)) = cached_data {
            if now - timestamp < cache_duration {
                set_users_state.set(cached_state);
                set_cache_hit.set(true);
                return;
            }
        }

        set_cache_hit.set(false);
        set_users_state.set(FetchState::Loading);

        // Simulate API call
        set_timeout(move || {
            let users = vec![
                User {
                    id: 1,
                    name: "Alice Johnson".to_string(),
                    email: "alice@example.com".to_string(),
                    avatar_url: Some("https://via.placeholder.com/100".to_string()),
                },
                User {
                    id: 2,
                    name: "Bob Smith".to_string(),
                    email: "bob@example.com".to_string(),
                    avatar_url: None,
                },
                User {
                    id: 3,
                    name: "Carol Williams".to_string(),
                    email: "carol@example.com".to_string(),
                    avatar_url: Some("https://via.placeholder.com/100".to_string()),
                },
            ];

            let success_state = FetchState::Success(users.clone());

            // Update cache
            cache.update_value(|cache| {
                cache.insert(cache_key, (success_state.clone(), now));
            });

            set_users_state.set(success_state);
        }, 1500);
    };

    // Fetch on mount
    create_effect(cx, move |_| {
        fetch_users();
    });

    view! { cx,
        div(class="data-cache-demo") {
            h3 { "Cached Data Fetching" }

            div(class="cache-status") {
                (if cache_hit.get() {
                    view! { cx, span(class="cache-hit") { "⚡ Cache Hit!" } }
                } else {
                    view! { cx, span(class="cache-miss") { "🌐 Network Request" } }
                })
            }

            button(on:click=move |_| fetch_users()) { "Refresh Data" }

            (match users_state.get() {
                FetchState::Idle => view! { cx, }.into_view(cx),
                FetchState::Loading => view! { cx,
                    div(class="loading") { "Loading users..." }
                }.into_view(cx),
                FetchState::Success(users) => view! { cx,
                    ul(class="users-list") {
                        (users.into_iter().map(|user| {
                            view! { cx,
                                li(class="user-item") {
                                    (if let Some(avatar_url) = &user.avatar_url {
                                        view! { cx, img(src=avatar_url, alt="Avatar", class="small-avatar") }
                                    } else {
                                        view! { cx, }
                                    })
                                    span { (user.name) " - " (user.email) }
                                }
                            }
                        }).collect::<Vec<_>>())
                    }
                }.into_view(cx),
                FetchState::Error(error) => view! { cx,
                    div(class="error") { "Error: " (error) }
                }.into_view(cx),
            })
        }
    }
}

#[component]
fn RealTimeUpdates(cx: Scope) -> impl IntoView {
    let (messages, set_messages) = create_signal(cx, Vec::<String>::new());
    let (connection_status, set_connection_status) = create_signal(cx, "Connecting...".to_string());

    // Simulate SSE connection
    create_effect(cx, move |_| {
        set_connection_status.set("Connected".to_string());

        // Simulate receiving messages
        let mut count = 0;
        let interval = set_interval(move || {
            count += 1;
            let message = format!("Message {} at {}", count, js_sys::Date::now());
            set_messages.update(|messages| messages.push(message));

            if count >= 10 {
                set_connection_status.set("Connection closed".to_string());
                // In real app, clear interval
            }
        }, 2000);

        // Cleanup
        on_cleanup(cx, move || {
            // Clear interval in real implementation
        });
    });

    view! { cx,
        div(class="real-time-demo") {
            h3 { "Real-Time Updates (SSE Simulation)" }

            div(class="connection-status") {
                "Status: " (connection_status.get())
            }

            div(class="messages-container") {
                ul(class="messages-list") {
                    (messages.get().into_iter().enumerate().map(|(i, message)| {
                        view! { cx,
                            li(class="message-item", key=i) {
                                span(class="message-time") {
                                    (js_sys::Date::now().to_string())
                                }
                                " " (message)
                            }
                        }
                    }).collect::<Vec<_>>())
                }
            }

            (if messages.get().is_empty() {
                view! { cx,
                    div(class="no-messages") { "Waiting for messages..." }
                }.into_view(cx)
            } else {
                view! { cx, }.into_view(cx)
            })
        }
    }
}

#[component]
fn DataFetchingWithRetry(cx: Scope) -> impl IntoView {
    let (data_state, set_data_state) = create_signal(cx, FetchState::<String>::Idle);
    let (retry_count, set_retry_count) = create_signal(cx, 0);
    let max_retries = 3;

    let fetch_with_retry = move || {
        set_data_state.set(FetchState::Loading);
        set_retry_count.set(0);

        let attempt_fetch = move |attempt: u32| {
            set_timeout(move || {
                // Simulate random failure (70% success rate)
                let success = js_sys::Math::random() > 0.3;

                if success || attempt >= max_retries {
                    if success {
                        set_data_state.set(FetchState::Success("Data loaded successfully!".to_string()));
                    } else {
                        set_data_state.set(FetchState::Error("Failed to load data after all retries".to_string()));
                    }
                } else {
                    set_retry_count.set(attempt);
                    attempt_fetch(attempt + 1);
                }
            }, 1000);
        };

        attempt_fetch(1);
    };

    // Auto-fetch on mount
    create_effect(cx, move |_| {
        fetch_with_retry();
    });

    view! { cx,
        div(class="retry-demo") {
            h3 { "Data Fetching with Retry Logic" }

            (match data_state.get() {
                FetchState::Idle => view! { cx, }.into_view(cx),
                FetchState::Loading => view! { cx,
                    div(class="loading") {
                        "Loading... " (if retry_count.get() > 0 {
                            format!("(Retry {}/{})", retry_count.get(), max_retries)
                        } else {
                            "".to_string()
                        })
                    }
                }.into_view(cx),
                FetchState::Success(data) => view! { cx,
                    div(class="success") {
                        "✅ " (data)
                    }
                }.into_view(cx),
                FetchState::Error(error) => view! { cx,
                    div(class="error") {
                        "❌ " (error)
                        button(on:click=move |_| fetch_with_retry()) { "Retry" }
                    }
                }.into_view(cx),
            })
        }
    }
}

#[component]
fn ParallelDataFetching(cx: Scope) -> impl IntoView {
    let (user_state, set_user_state) = create_signal(cx, FetchState::<User>::Idle);
    let (posts_state, set_posts_state) = create_signal(cx, FetchState::<Vec<Post>>::Idle);
    let (comments_state, set_comments_state) = create_signal(cx, FetchState::<Vec<String>>::Idle);

    let fetch_all_data = move || {
        // Start all requests simultaneously
        set_user_state.set(FetchState::Loading);
        set_posts_state.set(FetchState::Loading);
        set_comments_state.set(FetchState::Loading);

        // Fetch user
        set_timeout(move || {
            let user = User {
                id: 1,
                name: "Jane Doe".to_string(),
                email: "jane@example.com".to_string(),
                avatar_url: Some("https://via.placeholder.com/150".to_string()),
            };
            set_user_state.set(FetchState::Success(user));
        }, 1200);

        // Fetch posts
        set_timeout(move || {
            let posts = vec![
                Post {
                    id: 1,
                    title: "Parallel Fetching".to_string(),
                    content: "This demonstrates parallel data fetching!".to_string(),
                    author_id: 1,
                    created_at: "2024-01-25T10:00:00Z".to_string(),
                    tags: vec!["async".to_string(), "performance".to_string()],
                },
            ];
            set_posts_state.set(FetchState::Success(posts));
        }, 800);

        // Fetch comments
        set_timeout(move || {
            let comments = vec![
                "Great post!".to_string(),
                "Very informative".to_string(),
                "Thanks for sharing".to_string(),
            ];
            set_comments_state.set(FetchState::Success(comments));
        }, 600);
    };

    // Fetch on mount
    create_effect(cx, move |_| {
        fetch_all_data();
    });

    let all_loaded = create_memo(cx, move |_| {
        matches!(user_state.get(), FetchState::Success(_)) &&
        matches!(posts_state.get(), FetchState::Success(_)) &&
        matches!(comments_state.get(), FetchState::Success(_))
    });

    view! { cx,
        div(class="parallel-fetch-demo") {
            h3 { "Parallel Data Fetching" }

            (if all_loaded.get() {
                view! { cx,
                    div(class="all-data-loaded") {
                        "✅ All data loaded successfully!"
                    }
                }.into_view(cx)
            } else {
                view! { cx,
                    div(class="loading-progress") {
                        "Loading progress: "
                        (if matches!(user_state.get(), FetchState::Success(_)) { "User ✓ " } else { "User ⟳ " })
                        (if matches!(posts_state.get(), FetchState::Success(_)) { "Posts ✓ " } else { "Posts ⟳ " })
                        (if matches!(comments_state.get(), FetchState::Success(_)) { "Comments ✓" } else { "Comments ⟳" })
                    }
                }.into_view(cx)
            })

            button(on:click=move |_| fetch_all_data()) { "Reload All Data" }
        }
    }
}

fn main() {
    mount_to_body(|| view! {
        <div class="app">
            <UserProfile user_id=1/>
            <hr/>
            <DataFetchingWithCache/>
            <hr/>
            <RealTimeUpdates/>
            <hr/>
            <DataFetchingWithRetry/>
            <hr/>
            <ParallelDataFetching/>
        </div>
    })
}
```

## Key Concepts Demonstrated

### 1. Async Data Fetching with Loading States
```rust
let (user_state, set_user_state) = create_signal(cx, FetchState::<User>::Idle);

// Simulate API call
set_timeout(move || {
    let user = User { /* ... */ };
    set_user_state.set(FetchState::Success(user));
}, 1000);
```
- Using `FetchState` enum for different loading states
- Simulating async operations with `set_timeout`
- Reactive UI updates based on state changes

### 2. Sequential Data Fetching
```rust
create_effect(cx, move |_| {
    if let FetchState::Success(user) = user_state.get() {
        // Fetch related data only after user is loaded
        set_posts_state.set(FetchState::Loading);
        // ...
    }
});
```
- Dependent data fetching
- Effects that react to other signals
- Avoiding unnecessary API calls

### 3. Data Caching
```rust
let cache: StoredValue<HashMap<String, (FetchState<Vec<User>>, u64)>> =
    store_value(cx, HashMap::new());

// Check cache before making request
if let Some((cached_state, timestamp)) = cached_data {
    if now - timestamp < cache_duration {
        set_users_state.set(cached_state);
        return;
    }
}
```
- In-memory caching with TTL
- Cache invalidation
- Performance optimization

### 4. Real-Time Updates (SSE Simulation)
```rust
let interval = set_interval(move || {
    count += 1;
    let message = format!("Message {} at {}", count, js_sys::Date::now());
    set_messages.update(|messages| messages.push(message));
}, 2000);
```
- Simulating Server-Sent Events
- Dynamic list updates
- Cleanup with `on_cleanup`

### 5. Retry Logic
```rust
let attempt_fetch = move |attempt: u32| {
    set_timeout(move || {
        if success || attempt >= max_retries {
            // Handle success or final failure
        } else {
            attempt_fetch(attempt + 1); // Retry
        }
    }, 1000);
};
```
- Exponential backoff simulation
- Maximum retry limits
- User feedback during retries

### 6. Parallel Data Fetching
```rust
// Start all requests simultaneously
set_user_state.set(FetchState::Loading);
set_posts_state.set(FetchState::Loading);
set_comments_state.set(FetchState::Loading);

// All timeouts run in parallel
set_timeout(/* user */, 1200);
set_timeout(/* posts */, 800);
set_timeout(/* comments */, 600);
```
- Independent API calls
- Progress tracking
- Completion detection with memos

## Advanced Patterns

### Custom Hook for Data Fetching
```rust
#[derive(Clone)]
pub struct UseFetchResult<T> {
    pub data: ReadSignal<FetchState<T>>,
    pub refetch: Callback<()>,
    pub is_loading: ReadSignal<bool>,
    pub error: ReadSignal<Option<String>>,
}

pub fn use_fetch<T, F>(
    cx: Scope,
    fetch_fn: F,
) -> UseFetchResult<T>
where
    T: Clone + 'static,
    F: Fn() -> () + 'static,
{
    let (data, set_data) = create_signal(cx, FetchState::<T>::Idle);
    let (refetch_trigger, set_refetch_trigger) = create_signal(cx, ());

    let refetch = Callback::new(cx, move |_| {
        set_refetch_trigger.update(|_| ());
    });

    create_effect(cx, move |_| {
        refetch_trigger.get(); // React to refetch trigger
        set_data.set(FetchState::Loading);
        fetch_fn();
    });

    let is_loading = create_memo(cx, move |_| {
        matches!(data.get(), FetchState::Loading)
    });

    let error = create_memo(cx, move |_| {
        if let FetchState::Error(err) = data.get() {
            Some(err)
        } else {
            None
        }
    });

    UseFetchResult {
        data,
        refetch,
        is_loading,
        error,
    }
}
```

### Error Boundary Component
```rust
#[component]
pub fn ErrorBoundary<T>(
    cx: Scope,
    children: Children,
    fallback: Callback<String, View>,
) -> impl IntoView
where
    T: IntoView,
{
    let (error, set_error) = create_signal(cx, None::<String>);

    // In a real implementation, this would catch panics
    // For now, we'll just render children

    move || {
        if let Some(err) = error.get() {
            fallback.call(err)
        } else {
            children(cx).into_view(cx)
        }
    }
}
```

### Data Fetching with Suspense
```rust
#[component]
pub fn SuspenseBoundary(
    cx: Scope,
    children: Children,
    fallback: View,
) -> impl IntoView {
    let (is_pending, set_is_pending) = create_signal(cx, true);

    // Simulate resource loading
    create_effect(cx, move |_| {
        set_timeout(move || {
            set_is_pending.set(false);
        }, 1000);
    });

    move || {
        if is_pending.get() {
            fallback.clone()
        } else {
            children(cx).into_view(cx)
        }
    }
}
```

## Best Practices

### 1. State Management
```rust
// ✅ Good: Use dedicated state enum
#[derive(Clone)]
pub enum FetchState<T> {
    Idle,
    Loading,
    Success(T),
    Error(String),
}

// ❌ Avoid: Multiple boolean flags
let (is_loading, set_is_loading) = create_signal(cx, false);
let (error, set_error) = create_signal(cx, None::<String>);
let (data, set_data) = create_signal(cx, None::<T>);
```

### 2. Error Handling
```rust
// ✅ Good: Handle all error cases
match result {
    Ok(data) => set_state.set(FetchState::Success(data)),
    Err(err) => {
        log::error!("Failed to fetch data: {:?}", err);
        set_state.set(FetchState::Error("Failed to load data".to_string()));
    }
}

// ✅ Good: User-friendly error messages
let error_message = match error_type {
    NetworkError => "Please check your internet connection",
    ServerError => "Server is temporarily unavailable",
    AuthError => "Please log in again",
    _ => "An unexpected error occurred",
};
```

### 3. Loading States
```rust
// ✅ Good: Progressive loading
div(class="loading") {
    (if is_initial_load.get() {
        "Loading for the first time..."
    } else {
        "Refreshing..."
    })
}

// ✅ Good: Skeleton loading
div(class="skeleton") {
    div(class="skeleton-avatar") </div>
    div(class="skeleton-text") </div>
    div(class="skeleton-text short") </div>
}
```

### 4. Caching Strategy
```rust
// ✅ Good: Cache with invalidation
struct CacheEntry<T> {
    data: T,
    timestamp: u64,
    ttl: u64,
}

impl<T> CacheEntry<T> {
    fn is_expired(&self) -> bool {
        js_sys::Date::now() as u64 > self.timestamp + self.ttl
    }
}

// ✅ Good: Cache keys
let cache_key = format!("user_{}", user_id);
let cache_key = format!("posts_user_{}_page_{}", user_id, page);
```

### 5. Performance Optimization
```rust
// ✅ Good: Debounced requests
let debounced_fetch = debounce(cx, move || {
    fetch_data()
}, 300);

// ✅ Good: Request deduplication
static REQUESTS: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

let request_key = format!("user_{}", user_id);
if !REQUESTS.lock().unwrap().contains(&request_key) {
    REQUESTS.lock().unwrap().insert(request_key.clone());
    // Make request
}
```

## Testing Data Fetching

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_fetch_state_transitions() {
        let runtime = create_runtime();
        let scope = create_scope(runtime, |cx| {
            let (state, set_state) = create_signal(cx, FetchState::<String>::Idle);

            // Test initial state
            assert!(matches!(state.get(), FetchState::Idle));

            // Test loading state
            set_state.set(FetchState::Loading);
            assert!(matches!(state.get(), FetchState::Loading));

            // Test success state
            let test_data = "Hello World".to_string();
            set_state.set(FetchState::Success(test_data.clone()));
            assert!(matches!(state.get(), FetchState::Success(data) if data == test_data));

            // Test error state
            let error_msg = "Network error".to_string();
            set_state.set(FetchState::Error(error_msg.clone()));
            assert!(matches!(state.get(), FetchState::Error(err) if err == error_msg));
        });
        dispose_runtime(runtime);
    }

    #[test]
    fn test_sequential_fetching() {
        let runtime = create_runtime();
        let scope = create_scope(runtime, |cx| {
            let (user_state, set_user_state) = create_signal(cx, FetchState::<User>::Idle);
            let (posts_state, set_posts_state) = create_signal(cx, FetchState::<Vec<Post>>::Idle);

            // Simulate user loading first
            let user = User {
                id: 1,
                name: "Test User".to_string(),
                email: "test@example.com".to_string(),
                avatar_url: None,
            };
            set_user_state.set(FetchState::Success(user.clone()));

            // Posts should be idle initially
            assert!(matches!(posts_state.get(), FetchState::Idle));

            // In real implementation, an effect would trigger posts loading
            // Here we simulate the effect manually
            set_posts_state.set(FetchState::Loading);
            assert!(matches!(posts_state.get(), FetchState::Loading));
        });
        dispose_runtime(runtime);
    }

    #[test]
    fn test_retry_logic() {
        let runtime = create_runtime();
        let scope = create_scope(runtime, |cx| {
            let (retry_count, set_retry_count) = create_signal(cx, 0);
            let max_retries = 3;

            // Simulate retry increment
            set_retry_count.update(|count| *count += 1);
            assert_eq!(retry_count.get(), 1);

            // Test max retries
            for _ in 1..max_retries {
                set_retry_count.update(|count| *count += 1);
            }
            assert_eq!(retry_count.get(), max_retries);
        });
        dispose_runtime(runtime);
    }
}
```

## Real-World Integration

### With Real HTTP Client
```rust
use reqwest;

#[component]
fn RealDataFetching(cx: Scope) -> impl IntoView {
    let (data, set_data) = create_signal(cx, FetchState::<serde_json::Value>::Idle);

    let fetch_data = create_action(cx, |url: &String| {
        let url = url.clone();
        async move {
            match reqwest::get(&url).await {
                Ok(response) => match response.json::<serde_json::Value>().await {
                    Ok(json) => FetchState::Success(json),
                    Err(_) => FetchState::Error("Failed to parse JSON".to_string()),
                },
                Err(_) => FetchState::Error("Network request failed".to_string()),
            }
        }
    });

    create_effect(cx, move |_| {
        if let Some(result) = fetch_data.value().get() {
            set_data.set(result);
        }
    });

    // Usage
    fetch_data.dispatch("https://api.example.com/data".to_string());
}
```

### With GraphQL
```rust
#[component]
fn GraphQLDataFetching(cx: Scope) -> impl IntoView {
    let query = r#"
        query GetUser($id: ID!) {
            user(id: $id) {
                id
                name
                email
                posts {
                    id
                    title
                    content
                }
            }
        }
    "#;

    // Implementation would use a GraphQL client
    // Similar pattern to HTTP fetching
}
```

## Performance Considerations

### 1. Request Batching
```rust
// Batch multiple requests
let batch_request = create_action(cx, |requests: &Vec<String>| {
    async move {
        let futures: Vec<_> = requests.iter()
            .map(|url| reqwest::get(url))
            .collect();

        let results = futures::future::join_all(futures).await;
        // Process results...
    }
});
```

### 2. Request Cancellation
```rust
use futures::future::AbortHandle;

#[component]
fn CancellableFetch(cx: Scope) -> impl IntoView {
    let (abort_handle, set_abort_handle) = create_signal(cx, None::<AbortHandle>);

    let fetch_data = move || {
        let (handle, registration) = AbortHandle::new_pair();
        set_abort_handle.set(Some(handle));

        // Use registration with the future
    };

    let cancel_fetch = move |_| {
        if let Some(handle) = abort_handle.get() {
            handle.abort();
        }
    };

    // ...
}
```

### 3. Memory Management
```rust
// Clean up large data structures
create_effect(cx, move |_| {
    // When component unmounts or data changes
    on_cleanup(cx, || {
        // Clear large caches
        // Cancel pending requests
        // Clean up resources
    });
});
```

## Related Examples

- [Basic Counter Example](basic-counter.md) - Simple reactive state
- [Todo App Example](todo-app.md) - Complex state management with persistence
- [Form Handling Example](form-handling.md) - Controlled inputs with validation

## Next Steps

1. Add offline support with Service Workers
2. Implement optimistic updates
3. Add request/response interceptors
4. Create a data fetching library
5. Add support for WebSockets
6. Implement pagination and infinite scrolling
7. Add request deduplication
8. Create a caching layer with localStorage/indexedDB

This data fetching example demonstrates how to build robust, performant data loading experiences with proper error handling, caching, and real-time updates in Leptos.