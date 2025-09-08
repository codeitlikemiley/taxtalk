# 10: Deployment

## Overview

Deploying Leptos applications requires understanding the different rendering modes (CSR, SSR, Hydration) and choosing the right hosting strategy. This chapter covers build configuration, deployment options, performance optimization, and monitoring for production Leptos applications.

## Build Configuration

### Cargo.toml Configuration

Configure your Leptos application for production builds:

```toml
[package]
name = "my-leptos-app"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = { version = "0.6", features = ["csr"] }
leptos_router = "0.6"
leptos_meta = "0.6"
console_log = "1"
console_error_panic_hook = "0.1"
wasm-bindgen = "0.2"

[features]
default = ["csr"]
csr = ["leptos/csr"]
hydrate = ["leptos/hydrate"]
ssr = ["leptos/ssr", "leptos_meta/ssr", "leptos_router/ssr"]

[profile.release]
codegen-units = 1
lto = true
opt-level = "z"
panic = "abort"
strip = true
```

### Trunk Configuration

Configure Trunk for optimal builds:

```toml
# Trunk.toml
[build]
target = "index.html"
dist = "dist"
public_url = "/"

[watch]
ignore = [
    "./target/",
    "./dist/",
    "./node_modules/",
]

[[hooks]]
stage = "pre_build"
command = "cargo"
command_arguments = ["clean"]

[[hooks]]
stage = "post_build"
command = "echo"
command_arguments = ["Build completed successfully"]
```

### Build Scripts

Create build scripts for different environments:

```bash
#!/bin/bash
# build.sh

# Clean previous builds
rm -rf dist/

# Build for production
trunk build --release

# Optimize WASM
wasm-opt -Oz dist/*.wasm -o dist/optimized.wasm
mv dist/optimized.wasm dist/*.wasm

# Generate service worker for caching
npx workbox generateSW workbox-config.js

echo "Build completed successfully"
```

```javascript
// workbox-config.js
module.exports = {
  globDirectory: 'dist/',
  globPatterns: [
    '**/*.{html,js,wasm,png,svg,jpg,gif,json,css,txt,ico}'
  ],
  swDest: 'dist/sw.js',
  clientsClaim: true,
  skipWaiting: true,
  runtimeCaching: [{
    urlPattern: /^https:\/\/api\./,
    handler: 'NetworkFirst',
    options: {
      cacheName: 'api-cache',
      expiration: {
        maxEntries: 100,
        maxAgeSeconds: 24 * 60 * 60 // 24 hours
      }
    }
  }]
};
```

## Rendering Modes

### Client-Side Rendering (CSR)

Best for:
- Marketing websites
- Admin dashboards
- Applications with heavy client-side interactions
- When SEO is not critical

```rust
// CSR Configuration
#[cfg(feature = "csr")]
fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    
    leptos::mount_to_body(|| {
        view! { <App/> }
    });
}
```

### Server-Side Rendering (SSR)

Best for:
- Content-heavy websites
- SEO-critical applications
- Fast initial page loads
- Social media sharing

```rust
// SSR Configuration
#[cfg(feature = "ssr")]
fn main() {
    use leptos::*;
    use leptos_router::*;
    
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    
    let routes = generate_route_list(|| view! { <App/> });
    
    let app = Router::new()
        .routes(&routes)
        .fallback(|| view! { <NotFound/> })
        .with_state(leptos_options);
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

### Hydration

Best for:
- Complex interactive applications
- Applications requiring both SEO and interactivity
- Progressive enhancement

```rust
// Hydration Configuration
#[cfg(feature = "hydrate")]
fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    
    leptos::hydrate_body(|| {
        view! { <App/> }
    });
}
```

## Deployment Platforms

### Vercel

Deploy to Vercel for SSR and static sites:

```javascript
// vercel.json
{
  "buildCommand": "trunk build --release",
  "outputDirectory": "dist",
  "framework": null,
  "rewrites": [
    { "source": "/(.*)", "destination": "/index.html" }
  ]
}
```

```toml
# Cargo.toml for Vercel
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[features]
default = ["ssr"]
ssr = ["leptos/ssr", "leptos_meta/ssr", "leptos_router/ssr"]

[dependencies]
leptos = { version = "0.6", features = ["ssr"] }
leptos_axum = "0.6"
tokio = { version = "1", features = ["full"] }
```

### Netlify

Deploy to Netlify with edge functions:

```toml
# netlify.toml
[build]
  command = "trunk build --release"
  publish = "dist"

[functions]
  directory = "netlify/functions"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```

```javascript
// netlify/functions/api.js
export async function handler(event) {
  // Handle API routes
  return {
    statusCode: 200,
    body: JSON.stringify({ message: "Hello from Netlify!" }),
  };
}
```

### Railway

Deploy full-stack applications to Railway:

```yaml
# railway.toml
[build]
builder = "NIXPACKS"

