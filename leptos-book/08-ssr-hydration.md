# 08: SSR & Hydration

## Overview

Server-Side Rendering (SSR) and hydration are crucial for modern web applications, providing better performance, SEO, and user experience. This chapter explores Leptos' SSR capabilities, hydration strategies, and best practices for building universal applications that work seamlessly on both server and client.

## Understanding SSR in Leptos

### What is Server-Side Rendering?

Server-Side Rendering (SSR) is the process of rendering web pages on the server instead of in the browser. This provides several benefits:

- **Better SEO**: Search engines can crawl fully rendered content
- **Faster initial page loads**: Users see content immediately
- **Improved performance**: Less JavaScript to parse and execute initially
- **Better accessibility**: Screen readers can access content immediately

### Leptos SSR Architecture

Leptos provides a unified architecture for both SSR and client-side rendering:

```rust
use leptos::*;

// Same component works on both server and client
#[component]
pub fn UniversalComponent() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    
    view! {
        <div>
            <h1>"Universal Component"</h1>
            <p>"Count: " {count}</p>
            <button on:click=move |_| set_count.update(|c| *c + 1)>
                "Increment"
            </button>
        </div>
    }
}
```

## Setting Up SSR

### Cargo.toml Configuration

Configure your project for SSR:

```toml
[package]
name = "my-ssr-app"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = { version = "0.6", features = ["ssr"] }
leptos_router = { version = "0.6", features = ["ssr"] }
leptos_meta = { version = "0.6", features = ["ssr"] }
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["fs"] }

[features]
default = ["hydrate"]
hydrate = ["leptos/hydrate", "leptos_router/hydrate", "leptos_meta/hydrate"]
ssr = ["leptos/ssr", "leptos_router/ssr", "leptos_meta/ssr"]
```

### Server Setup

Create an Axum server for SSR:

```rust
use axum::{routing::get, Router};
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use std::env;

#[tokio::main]
async fn main() {
    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(leptos_axum::file_and_error_handler(&leptos_options))
        .with_state(leptos_options);

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
```

### App Configuration

Configure your Leptos app for SSR:

```rust
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=leptos_options.clone() />
                <HydrationScripts options=leptos_options.clone()/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <header>
                <h1>"My SSR App"</h1>
                <nav>
                    <a href="/">"Home"</a>
                    <a href="/about">"About"</a>
                </nav>
            </header>
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/about" view=AboutPage/>
                </Routes>
            </main>
        </Router>
    }
}
```

## Hydration Strategies

### Progressive Hydration

Load and hydrate components progressively:

```rust
use leptos::*;

// Component that hydrates immediately
#[component]
pub fn CriticalContent() -> impl IntoView {
    let (data, set_data) = create_signal("Loading...".to_string());
    
    // Load critical data immediately
    create_effect(move |_| {
        // Simulate data loading
        set_timeout(move || {
            set_data.set("Critical content loaded!".to_string());
        }, std::time::Duration::from_millis(100));
    });
    
    view! {
        <div class="critical">
            <h2>"Critical Content"</h2>
            <p>{data}</p>
        </div>
    }
}

// Component that hydrates later
#[component]
pub fn DeferredContent() -> impl IntoView {
    view! {
        <Suspense fallback=move || view! { <div>"Loading deferred content..."</div> }>
            <DeferredComponent/>
        </Suspense>
    }
}

#[component]
pub fn DeferredComponent() -> impl IntoView {
    let (data, set_data) = create_signal("Loading...".to_string());
    
    // Load non-critical data after hydration
    create_effect(move |_| {
        set_timeout(move || {
            set_data.set("Deferred content loaded!".to_string());
        }, std::time::Duration::from_millis(2000));
    });
    
    view! {
        <div class="deferred">
            <h3>"Deferred Content"</h3>
            <p>{data}</p>
        </div>
    }
}
```

### Selective Hydration

Control which parts of your app hydrate:

