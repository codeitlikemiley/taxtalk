# CURRENT Crux API Rules

## CRITICAL IMPORT PATTERNS (EXACT)

### HTTP Capability Import (CURRENT)
```rust
use crux_http::{HttpError, command::Http, protocol::HttpRequest};
```

### Core Crux Imports (CURRENT)  
```rust
use crux_core::{render, App, Command};
use crux_core::macros::effect;
```

### Required App Structure (CURRENT)
```rust
#[derive(Default)]
pub struct Counter;

impl App for Counter {
    type Event = Event;
    type Model = Model; 
    type ViewModel = ViewModel;
    type Capabilities = (); // DEPRECATED - use unit type
    type Effect = Effect;   // REQUIRED

    fn update(
        &self,
        event: Self::Event,
        model: &mut Self::Model,
        _caps: &Self::Capabilities, // REQUIRED signature even though deprecated
    ) -> Command<Self::Effect, Self::Event> {
        // Implementation here
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        // View transformation
    }
}
```

## COMMAND API PATTERNS (CURRENT)

### HTTP Commands (EXACT PATTERN)
```rust
// GET request
Http::get("https://crux-counter.fly.dev/")
    .expect_json::<Counter>()
    .build()
    .then_send(Event::Set)

// POST request  
Http::post("https://crux-counter.fly.dev/")
    .body_json(Counter {
        value: model.count,
        updated_at: 0,
    })
    .expect_json::<Counter>()
    .build()
    .then_send(Event::Set)

// Multiple commands
Command::batch([
    render::render(),
    Http::post("https://crux-counter.fly.dev/")
        .body_json(counter_data)
        .expect_json::<Counter>()
        .build()
        .then_send(Event::Set),
])
```

### Render Commands (CURRENT)
```rust
// Simple render
render::render()

// Render after state change
Event::Set(Ok(counter)) => {
    model.count = counter.value;
    model.confirmed = true;
    render::render()
}
```

### No Effect Commands (CURRENT)
```rust
// When no side effects needed
Command::done()

// Or return from match arm that doesn't need effects
_ => Command::done()
```

## EFFECT ENUM DEFINITION (CURRENT)

### Required Effect Structure
```rust
#[effect(typegen)]
pub enum Effect {
    Render(crux_core::render::RenderOperation),
    Http(crux_http::protocol::HttpRequest),
    // Add other effects as needed
}
```

### NO CAPABILITIES STRUCT
```rust
// DON'T DO THIS (deprecated):
#[derive(Effect)]
pub struct Capabilities<Ev> {
    pub render: Render<Ev>,
    pub http: Http<Ev>,
}

// DO THIS INSTEAD:
impl App for MyApp {
    type Capabilities = (); // Unit type
}
```

## EVENT HANDLING PATTERNS (CURRENT)

### Event Definition
```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Increment,
    Decrement,
    Reset,
    Get,
    Set(Result<Counter, HttpError>),
    SseMessage(Result<String, String>),
}
```

### Building our own SSE Capabilities

```rust
use std::{convert::From, future};

use async_sse::{Event as SseEvent, decode};
use crux_core::{Request, capability::Operation, command::StreamBuilder};
use facet::Facet;
use futures::{Stream, StreamExt, io::Cursor};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Facet, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[facet(namespace = "server_sent_events")]
pub struct SseRequest {
    pub url: String,
}

#[derive(Facet, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[facet(namespace = "server_sent_events")]
#[repr(C)]
pub enum SseResponse {
    Chunk(Vec<u8>),
    Done,
}

impl SseResponse {
    #[must_use]
    pub fn is_done(&self) -> bool {
        matches!(self, SseResponse::Done)
    }
}

impl Operation for SseRequest {
    type Output = SseResponse;
}

pub fn get<Effect, Event, T>(
    url: impl Into<String>,
) -> StreamBuilder<Effect, Event, impl Stream<Item = T>>
where
    Effect: From<Request<SseRequest>> + Send + 'static,
    Event: Send + 'static,
    T: Send + DeserializeOwned,
{
    let url = url.into();

    StreamBuilder::new(|ctx| {
        ctx.stream_from_shell(SseRequest { url })
            .take_while(|response| future::ready(!response.is_done()))
            .flat_map(|response| {
                let SseResponse::Chunk(data) = response else {
                    unreachable!()
                };

                decode(Cursor::new(data))
            })
            .filter_map(|sse_event| async {
                sse_event.ok().and_then(|event| match event {
                    SseEvent::Message(msg) => serde_json::from_slice(msg.data()).ok(),
                    SseEvent::Retry(_) => None, // do we need to worry about this?
                })
            })
    })
}
```

### Event Processing in Update
```rust
fn update(
    &self,
    event: Self::Event,
    model: &mut Self::Model,
    _caps: &Self::Capabilities,
) -> Command<Self::Effect, Self::Event> {
    match event {
        Event::Increment => {
            model.count += 1;
            model.confirmed = false;
            Command::batch([
                render::render(),
                Http::post("https://crux-counter.fly.dev/")
                    .body_json(Counter {
                        value: model.count,
                        updated_at: 0,
                    })
                    .expect_json::<Counter>()
                    .build()
                    .then_send(Event::Set),
            ])
        }
        Event::Set(Ok(counter)) => {
            model.count = counter.value;
            model.confirmed = true;
            render::render()
        }
        Event::Set(Err(_)) => {
            model.confirmed = false;
            render::render()
        }
        _ => Command::done(),
    }
}
```