[deploy]
healthcheckPath = "/"
restartPolicyType = "ON_FAILURE"
```

```rust
// main.rs for Railway
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use leptos::*;
    use leptos_axum::*;
    
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    
    let app = Router::new()
        .leptos_routes(&conf.leptos_options, || view! { <App/> })
        .fallback(file_and_error_handler)
        .with_state(conf.leptos_options);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Fly.io

Deploy globally distributed applications:

```toml
# fly.toml
app = "my-leptos-app"
primary_region = "iad"

[build]
  builder = "paketobuildpacks/builder:base"
  buildpacks = ["gcr.io/paketo-buildpacks/rust"]

[http_service]
  internal_port = 3000
  force_https = true
  auto_stop_machines = true
  auto_start_machines = true
  min_machines_running = 1

[[vm]]
  cpu_kind = "shared"
  cpus = 1
  memory_mb = 1024
```

### AWS

Deploy to AWS with multiple options:

#### Lambda + API Gateway (Serverless)

```rust
// main.rs for Lambda
use lambda_http::{run, service_fn, Body, Error, Request, Response};

#[cfg(feature = "ssr")]
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let conf = get_configuration(None).await.unwrap();
    
    // Handle SSR request
    let html = leptos::ssr::render_to_string(|| {
        view! { <App/> }
    });
    
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(html.into())
        .map_err(Box::new)?)
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
```

#### ECS + Fargate (Containerized)

```dockerfile
# Dockerfile
FROM rust:1.70-slim as builder

WORKDIR /app
COPY . .

RUN cargo build --release --features ssr

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/my-app /usr/local/bin/my-app

EXPOSE 3000

CMD ["my-app"]
```

```yaml
# docker-compose.yml for local development
version: '3.8'
services:
  app:
    build: .
    ports:
      - "3000:3000"
    environment:
      - LEPTOS_SITE_ADDR=0.0.0.0:3000
      - LEPTOS_SITE_ROOT=.
```

### Docker

Create optimized Docker images:

```dockerfile
# Multi-stage Dockerfile
FROM rust:1.70-alpine as builder

# Install required packages
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --features ssr
RUN rm -rf src

# Copy source code
COPY src ./src

# Build the application
RUN touch src/main.rs
RUN cargo build --release --features ssr

# Runtime stage
FROM alpine:3.18

RUN apk add --no-cache ca-certificates tzdata
RUN addgroup -g 1001 -S appuser && adduser -S -D -H -u 1001 -h /app -s /sbin/nologin -G appuser -g appuser appuser

WORKDIR /app

COPY --from=builder /app/target/release/my-app /app/my-app

USER appuser

EXPOSE 3000

CMD ["./my-app"]
```

## Performance Optimization

### Bundle Optimization

Optimize your bundle size:

```javascript
// webpack.config.js (if using custom bundler)
const path = require('path');

module.exports = {
  entry: './pkg/index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },
  optimization: {
    splitChunks: {
      chunks: 'all',
      cacheGroups: {
        vendor: {
          test: /[\\/]node_modules[\\/]/,
          name: 'vendors',
          chunks: 'all',
        },
      },
    },
  },
};
```

### Code Splitting

Implement lazy loading:

```rust
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
                <Route path="/dashboard" view=|| view! { <Suspense fallback=|| "Loading..."><Dashboard/></Suspense> }/>
                <Route path="/admin" view=|| view! { <Suspense fallback=|| "Loading..."><AdminPanel/></Suspense> }/>
            </Routes>
        </Router>
    }
}

#[component]
pub fn Dashboard() -> impl IntoView {
    // Heavy component loaded on demand
    view! {
        <div>
            <h1>"Dashboard"</h1>
            // Complex dashboard content
        </div>
    }
}
```

### Caching Strategies

Implement effective caching:

```rust
use leptos::*;
use leptos_router::*;

#[component]
pub fn CachedData() -> impl IntoView {
    let data = create_resource(
        || (),
        |_| async move {
            // Check cache first
            if let Some(cached) = get_cached_data().await {
                return cached;
            }
            
            // Fetch from API
            let fresh = fetch_data().await?;
            
            // Cache the result
            cache_data(&fresh).await;
            
            Ok(fresh)
        }
    );
    
    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            {move || data.get().map(|result| match result {
                Ok(data) => view! { <DataDisplay data/> },
                Err(_) => view! { <ErrorMessage/> }
            })}
        </Suspense>
    }
}
```

### Service Workers

Implement offline functionality:

```javascript
// public/sw.js
const CACHE_NAME = 'my-app-v1';
const urlsToCache = [
  '/',
  '/static/css/main.css',
  '/static/js/main.js',
  '/static/manifest.json'
];

self.addEventListener('install', event => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(cache => cache.addAll(urlsToCache))
  );
});

self.addEventListener('fetch', event => {
  event.respondWith(
    caches.match(event.request)
      .then(response => {
        if (response) {
          return response;
        }
        return fetch(event.request);
      })
  );
});
```

## Monitoring and Analytics

### Error Tracking

Set up error monitoring:

```rust
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn setup_error_handling() {
    // Set up panic hook
    console_error_panic_hook::set_once();
    
    // Custom error handler
    leptos::set_error_handler(|error| {
        error(&format!("Leptos error: {:?}", error));
        // Send to error tracking service
        send_to_error_tracker(error);
    });
}

fn send_to_error_tracker(error: &dyn std::fmt::Debug) {
    // Send to Sentry, Rollbar, etc.
    log(&format!("Error sent to tracker: {:?}", error));
}
```

### Performance Monitoring

Monitor application performance:

```rust
use leptos::*;
use web_sys::{window, Performance};

#[component]
pub fn PerformanceMonitor() -> impl IntoView {
    let performance = window().unwrap().performance().unwrap();
    let start_time = performance.now();
    
    // Track page load time
    let load_time = create_rw_signal(0.0);
    
    create_effect(move |_| {
        let current_time = performance.now();
        load_time.set(current_time - start_time);
        
        // Send metrics to analytics
        send_performance_metric("page_load_time", load_time.get());
    });
    
    view! {
        <div class="performance-monitor">
            <p>"Page load time: " {move || format!("{:.2}ms", load_time.get())}</p>
        </div>
    }
}

fn send_performance_metric(name: &str, value: f64) {
    // Send to Google Analytics, etc.
    log::info!("Performance metric: {} = {}", name, value);
}
```

### Analytics Integration

Integrate with analytics services:

```rust
use leptos::*;
use leptos_router::*;

#[component]
pub fn AnalyticsTracker() -> impl IntoView {
    let location = use_location();
    
    create_effect(move |_| {
        let pathname = location.pathname.get();
        
        // Track page views
        track_page_view(&pathname);
        
        // Track user interactions
        track_event("page_view", &pathname);
    });
    
    // This component doesn't render anything
    view! { <></> }
}

fn track_page_view(path: &str) {
    // Send to Google Analytics, Mixpanel, etc.
    log::info!("Page view: {}", path);
}

fn track_event(event_type: &str, data: &str) {
    // Send custom events
    log::info!("Event: {} - {}", event_type, data);
}
```

## Environment Configuration

### Environment Variables

Handle different environments:

```rust
use leptos::*;
use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub api_url: String,
    pub environment: String,
    pub debug_mode: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_url: env::var("API_URL").unwrap_or_else(|_| "http://localhost:3000".to_string()),
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            debug_mode: env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true",
        }
    }
}

pub fn provide_config() {
    let config = AppConfig::default();
    provide_context(config);
}

pub fn use_config() -> AppConfig {
    use_context::<AppConfig>().unwrap_or_default()
}
```

### Feature Flags

Implement feature flags:

```rust
use leptos::*;

#[derive(Clone, Debug)]
pub struct FeatureFlags {
    pub new_dashboard: bool,
    pub beta_features: bool,
    pub analytics: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            new_dashboard: false,
            beta_features: false,
            analytics: true,
        }
    }
}

#[component]
pub fn FeatureGate(
    #[prop(into)] feature: String,
    children: Children,
) -> impl IntoView {
    let flags = use_context::<FeatureFlags>().unwrap_or_default();
    
    match feature.as_str() {
        "new_dashboard" if flags.new_dashboard => children(),
        "beta_features" if flags.beta_features => children(),
        _ => view! { <></> },
    }
}
```

## Security Considerations

### Content Security Policy

Implement CSP headers:

```rust
// For SSR applications
use axum::{
    http::HeaderMap,
    response::IntoResponse,
};

async fn csp_middleware(request: Request, next: Next) -> impl IntoResponse {
    let mut response = next.run(request).await;
    
    let csp = "default-src 'self'; \
               script-src 'self' 'unsafe-inline' 'unsafe-eval'; \
               style-src 'self' 'unsafe-inline'; \
               img-src 'self' data: https:; \
               font-src 'self'; \
               connect-src 'self' https://api.example.com";
    
    response.headers_mut().insert(
        "Content-Security-Policy",
        csp.parse().unwrap(),
    );
    
    response
}
```

### HTTPS Enforcement

Ensure HTTPS in production:

```rust
use axum::http::Uri;

async fn https_redirect(uri: Uri) -> impl IntoResponse {
    let https_uri = format!("https://{}{}", uri.host().unwrap(), uri.path());
    Redirect::temporary(&https_uri)
}

#[cfg(feature = "ssr")]
fn create_app() -> Router {
    let app = Router::new()
        .route("/", get(root))
        .route("/api/*path", get(api_handler));
    
    // Redirect HTTP to HTTPS in production
    if std::env::var("ENVIRONMENT").unwrap_or_default() == "production" {
        app.layer(from_fn(https_redirect))
    } else {
        app
    }
}
```

## Scaling Strategies

### Horizontal Scaling

Scale across multiple instances:

```rust
use axum::extract::State;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<RwLock<DatabaseConnection>>,
    pub cache: Arc<RwLock<Cache>>,
}

#[cfg(feature = "ssr")]
fn create_app() -> Router<AppState> {
    let state = AppState {
        db_pool: Arc::new(RwLock::new(create_db_pool())),
        cache: Arc::new(RwLock::new(Cache::new())),
    };
    
    Router::new()
        .route("/", get(root))
        .route("/api/data", get(get_data))
        .with_state(state)
        .layer(CorsLayer::permissive())
}

async fn get_data(
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Use shared state across instances
    let cache = state.cache.read().await;
    // ... handle request
}
```

### Database Optimization

Optimize database connections:

```rust
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub async fn create_db_pool() -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap()
}
```

### CDN Integration

Serve static assets via CDN:

```rust
use axum::{
    http::StatusCode,
    response::Redirect,
};

async fn cdn_redirect(path: &str) -> impl IntoResponse {
    let cdn_url = format!("https://cdn.example.com{}", path);
    Redirect::temporary(&cdn_url)
}

// Redirect static assets to CDN
async fn static_handler(uri: Uri) -> impl IntoResponse {
    if uri.path().starts_with("/static/") {
        cdn_redirect(uri.path()).await
    } else {
        // Serve from local
        serve_file(uri.path()).await
    }
}
```

## Deployment Checklist

### Pre-deployment Checklist

- [ ] Environment variables configured
- [ ] Database migrations run
- [ ] SSL certificates installed
- [ ] CDN configured
- [ ] Monitoring set up
- [ ] Error tracking configured
- [ ] Performance benchmarks established
- [ ] Security headers configured
- [ ] Backup strategy in place

### Production Readiness Checklist

- [ ] All tests passing
- [ ] Bundle size optimized
- [ ] Lighthouse scores > 90
- [ ] Accessibility compliance
- [ ] Cross-browser testing completed
- [ ] Mobile responsiveness verified
- [ ] SEO optimization complete
- [ ] Analytics integrated
- [ ] Documentation updated

### Post-deployment Checklist

- [ ] Application accessible
- [ ] Database connections working
- [ ] External APIs responding
- [ ] Error monitoring active
- [ ] Performance metrics collecting
- [ ] User feedback mechanisms in place
- [ ] Rollback plan documented

## Troubleshooting

### Common Deployment Issues

**WASM Bundle Too Large**
```bash
# Analyze bundle size
wasm-objdump -h dist/*.wasm

# Use wasm-opt for optimization
wasm-opt -Oz input.wasm -o output.wasm

# Split chunks
trunk build --release --split-chunks
```

**SSR Performance Issues**
```rust
// Add caching
use leptos::*;
use std::collections::HashMap;

static CACHE: once_cell::sync::Lazy<RwLock<HashMap<String, String>>> = 
    once_cell::sync::Lazy::new(|| RwLock::new(HashMap::new()));

pub async fn cached_ssr_render(path: &str) -> String {
    let cache = CACHE.read().await;
    if let Some(cached) = cache.get(path) {
        return cached.clone();
    }
    drop(cache);
    
    let rendered = leptos::ssr::render_to_string(|| view! { <App/> });
    
    let mut cache = CACHE.write().await;
    cache.insert(path.to_string(), rendered.clone());
    
    rendered
}
```

**Hydration Mismatches**
```rust
// Ensure server and client render the same content
#[component]
pub fn SafeComponent() -> impl IntoView {
    // Avoid using Date.now() or Math.random() directly
    // Use signals for dynamic content
    let timestamp = create_signal(chrono::Utc::now().timestamp());
    
    view! {
        <div>
            "Rendered at: " {timestamp.get()}
        </div>
    }
}
```

---

**Congratulations!** You have completed the Leptos Book. This comprehensive guide covers everything from reactive fundamentals to production deployment. Remember to regularly update your knowledge as Leptos continues to evolve, and always test thoroughly before deploying to production.

For additional resources, check out:
- [Official Leptos Documentation](https://leptos.dev/)
- [Leptos GitHub Repository](https://github.com/leptos-rs/leptos)
- [Leptos Discord Community](https://discord.gg/leptos)