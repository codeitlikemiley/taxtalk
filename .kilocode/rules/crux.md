# Crux Plugin Bootstrap Rules - Counter-Next Patterns

## Required Imports (EXACT)

### Core Imports
```rust
use chrono::{DateTime, Utc, serde::ts_milliseconds_option::deserialize as ts_milliseconds_option};
use crux_core::{
    Command,
    macros::effect,
    render::{RenderOperation, render},
};
use crux_http::{HttpError, command::Http, protocol::HttpRequest};
use facet::Facet;
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use url::Url;
```

## API URL Definition (REQUIRED)

### Always Define API Base URL
```rust
const API_URL: &str = "https://your-api-endpoint.com";
```

## HttpResult Type (REQUIRED for HTTP)

### HttpResult Wrapper (ALWAYS INCLUDE)
```rust
#[derive(Facet, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum HttpResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T> From<crux_http::Result<crux_http::Response<T>>>
    for HttpResult<crux_http::Response<T>, HttpError>
{
    fn from(value: crux_http::Result<crux_http::Response<T>>) -> Self {
        match value {
            Ok(response) => HttpResult::Ok(response),
            Err(error) => HttpResult::Err(error),
        }
    }
}
```

## Model Definition Rules

### Model Structure (EXACT PATTERN)
```rust
#[derive(Default, Serialize)]
pub struct Model {
    // Your model fields here
    count: Count,
    // Add other fields as needed
}
```

### Data Type Minimum Derives
```rust
#[derive(Facet, Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Count {
    value: isize,
    #[serde(deserialize_with = "ts_milliseconds_option")]
    updated_at: Option<DateTime<Utc>>,
}
```

## ViewModel Definition Rules

### ViewModel Pattern (EXACT)
```rust
#[derive(Facet, Serialize, Deserialize, Debug, Clone)]
#[facet(namespace = "view_model")]
pub struct ViewModel {
    pub text: String,
    pub confirmed: bool,
    // Add other UI fields as needed
}
```

## Event Definition Rules

### Event Enum Pattern (EXACT)
```rust
#[derive(Facet, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[repr(C)]
pub enum Event {
    // Events from the shell (serializable)
    Get,
    Increment,
    Decrement,
    Reset,
    
    // Events local to the core (not serializable)
    #[serde(skip)]
    #[facet(skip)]
    Set(HttpResult<crux_http::Response<Count>, HttpError>),
    #[serde(skip)]
    #[facet(skip)]
    Update(Count),
    #[serde(skip)]
    #[facet(skip)]
    UpdateBy(isize),
}
```

### Event Categorization Rules
- **Shell Events**: Simple, serializable events from UI (Get, Increment, etc.)
- **Core Events**: Complex events with `#[serde(skip)]` and `#[facet(skip)]`
- Use `HttpResult<T, E>` wrapper for HTTP response events

## Effect Definition Rules

### Effect Enum Pattern (EXACT)
```rust
#[effect(facet_typegen)]
#[derive(Debug)]
pub enum Effect {
    Render(RenderOperation),
    Http(HttpRequest),
    ServerSentEvents(SseRequest),
    Random(RandomNumberRequest),
    // Add other capabilities as needed
}
```

## Plugin/App Definition Rules

### Plugin Structure (EXACT)
```rust
// Can be named anything: App, InventoryPlugin, CrmPlugin, etc.
#[derive(Default)]
pub struct App;
```

### App Implementation (EXACT PATTERN)
```rust
impl crux_core::App for App {
    type Model = Model;
    type Event = Event;
    type ViewModel = ViewModel;
    type Capabilities = (); // ALWAYS unit type
    type Effect = Effect;

    #[allow(clippy::too_many_lines)]
    fn update(
        &self,
        msg: Self::Event,
        model: &mut Self::Model,
        _caps: &Self::Capabilities, // REQUIRED signature
    ) -> Command<Effect, Event> {
        match msg {
            // Handle events here
            Event::Get => Http::get(API_URL)
                .expect_json()
                .build()
                .map(Into::into)
                .then_send(Event::Set),
            // More event handlers...
            _ => Command::done(),
        }
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            text: model.count.value.to_string(),
            confirmed: model.count.updated_at.is_some(),
        }
    }
}
```