## MODEL AND VIEWMODEL PATTERNS (CURRENT)

### Model Structure
```rust
#[derive(Default, Clone, Debug)]
pub struct Model {
    pub count: isize,
    pub confirmed: bool,
}
```

### ViewModel Structure
```rust
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct ViewModel {
    pub count: String,
    pub confirmed: bool,
}
```

### View Implementation
```rust
fn view(&self, model: &Self::Model) -> Self::ViewModel {
    ViewModel {
        count: if model.confirmed {
            model.count.to_string()
        } else {
            format!("{}*", model.count)
        },
        confirmed: model.confirmed,
    }
}
```

## CUSTOM CAPABILITIES CREATION (CURRENT)

### Custom Capability Structure
```rust
use crux_core::capability::{CapabilityContext, Operation};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SseRequest {
    pub url: String,
}

impl Operation for SseRequest {
    type Output = Result<String, String>;
}

#[derive(crux_core::macros::Capability)]
pub struct ServerSentEvents<Ev> {
    context: CapabilityContext<SseRequest, Ev>,
}

impl<Ev> ServerSentEvents<Ev>
where
    Ev: 'static + Send,
{
    pub fn new(context: CapabilityContext<SseRequest, Ev>) -> Self {
        Self { context }
    }

    pub fn connect(&self, url: impl Into<String>) -> Command<SseRequest, Ev> {
        Command::request_from_shell(SseRequest { url: url.into() })
    }
}
```

## MIDDLEWARE PATTERNS (CURRENT)

### Middleware Implementation
```rust
use crux_core::{Request, middleware::{Middleware, RequestContext}};

pub struct LoggingMiddleware;

impl<Ev> Middleware<Ev> for LoggingMiddleware {
    fn process_request(
        &self,
        request: &Request<Effect>,
        _context: &RequestContext<Effect, Ev>,
    ) -> Option<Request<Effect>> {
        println!("Processing request: {:?}", request.effect);
        Some(request.clone())
    }
}
```

## CARGO.TOML DEPENDENCIES (CURRENT)

### Core Dependencies
```toml
[dependencies]
crux_core = "0.12.0"
crux_http = "0.14.0" 
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[features]
typegen = ["crux_core/typegen", "crux_http/typegen"]

[build-dependencies]
crux_core = { version = "0.12.0", features = ["typegen"] }
```

## FFI INTEGRATION (CURRENT - NO .udl)

### Bridge Setup
```rust
use crux_core::{bridge::Bridge, Core};
use lazy_static::lazy_static;

lazy_static! {
    static ref CORE: Bridge<Effect, Counter> = Bridge::new(Core::new());
}

// Required exports for Shell
pub fn process_event(data: &[u8]) -> Vec<u8> {
    CORE.process_event(data)
}

pub fn handle_response(id: u32, data: &[u8]) -> Vec<u8> {
    CORE.handle_response(id, data)
}

pub fn view() -> Vec<u8> {
    CORE.view()
}
```

### NO uniffi.udl Files
- The counter-next example does NOT use .udl files
- Uses direct Rust exports only
- WIT interfaces for capabilities

## TESTING PATTERNS (CURRENT)

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crux_core::testing::AppTester;

    #[test]
    fn increments_count() {
        let app = AppTester::<Counter, _>::default();
        let mut model = Model::default();

        let update = app.update(Event::Increment, &mut model);
        
        assert_eq!(model.count, 1);
        assert!(!model.confirmed);
        
        // Check effects
        let effects = update.effects();
        assert!(effects.len() >= 2); // render + http
    }
}
```

## COMMAND COMPOSITION RULES (CURRENT)

### Sequential Commands
```rust
Http::get("https://api.example.com/auth")
    .expect_json::<AuthToken>()
    .build()
    .then_request(|token| {
        Http::get("https://api.example.com/data")
            .header("Authorization", &format!("Bearer {}", token.access_token))
            .expect_json::<Data>()
            .build()
    })
    .then_send(Event::DataReceived)
```

### Concurrent Commands
```rust
Command::batch([
    Http::get("https://api.example.com/users")
        .expect_json::<Vec<User>>()
        .build()
        .then_send(Event::UsersLoaded),
    Http::get("https://api.example.com/settings")
        .expect_json::<Settings>()
        .build()
        .then_send(Event::SettingsLoaded),
    render::render(),
])
```

### Conditional Commands
```rust
if model.needs_refresh {
    Http::get("https://api.example.com/refresh")
        .expect_json::<RefreshData>()
        .build()
        .then_send(Event::Refreshed)
} else {
    Command::done()
}
```

## CRITICAL RULES TO FOLLOW

1. **ALWAYS** use `_caps: &Self::Capabilities` in update signature
2. **NEVER** use Capabilities struct - use `type Capabilities = ()`
3. **ALWAYS** use `Command::batch([...])` for multiple effects
4. **ALWAYS** use `.build().then_send(Event)` pattern
5. **NEVER** use deprecated `.send()` API
6. **ALWAYS** include `type Effect = Effect` in App impl
7. **ALWAYS** use exact import: `use crux_http::{HttpError, command::Http, protocol::HttpRequest}`
8. **NO .udl files** - use direct Rust exports only
9. **ALWAYS** add this on update: `#[allow(clippy::too_many_lines)]`
10. **ALWAYS** add on Effect enum: `#[effect(facet_typegen)]`