```rust
use leptos::*;

// Server-only component (no hydration)
#[component]
pub fn ServerOnly() -> impl IntoView {
    view! {
        <div class="server-only">
            <p>"This content is only rendered on the server"</p>
            <p>"Current server time: " {chrono::Utc::now().to_string()}</p>
        </div>
    }
}

// Client-only component (hydrates but not server-rendered)
#[component]
pub fn ClientOnly() -> impl IntoView {
    let (mounted, set_mounted) = create_signal(false);
    
    create_effect(move |_| {
        set_mounted.set(true);
    });
    
    move || if mounted.get() {
        view! {
            <div class="client-only">
                <p>"This content only appears after hydration"</p>
                <p>"Client-side timestamp: " {js_sys::Date::now() as u64}</p>
            </div>
        }
    } else {
        view! { <div>"Loading client content..."</div> }
    }
}

// Universal component (server-rendered and hydrated)
#[component]
pub fn Universal() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    
    view! {
        <div class="universal">
            <h3>"Universal Component"</h3>
            <p>"Count: " {count}</p>
            <button on:click=move |_| set_count.update(|c| *c + 1)>
                "Increment"
            </button>
        </div>
    }
}
```

## Data Fetching in SSR

### Server-Side Data Fetching

Fetch data on the server for better performance:

```rust
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub content: String,
}

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let post_id = move || params.get("id").unwrap_or_default();
    
    // Server-side data fetching
    let post = create_resource(
        move || post_id(),
        |id| async move {
            if id.is_empty() {
                return None;
            }
            
            // This runs on the server during SSR
            fetch_post(&id).await
        }
    );
    
    view! {
        <Suspense fallback=move || view! { <p>"Loading post..."</p> }>
            {move || post.get().flatten().map(|post| view! {
                <article>
                    <h1>{&post.title}</h1>
                    <div>{&post.content}</div>
                </article>
            })}
        </Suspense>
    }
}

#[server(FetchPost, "/api")]
pub async fn fetch_post(id: &str) -> Result<Option<Post>, ServerFnError> {
    // Server-side database query
    // This only runs on the server
    let post = sqlx::query_as!(
        Post,
        "SELECT id, title, content FROM posts WHERE id = ?",
        id
    )
    .fetch_optional(&pool)
    .await?;
    
    Ok(post)
}
```

### Client-Side Data Fetching

Handle client-side data fetching for dynamic content:

```rust
use leptos::*;

// Client-side data fetching
#[component]
pub fn ClientData() -> impl IntoView {
    let (search_term, set_search_term) = create_signal(String::new());
    
    // Client-side search
    let search_results = create_resource(
        move || search_term.get(),
        |term| async move {
            if term.is_empty() {
                return vec![];
            }
            
            // This runs on the client after hydration
            search_api(&term).await.unwrap_or_default()
        }
    );
    
    view! {
        <div>
            <input
                type="text"
                placeholder="Search..."
                prop:value=search_term
                on:input=move |ev| set_search_term.set(event_target_value(&ev))
            />
            
            <Suspense fallback=move || view! { <p>"Searching..."</p> }>
                <ul>
                    {move || search_results.get().into_iter().map(|result| view! {
                        <li>{result}</li>
                    }).collect::<Vec<_>>()}
                </ul>
            </Suspense>
        </div>
    }
}

#[server(SearchApi, "/api")]
pub async fn search_api(term: &str) -> Result<Vec<String>, ServerFnError> {
    // This can run on either server or client
    // In SSR, it runs on server; after hydration, it runs on client
    let results = vec![
        format!("Result 1 for '{}'", term),
        format!("Result 2 for '{}'", term),
    ];
    
    Ok(results)
}
```

## State Management in SSR

### Server-Safe State

Ensure state works correctly in SSR environments:

```rust
use leptos::*;

// Server-safe state management
#[component]
pub fn ServerSafeState() -> impl IntoView {
    // Use RwSignal for complex state that needs both read and write
    let state = create_rw_signal(AppState::default());
    
    // Server-safe local storage
    let saved_count = create_resource(
        || (),
        |_| async move {
            // This will be None on server, Some(value) on client
            get_local_storage("count").await.unwrap_or(0)
        }
    );
    
    // Sync server state with client state
    create_effect(move |_| {
        if let Some(count) = saved_count.get() {
            state.update(|s| s.count = count);
        }
    });
    
    // Save to localStorage when state changes
    create_effect(move |_| {
        let count = state.read().count;
        spawn_local(async move {
            set_local_storage("count", count).await;
        });
    });
    
    view! {
        <div>
            <p>"Count: " {move || state.read().count}</p>
            <button on:click=move |_| state.update(|s| s.count += 1)>
                "Increment"
            </button>
        </div>
    }
}

#[derive(Clone, Default)]
pub struct AppState {
    pub count: i32,
    pub user: Option<User>,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
}

// Server-safe localStorage functions
#[cfg(feature = "hydrate")]
async fn get_local_storage(key: &str) -> Option<i32> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let value = storage.get_item(key).ok()??;
    value.parse().ok()
}

#[cfg(feature = "hydrate")]
async fn set_local_storage(key: &str, value: i32) {
    if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
        let _ = storage.set_item(key, &value.to_string());
    }
}

#[cfg(not(feature = "hydrate"))]
async fn get_local_storage(_key: &str) -> Option<i32> {
    None
}

#[cfg(not(feature = "hydrate"))]
async fn set_local_storage(_key: &str, _value: i32) {
    // No-op on server
}
```

## Routing in SSR

### Server-Side Routing

Configure routing for SSR:

```rust
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage ssr=SsrMode::Async/>
                <Route path="/blog/:id" view=BlogPost ssr=SsrMode::Async/>
                <Route path="/dashboard" view=DashboardPage ssr=SsrMode::InOrder/>
                <Route path="/*" view=NotFoundPage/>
            </Routes>
        </Router>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    // This component will be server-rendered
    view! {
        <div>
            <h1>"Welcome to the Home Page"</h1>
            <p>"This content is server-rendered for better SEO"</p>
        </div>
    }
}

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.get("id").unwrap_or_default();
    
    let post = create_resource(
        move || id(),
        |id| async move {
            // Fetch post data on server
            get_post(&id).await
        }
    );
    
    view! {
        <Suspense fallback=move || view! { <p>"Loading post..."</p> }>
            {move || post.get().map(|post| view! {
                <article>
                    <h1>{&post.title}</h1>
                    <div>{&post.content}</div>
                </article>
            })}
        </Suspense>
    }
}

#[server(GetPost, "/api")]
pub async fn get_post(id: &str) -> Result<Post, ServerFnError> {
    // Server-side database query
    let post = sqlx::query_as!(
        Post,
        "SELECT * FROM posts WHERE id = ?",
        id
    )
    .fetch_one(&pool)
    .await?;
    
    Ok(post)
}
```

## Meta Tags and SEO

### Dynamic Meta Tags

Set dynamic meta tags for better SEO:

```rust
use leptos::*;
use leptos_meta::*;

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.get("id").unwrap_or_default();
    
    let post = create_resource(
        move || id(),
        |id| async move { get_post(&id).await }
    );
    
    // Set dynamic meta tags
    view! {
        <Suspense fallback=move || view! { <div>"Loading..."</div> }>
            {move || post.get().map(|post| view! {
                <MetaTags>
                    <Title text=move || format!("{} - My Blog", post.title)/>
                    <Meta name="description" content=move || post.excerpt.clone()/>
                    <Meta property="og:title" content=move || post.title.clone()/>
                    <Meta property="og:description" content=move || post.excerpt.clone()/>
                    <Meta property="og:image" content=move || post.image_url.clone()/>
                    <Link rel="canonical" href=move || format!("/blog/{}", post.id)/>
                </MetaTags>
                
                <article>
                    <h1>{&post.title}</h1>
                    <div>{&post.content}</div>
                </article>
            })}
        </Suspense>
    }
}
```

## Error Handling in SSR

### Server-Side Error Boundaries

Handle errors gracefully in SSR:

```rust
use leptos::*;

// Error boundary for SSR
#[component]
pub fn ErrorBoundary<F, IV>(
    children: F,
    fallback: Fallback<IV>
) -> impl IntoView 
where
    F: Fn() -> IV + 'static,
    IV: IntoView + 'static,
    Fallback<IV>: Fn() -> IV + 'static,
{
    let (error, set_error) = create_signal(None::<String>);
    
    // Catch panics in SSR
    let children = store_value(children);
    
    create_effect(move |_| {
        // In a real implementation, you'd use error boundaries
        // For now, this is a simplified version
    });
    
    move || {
        if let Some(err) = error.get() {
            fallback()
        } else {
            children.get_value()()
        }
    }
}

// Usage
#[component]
pub fn App() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|| view! { <div>"Something went wrong!"</div> }>
            <Router>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/error-demo" view=ErrorDemoPage/>
                </Routes>
            </Router>
        </ErrorBoundary>
    }
}

#[component]
pub fn ErrorDemoPage() -> impl IntoView {
    // This might cause an error
    let data = create_resource(
        || (),
        |_| async move {
            // Simulate an error
            Err("Something went wrong on the server!".to_string())
        }
    );
    
    view! {
        <div>
            <h1>"Error Demo"</h1>
            {move || data.get().map(|result| match result {
                Ok(data) => view! { <p>{data}</p> },
                Err(err) => view! { <p class="error">"Error: " {err}</p> }
            })}
        </div>
    }
}
```

## Performance Optimization

### Code Splitting

Split your code for better loading performance:

```rust
use leptos::*;
use leptos_router::*;

// Lazy-loaded routes
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
                // These components will be loaded lazily
                <Route path="/admin" view=AdminPage/>
                <Route path="/reports" view=ReportsPage/>
                <Route path="/settings" view=SettingsPage/>
            </Routes>
        </Router>
    }
}

// Lazy-loaded components
#[component]
pub fn AdminPage() -> impl IntoView {
    view! {
        <Suspense fallback=move || view! { <p>"Loading admin panel..."</p> }>
            <AdminDashboard/>
        </Suspense>
    }
}

#[component]
pub fn ReportsPage() -> impl IntoView {
    view! {
        <Suspense fallback=move || view! { <p>"Loading reports..."</p> }>
            <ReportsView/>
        </Suspense>
    }
}

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <Suspense fallback=move || view! { <p>"Loading settings..."</p> }>
            <SettingsPanel/>
        </Suspense>
    }
}

// These would be in separate modules in a real app
#[component]
pub fn AdminDashboard() -> impl IntoView {
    view! { <div>"Admin Dashboard"</div> }
}

#[component]
pub fn ReportsView() -> impl IntoView {
    view! { <div>"Reports View"</div> }
}

#[component]
pub fn SettingsPanel() -> impl IntoView {
    view! { <div>"Settings Panel"</div> }
}
```

### Caching Strategies

Implement caching for better performance:

```rust
use leptos::*;
use std::collections::HashMap;

// Server-side caching
#[derive(Clone, Default)]
pub struct Cache {
    data: std::sync::RwLock<HashMap<String, (String, std::time::Instant)>>,
    ttl: std::time::Duration,
}

impl Cache {
    pub fn new(ttl: std::time::Duration) -> Self {
        Self {
            data: Default::default(),
            ttl,
        }
    }
    
    pub fn get(&self, key: &str) -> Option<String> {
        let data = self.data.read().unwrap();
        if let Some((value, timestamp)) = data.get(key) {
            if timestamp.elapsed() < self.ttl {
                return Some(value.clone());
            }
        }
        None
    }
    
    pub fn set(&self, key: String, value: String) {
        let mut data = self.data.write().unwrap();
        data.insert(key, (value, std::time::Instant::now()));
    }
}

// Usage in server functions
#[server(GetCachedData, "/api")]
pub async fn get_cached_data(key: String) -> Result<String, ServerFnError> {
    static CACHE: once_cell::sync::Lazy<Cache> = once_cell::sync::Lazy::new(|| {
        Cache::new(std::time::Duration::from_secs(300)) // 5 minute TTL
    });
    
    if let Some(cached) = CACHE.get(&key) {
        return Ok(cached);
    }
    
    // Fetch fresh data
    let data = fetch_fresh_data(&key).await?;
    
    // Cache the result
    CACHE.set(key, data.clone());
    
    Ok(data)
}
```