## Command Patterns (FROM EXAMPLE)

### HTTP GET Pattern
```rust
Event::Get => Http::get(API_URL)
    .expect_json()
    .build()
    .map(Into::into)
    .then_send(Event::Set),
```

### HTTP POST Pattern
```rust
Event::Increment => {
    // Optimistic update
    model.count = Count {
        value: model.count.value + 1,
        updated_at: None,
    };

    let call_api = {
        let base = Url::parse(API_URL).unwrap();
        let url = base.join("/inc").unwrap();
        Http::post(url)
            .expect_json()
            .build()
            .map(Into::into)
            .then_send(Event::Set)
    };

    render().and(call_api)
}
```

### Async Command Pattern
```rust
Event::UpdateBy(change) => {
    // Complex async operation
    Command::new(|ctx| async move {
        let futures = (0..n).map(|_| {
            Http::post(url.clone())
                .expect_json::<Count>()
                .build()
                .into_future(ctx.clone())
        });

        let result: Result<Vec<crux_http::Response<Count>>, crux_http::HttpError> =
            join_all(futures).await.into_iter().collect();

        let latest = result.map(|counts| {
            counts
                .into_iter()
                .max_by_key(|c| c.body().unwrap().updated_at.unwrap())
                .unwrap()
        });

        ctx.send_event(Event::Set(latest.into()));
    })
}
```

### Command Composition Patterns
```rust
// Render and HTTP together
render().and(call_api)

// Event without side effects
Command::event(Event::Update(count))

// No operation
Command::done()
```

## FFI Module (COPY-PASTE REQUIRED)

### FFI Module Structure
```rust
#[cfg(not(target_family = "wasm"))]
pub mod uniffi_ffi {
    use std::sync::Arc;
    
    use crux_core::{
        Core,
        bridge::EffectId,
        macros::effect,
        middleware::{BincodeFfiFormat, Bridge, HandleEffectLayer, Layer as _, MapEffectLayer},
        render::RenderOperation,
    };
    use crux_http::protocol::HttpRequest;

    use crate::{App, middleware::RngMiddleware, sse::SseRequest};

    #[effect(facet_typegen)]
    pub enum Effect {
        Render(RenderOperation),
        Http(HttpRequest),
        ServerSentEvents(SseRequest),
    }

    impl From<crate::app::Effect> for Effect {
        fn from(effect: crate::app::Effect) -> Self {
            match effect {
                crate::Effect::Render(request) => Effect::Render(request),
                crate::Effect::Http(request) => Effect::Http(request),
                crate::Effect::ServerSentEvents(request) => Effect::ServerSentEvents(request),
                crate::Effect::Random(_) => panic!("Encountered a Random effect"),
            }
        }
    }

    #[uniffi::export(with_foreign)]
    pub trait CruxShell: Send + Sync {
        fn process_effects(&self, bytes: Vec<u8>);
    }

    #[derive(uniffi::Object)]
    pub struct CoreFFI {
        core: Bridge<
            MapEffectLayer<HandleEffectLayer<Core<App>, RngMiddleware>, Effect>,
            BincodeFfiFormat,
        >,
    }

    #[uniffi::export]
    #[allow(clippy::missing_panics_doc)]
    impl CoreFFI {
        #[uniffi::constructor]
        pub fn new(shell: Arc<dyn CruxShell>) -> Self {
            let core = Core::<App>::new()
                .handle_effects_using(RngMiddleware::new())
                .map_effect::<Effect>()
                .bridge::<BincodeFfiFormat>(move |effect_bytes| match effect_bytes {
                    Ok(effect) => shell.process_effects(effect),
                    Err(e) => panic!("{e}"),
                });

            Self { core }
        }

        #[must_use]
        pub fn update(&self, data: &[u8]) -> Vec<u8> {
            match self.core.update(data) {
                Ok(effects) => effects,
                Err(e) => panic!("{e}"),
            }
        }

        #[must_use]
        pub fn resolve(&self, effect_id: u32, data: &[u8]) -> Vec<u8> {
            match self.core.resolve(EffectId(effect_id), data) {
                Ok(effects) => effects,
                Err(e) => panic!("{e}"),
            }
        }

        #[must_use]
        pub fn view(&self) -> Vec<u8> {
            match self.core.view() {
                Ok(view) => view,
                Err(e) => panic!("{e}"),
            }
        }
    }
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub mod wasm_ffi {
    // WASM FFI implementation - copy from example
}

#[cfg(all(target_os = "wasi", target_env = "p2"))]
pub mod wasip2 {
    // WASI FFI implementation - copy from example
}
```

