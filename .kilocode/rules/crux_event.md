# Crux Event Serialization and Core vs Shell Events

## Why We Use `Set(HttpResult<crux_http::Response<Count>, HttpError>)`

### The Two-Event Pattern Explained

In the counter-next example, we have two events that both consume `Count` data:

```rust
#[derive(Facet, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[repr(C)]
pub enum Event {
    // Events from the shell (serializable)
    Get,
    Increment,
    Decrement,
    
    // Events local to the core (not serializable)
    #[serde(skip)]
    #[facet(skip)]
    Set(HttpResult<crux_http::Response<Count>, HttpError>),
    #[serde(skip)]
    #[facet(skip)]
    Update(Count),
}
```

### Event Flow Architecture

The reason for this pattern becomes clear when you understand the event flow:

```
1. Shell Event: Increment
   ↓
2. Core processes → HTTP request
   ↓
3. Core Event: Set(HttpResult<Response<Count>, HttpError>) ← HTTP response
   ↓
4. Core processes → extracts Count from response
   ↓
5. Core Event: Update(Count) ← clean data
   ↓
6. Core updates model → renders
```

### Why Set() Takes HttpResult<Response<Count>, HttpError>

The `Set` event receives the **raw HTTP response** because:

1. **Error Handling**: The HTTP call can fail, so we need `HttpResult<T, E>`
2. **Response Metadata**: The response contains headers, status codes, not just data
3. **Processing Logic**: The core needs to extract and validate the actual `Count` from the response

```rust
Event::Set(HttpResult::Ok(mut response)) => {
    let count = response.take_body().unwrap(); // Extract Count from response
    Command::event(Event::Update(count))      // Send clean Count to Update
}
Event::Set(HttpResult::Err(e)) => {
    panic!("Oh no something went wrong: {e:?}"); // Handle HTTP error
}
```

### Why Update() Takes Clean Count

The `Update` event receives **clean, validated data**:

```rust
Event::Update(count) => {
    model.count = count;  // Direct assignment of clean data
    render()             // Update UI
}
```

## Why `#[serde(skip)]` and `#[facet(skip)]` Are Critical

### Serialization Boundary Problem

The core issue is **serialization across the FFI boundary**:

```
Shell (TypeScript/Swift/Kotlin) ↔ Core (Rust)
         JSON/Bincode serialization
```

### What Can't Be Serialized

Complex Rust types that can't cross the FFI boundary:

1. **`crux_http::Response<T>`** - Contains internal HTTP implementation details
2. **`HttpError`** - Rust-specific error types with complex internals
3. **Raw HTTP data** - Headers, status codes, connection info

### The Skip Attributes Explained

```rust
#[serde(skip)]    // Skip JSON serialization - don't send to Shell
#[facet(skip)]    // Skip type generation - don't create TypeScript/Swift types
Set(HttpResult<crux_http::Response<Count>, HttpError>),
```

### What Happens Without Skip Attributes

Without these attributes, the type generator would try to create:

```typescript
// This would FAIL - can't serialize Rust HTTP types
interface HttpResponse {
  // Rust internal HTTP implementation details
  // Headers, connection pools, etc.
}

type Event = 
  | { type: "Set", value: HttpResult<HttpResponse<Count>, HttpError> } // BROKEN
```

### Event Categories in Crux

```rust
pub enum Event {
    // SHELL EVENTS (serializable to/from UI)
    Get,           // Simple enum variant
    Increment,     // Simple enum variant
    Decrement,     // Simple data only
    
    // CORE EVENTS (internal processing only)
    #[serde(skip)]
    #[facet(skip)]
    Set(HttpResult<crux_http::Response<Count>, HttpError>), // Complex Rust types
    #[serde(skip)]
    #[facet(skip)]
    Update(Count), // Could be serializable, but kept internal for architecture
}
```

## The Processing Pipeline

### Step-by-Step Event Flow

1. **Shell Event** (serializable):
   ```rust
   Event::Increment // Comes from UI (TypeScript/Swift)
   ```

2. **HTTP Request** (in update function):
   ```rust
   Http::post(url)
       .expect_json()
       .build()
       .map(Into::into)
       .then_send(Event::Set) // Will trigger Set event with response
   ```

3. **Core Event** (non-serializable):
   ```rust
   Event::Set(HttpResult::Ok(response)) // Raw HTTP response data
   ```

4. **Data Extraction** (in update function):
   ```rust
   Event::Set(HttpResult::Ok(mut response)) => {
       let count = response.take_body().unwrap(); // Extract clean data
       Command::event(Event::Update(count))       // Send to Update
   }
   ```

5. **Model Update** (in update function):
   ```rust
   Event::Update(count) => {
       model.count = count; // Update model with clean data
       render()            // Trigger UI update
   }
   ```

## Why This Architecture Matters

### Separation of Concerns

1. **Shell Events**: Simple, serializable, UI-focused
2. **Core Events**: Complex, processing-focused, internal

### Error Handling Isolation

```rust
Event::Set(HttpResult::Err(e)) => {
    // Handle HTTP errors in core
    // Don't leak Rust error types to Shell
    panic!("Oh no something went wrong: {e:?}");
}
```

### Type Safety Across Languages

- Shell only sees simple, serializable events
- Core handles complex Rust types internally
- Clean separation prevents serialization errors

## Implementation Rules

### Always Use This Pattern for HTTP

```rust
// Shell event triggers HTTP
Event::SomeAction => {
    Http::get(url)
        .expect_json::<YourType>()
        .build()
        .map(Into::into)
        .then_send(Event::SetYourType) // Non-serializable event
}

// Core event processes HTTP response
#[serde(skip)]
#[facet(skip)]
Event::SetYourType(HttpResult<crux_http::Response<YourType>, HttpError>) => {
    match result {
        HttpResult::Ok(mut response) => {
            let data = response.take_body().unwrap();
            Command::event(Event::UpdateYourType(data))
        }
        HttpResult::Err(e) => {
            // Handle error
        }
    }
}

// Core event updates model
#[serde(skip)]
#[facet(skip)]
Event::UpdateYourType(data) => {
    model.your_data = data;
    render()
}
```

### Critical Rules

1. **HTTP response events** MUST use `#[serde(skip)]` and `#[facet(skip)]`
2. **Complex Rust types** cannot be serialized to Shell
3. **Extract clean data** before updating model
4. **Keep Shell events simple** and serializable
5. **Process complex responses** in core-only events

This pattern ensures clean separation between the serializable Shell interface and the complex internal Core processing.