## Testing SSR Applications

### SSR Testing

Test your SSR implementation:

```rust
#[cfg(test)]
mod ssr_tests {
    use super::*;
    use leptos::*;
    
    #[test]
    fn test_ssr_rendering() {
        let html = ssr::render_to_string(|| view! {
            <div>
                <h1>"Test Page"</h1>
                <p>"This is a test"</p>
            </div>
        });
        
        assert!(html.contains("<h1>Test Page</h1>"));
        assert!(html.contains("<p>This is a test</p>"));
    }
    
    #[test]
    fn test_hydration() {
        // Test that components hydrate correctly
        let runtime = create_runtime();
        
        let (count, set_count) = create_signal(0);
        
        // Simulate hydration
        set_count.set(5);
        
        assert_eq!(count.get(), 5);
        
        runtime.dispose();
    }
    
    #[test]
    fn test_server_functions() {
        // Test server function behavior
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let result = get_test_data().await;
            assert!(result.is_ok());
        });
    }
}

#[server(GetTestData, "/api")]
pub async fn get_test_data() -> Result<String, ServerFnError> {
    Ok("test data".to_string())
}
```

## Deployment Considerations

### Build Configuration

Configure your build for SSR:

```toml
# In Cargo.toml
[features]
default = ["hydrate"]
hydrate = ["leptos/hydrate", "leptos_router/hydrate", "leptos_meta/hydrate"]
ssr = ["leptos/ssr", "leptos_router/ssr", "leptos_meta/ssr"]
```

### Environment Variables

Handle environment-specific configuration:

```rust
use leptos::*;

// Environment-aware configuration
#[component]
pub fn App() -> impl IntoView {
    let api_url = if cfg!(feature = "ssr") {
        // Server-side: use internal API
        std::env::var("INTERNAL_API_URL").unwrap_or_else(|_| "http://localhost:3001".to_string())
    } else {
        // Client-side: use public API
        std::env::var("PUBLIC_API_URL").unwrap_or_else(|_| "/api".to_string())
    };
    
    provide_context(api_url);
    
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
            </Routes>
        </Router>
    }
}
```

## Best Practices

### SSR Checklist

- [ ] Configure proper SSR features in Cargo.toml
- [ ] Set up Axum server with Leptos integration
- [ ] Use Suspense for async data loading
- [ ] Implement proper error boundaries
- [ ] Handle server/client state synchronization
- [ ] Configure meta tags for SEO
- [ ] Implement code splitting for performance
- [ ] Set up proper caching strategies
- [ ] Test both SSR and hydration paths
- [ ] Monitor performance and SEO metrics

### Common SSR Patterns

```rust
use leptos::*;

// Pattern 1: Conditional rendering based on environment
#[component]
pub fn EnvironmentAware() -> impl IntoView {
    if cfg!(feature = "ssr") {
        view! { <div>"Server-rendered content"</div> }
    } else {
        view! { <div>"Client-hydrated content"</div> }
    }
}

// Pattern 2: Progressive enhancement
#[component]
pub fn Progressive() -> impl IntoView {
    let (enhanced, set_enhanced) = create_signal(false);
    
    create_effect(move |_| {
        set_enhanced.set(true);
    });
    
    view! {
        <div class="progressive" class:enhanced=enhanced>
            <p>"Basic content (works without JS)"</p>
            {move || enhanced.get().then(|| view! {
                <p>"Enhanced content (requires JS)"</p>
            })}
        </div>
    }
}

// Pattern 3: Server-safe data fetching
#[component]
pub fn SafeDataFetch() -> impl IntoView {
    let data = create_resource(
        || (),
        |_| async move {
            // This works on both server and client
            fetch_data().await.unwrap_or_default()
        }
    );
    
    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <div>{move || data.get()}</div>
        </Suspense>
    }
}
```

---

**Next:** [09: Testing](09-testing.md) - Learn about testing strategies for Leptos applications.