## Library Root (lib.rs) Pattern

### Required lib.rs Structure
```rust
mod app;
mod capabilities;
mod ffi;
#[cfg(not(target_family = "wasm"))]
mod middleware;

pub use crux_core::Core;
pub use crux_http as http;
pub use app::*;
pub use capabilities::{RandomNumber, RandomNumberRequest, sse};

#[cfg(not(target_family = "wasm"))]
const _: () = assert!(
    uniffi::check_compatible_version("0.29.4"),
    "please use uniffi v0.29.4"
);

#[cfg(not(target_family = "wasm"))]
uniffi::setup_scaffolding!();

#[cfg(not(target_family = "wasm"))]
pub use ffi::uniffi_ffi::CoreFFI;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub use ffi::wasm_ffi::CoreFFI;

#[cfg(all(target_os = "wasi", target_env = "p2"))]
pub use ffi::wasip2::CoreFFI;
```

## Testing Patterns (FROM EXAMPLE)

### Test Structure Pattern
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crux_core::{App as _, assert_effect};
    use crux_http::{
        protocol::{HttpRequest, HttpResponse, HttpResult},
        testing::ResponseBuilder,
    };

    #[test]
    fn get_counter() {
        let app = App;
        let mut model = Model::default();

        // Send event to app
        let mut cmd = app.update(Event::Get, &mut model, &());

        // Check effects
        let (operation, mut request) = cmd.effects().next().unwrap().expect_http().split();
        
        // Assert request
        assert_eq!(
            &operation,
            &HttpRequest::get("https://crux-counter.fly.dev/").build()
        );

        // Resolve request
        let response = HttpResponse::ok()
            .body(r#"{ "value": 1, "updated_at": 1672531200000 }"#)
            .build();
        request
            .resolve(crux_http::protocol::HttpResult::Ok(response))
            .unwrap();

        // Check events
        let actual = cmd.events().next().unwrap();
        // Assert event matches expected
    }
}
```

## Cargo.toml Dependencies

### Required Dependencies
```toml
[dependencies]
crux_core = "0.12.0"
crux_http = "0.14.0"
facet = "0.4.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
url = "2.5"
futures = "0.3"

[features]
typegen = ["crux_core/typegen", "crux_http/typegen"]

[build-dependencies]
uniffi = { version = "0.29.4", features = ["build"] }
```

## CRITICAL BOOTSTRAP CHECKLIST

1. ✅ Define `API_URL` constant
2. ✅ Include `HttpResult<T, E>` wrapper with From impl
3. ✅ Use exact derive macros on all types
4. ✅ Event categorization with proper skip attributes
5. ✅ `#[effect(facet_typegen)]` on Effect enum
6. ✅ `type Capabilities = ()` in App impl
7. ✅ Required `_caps: &Self::Capabilities` parameter
8. ✅ Copy FFI module exactly from example
9. ✅ Include all required modules in lib.rs
10. ✅ Use exact command patterns from example