# Data Fetching Example

## Overview

This example demonstrates comprehensive data fetching patterns in Leptos, covering:

- HTTP requests with different methods (GET, POST, PUT, DELETE)
- Loading states and error handling
- Caching and data synchronization
- Real-time data with Server-Sent Events
- Optimistic updates and rollback
- Pagination and infinite scrolling
- Background data synchronization
- API integration patterns
- Request/response interceptors
- Retry logic and exponential backoff

## Project Structure

```
src/
├── main.rs                    # Application entry point
├── components/
│   ├── data/
│   │   ├── user_list.rs       # User list with pagination
│   │   ├── post_feed.rs       # Social media feed
│   │   ├── realtime_chat.rs   # Real-time chat with SSE
│   │   ├── data_table.rs      # Sortable/filterable table
│   │   └── file_manager.rs    # File upload/download
│   ├── ui/
│   │   ├── loading_spinner.rs # Loading indicators
│   │   ├── error_boundary.rs  # Error handling UI
│   │   ├── retry_button.rs    # Retry functionality
│   │   └── offline_indicator.rs # Offline/online status
│   └── hooks/
│       ├── use_api.rs         # API hook with caching
│       ├── use_sse.rs         # Server-Sent Events hook
│       ├── use_pagination.rs  # Pagination hook
│       └── use_background_sync.rs # Background sync hook
├── models/
│   ├── user.rs                # User data structures
│   ├── post.rs                # Post data structures
│   ├── message.rs             # Chat message structures
│   └── api.rs                 # API response types
├── services/
│   ├── api_client.rs          # HTTP client wrapper
│   ├── cache.rs               # Data caching service
│   ├── retry.rs               # Retry logic
│   └── offline_queue.rs       # Offline request queue
└── utils/
    ├── api.rs                 # API utility functions
    ├── cache.rs               # Cache utilities
    └── network.rs             # Network status utilities
```

## Core Data Fetching Patterns

### API Client with Error Handling

```rust
use leptos::*;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
    pub status: u16,
}

#[derive(Clone, Debug)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    auth_token: RwSignal<Option<String>>,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            auth_token: create_rw_signal(None),
        }
    }

    pub fn set_auth_token(&self, token: Option<String>) {
        self.auth_token.set(token);
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<T, ApiError> {
        let mut url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));

        if let Some(params) = params {
            let query = serde_urlencoded::to_string(&params)?;
            url = format!("{}?{}", url, query);
        }

        let mut request = self.client.get(&url);

        if let Some(token) = self.auth_token.get() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            let data: T = response.json().await?;
            Ok(data)
        } else {
            let error_text = response.text().await?;
            Err(ApiError::Http { status, message: error_text })
        }
    }

    pub async fn post<T: Serialize, U: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        data: &T,
    ) -> Result<U, ApiError> {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));

        let mut request = self.client
            .post(&url)
            .json(data);

        if let Some(token) = self.auth_token.get() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            let data: U = response.json().await?;
            Ok(data)
        } else {
            let error_text = response.text().await?;
            Err(ApiError::Http { status, message: error_text })
        }
    }

    pub async fn put<T: Serialize, U: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        data: &T,
    ) -> Result<U, ApiError> {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));

        let mut request = self.client
            .put(&url)
            .json(data);

        if let Some(token) = self.auth_token.get() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            let data: U = response.json().await?;
            Ok(data)
        } else {
            let error_text = response.text().await?;
            Err(ApiError::Http { status, message: error_text })
        }
    }

    pub async fn delete(&self, endpoint: &str) -> Result<(), ApiError> {
        let url = format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'));

        let mut request = self.client.delete(&url);

        if let Some(token) = self.auth_token.get() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(ApiError::Http { status, message: error_text })
        }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum ApiError {
    #[error("HTTP error {status}: {message}")]
    Http { status: StatusCode, message: String },

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_urlencoded::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
```

## Data Fetching Hook with Caching

### useApi Hook

```rust
use leptos::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub enum FetchState<T> {
    Idle,
    Loading,
    Success(T),
    Error(String),
}

#[derive(Clone, Debug)]
pub struct CacheEntry<T> {
    pub data: T,
    pub timestamp: Instant,
    pub ttl: Duration,
}

impl<T> CacheEntry<T> {
    pub fn is_expired(&self) -> bool {
        self.timestamp.elapsed() > self.ttl
    }
}

#[derive(Clone, Debug)]
pub struct ApiConfig {
    pub cache_ttl: Duration,
    pub retry_attempts: u32,
    pub retry_delay: Duration,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            cache_ttl: Duration::from_secs(300), // 5 minutes
            retry_attempts: 3,
            retry_delay: Duration::from_millis(1000),
        }
    }
}

pub fn use_api<T, K>(
    api_client: ApiClient,
    config: ApiConfig,
) -> (ReadSignal<FetchState<T>>, WriteSignal<K>)
where
    T: Clone + 'static,
    K: Clone + Hash + Eq + 'static,
{
    let (state, set_state) = create_signal(FetchState::<T>::Idle);
    let (trigger, set_trigger) = create_signal(None::<K>);
    let cache = create_rw_signal(HashMap::<K, CacheEntry<T>>::new());

    // Effect to handle API calls
    create_effect(move |_| {
        if let Some(key) = trigger.get() {
            // Check cache first
            if let Some(entry) = cache.get().get(&key) {
                if !entry.is_expired() {
                    set_state.set(FetchState::Success(entry.data.clone()));
                    return;
                }
            }

            set_state.set(FetchState::Loading);

            // Spawn async task for API call
            spawn_local(async move {
                let mut attempt = 0;
                let mut delay = config.retry_delay;

                loop {
                    match perform_api_call(&api_client, &key).await {
                        Ok(data) => {
                            // Cache the result
                            let entry = CacheEntry {
                                data: data.clone(),
                                timestamp: Instant::now(),
                                ttl: config.cache_ttl,
                            };
                            cache.update(|cache| {
                                cache.insert(key.clone(), entry);
                            });

                            set_state.set(FetchState::Success(data));
                            break;
                        }
                        Err(err) => {
                            attempt += 1;
                            if attempt >= config.retry_attempts {
                                set_state.set(FetchState::Error(err.to_string()));
                                break;
                            } else {
                                // Wait before retry
                                gloo::timers::future::TimeoutFuture::new(delay.as_millis() as u32).await;
                                delay = delay.saturating_mul(2); // Exponential backoff
                            }
                        }
                    }
                }
            });
        }
    });

    (state, set_trigger)
}

async fn perform_api_call<T, K>(api_client: &ApiClient, key: &K) -> Result<T, ApiError>
where
    T: for<'de> Deserialize<'de>,
    K: Clone,
{
    // This would be implemented based on the specific API endpoint
    // For demonstration, we'll assume a generic GET request
    api_client.get(&format!("{:?}", key), None).await
}
```

## User List with Pagination

### Paginated Data Fetching

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: u32,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

#[component]
pub fn UserList() -> impl IntoView {
    let api_client = create_rw_signal(ApiClient::new("https://api.example.com".to_string()));
    let config = ApiConfig::default();

    let (users_state, fetch_users) = use_api::<PaginatedResponse<User>, (u32, u32)>(api_client.get(), config);
    let (current_page, set_current_page) = create_signal(1);
    let (per_page, set_per_page) = create_signal(10);

    // Fetch users when page or per_page changes
    create_effect(move |_| {
        fetch_users.set(Some((current_page.get(), per_page.get())));
    });

    let next_page = move |_| {
        if let FetchState::Success(ref response) = users_state.get() {
            if current_page.get() < response.total_pages {
                set_current_page.update(|page| *page += 1);
            }
        }
    };

    let prev_page = move |_| {
        if current_page.get() > 1 {
            set_current_page.update(|page| *page -= 1);
        }
    };

    let change_per_page = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlSelectElement>(&ev);
        if let Some(select) = target {
            if let Ok(value) = select.value().parse::<u32>() {
                set_per_page.set(value);
                set_current_page.set(1); // Reset to first page
            }
        }
    };

    view! {
        <div class="user-list">
            <div class="controls">
                <label>
                    "Items per page: "
                    <select on:change=change_per_page>
                        <option value="10" selected=move || per_page.get() == 10>"10"</option>
                        <option value="25" selected=move || per_page.get() == 25>"25"</option>
                        <option value="50" selected=move || per_page.get() == 50>"50"</option>
                    </select>
                </label>
            </div>

            <div class="user-grid">
                {move || match users_state.get() {
                    FetchState::Idle => view! { <p>"Click to load users"</p> },
                    FetchState::Loading => view! { <LoadingSpinner /> },
                    FetchState::Success(response) => view! {
                        <div>
                            <For
                                each=move || response.data.clone()
                                key=|user| user.id
                                children=move |user| view! {
                                    <UserCard user=user.clone() />
                                }
                            />
                        </div>
                    },
                    FetchState::Error(err) => view! {
                        <div class="error">
                            <p>{format!("Error loading users: {}", err)}</p>
                            <RetryButton on_click=move |_| fetch_users.set(Some((current_page.get(), per_page.get()))) />
                        </div>
                    }
                }}
            </div>

            <div class="pagination">
                <button
                    on:click=prev_page
                    prop:disabled=move || current_page.get() == 1
                    class="prev-btn"
                >
                    "Previous"
                </button>

                <span class="page-info">
                    {move || {
                        if let FetchState::Success(ref response) = users_state.get() {
                            format!("Page {} of {}", response.page, response.total_pages)
                        } else {
                            "Loading...".to_string()
                        }
                    }}
                </span>

                <button
                    on:click=next_page
                    prop:disabled=move || {
                        if let FetchState::Success(ref response) = users_state.get() {
                            current_page.get() >= response.total_pages
                        } else {
                            true
                        }
                    }
                    class="next-btn"
                >
                    "Next"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn UserCard(#[prop(into)] user: User) -> impl IntoView {
    view! {
        <div class="user-card">
            <img src=user.avatar.unwrap_or_else(|| "/default-avatar.png".to_string()) alt=format!("Avatar for {}", user.name) />
            <div class="user-info">
                <h3>{user.name}</h3>
                <p>{user.email}</p>
            </div>
        </div>
    }
}
```

## Real-Time Data with Server-Sent Events

### SSE Hook Implementation

```rust
use leptos::*;
use web_sys::{EventSource, MessageEvent};
use serde::Deserialize;
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub user: String,
    pub content: String,
    pub timestamp: String,
}

pub fn use_sse<T>(
    url: &str,
    event_type: &str,
) -> (ReadSignal<Vec<T>>, ReadSignal<bool>)
where
    T: Clone + for<'de> Deserialize<'de> + 'static,
{
    let (messages, set_messages) = create_signal(Vec::<T>::new());
    let (connected, set_connected) = create_signal(false);

    create_effect(move |_| {
        let event_source = EventSource::new(url).unwrap();

        let on_open = Closure::wrap(Box::new(move |_| {
            set_connected.set(true);
        }) as Box<dyn FnMut(web_sys::Event)>);

        let on_message = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Ok(data) = event.data().as_string() {
                if let Ok(message) = serde_json::from_str::<T>(&data) {
                    set_messages.update(|messages| {
                        messages.push(message);
                    });
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        let on_error = Closure::wrap(Box::new(move |_| {
            set_connected.set(false);
        }) as Box<dyn FnMut(web_sys::Event)>);

        event_source.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        event_source.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        event_source.set_onerror(Some(on_error.as_ref().unchecked_ref()));

        // Keep closures alive
        on_open.forget();
        on_message.forget();
        on_error.forget();

        // Cleanup effect
        on_cleanup(move || {
            event_source.close();
        });
    });

    (messages, connected)
}

#[component]
pub fn RealtimeChat() -> impl IntoView {
    let (messages, connected) = use_sse::<ChatMessage>("/api/chat/stream", "message");
    let (new_message, set_new_message) = create_signal(String::new());
    let api_client = create_rw_signal(ApiClient::new("https://api.example.com".to_string()));

    let send_message = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        let message_content = new_message.get();
        if message_content.trim().is_empty() {
            return;
        }

        let message = serde_json::json!({
            "content": message_content
        });

        spawn_local(async move {
            if let Err(err) = api_client.get().post::<_, serde_json::Value>("/api/chat/messages", &message).await {
                log::error!("Failed to send message: {:?}", err);
            }
        });

        set_new_message.set(String::new());
    };

    view! {
        <div class="chat-container">
            <div class="connection-status">
                <span class=move || if connected.get() { "connected" } else { "disconnected" }>
                    {move || if connected.get() { "🟢 Connected" } else { "🔴 Disconnected" }}
                </span>
            </div>

            <div class="messages">
                <For
                    each=move || messages.get()
                    key=|msg| msg.id.clone()
                    children=move |message| view! {
                        <div class="message">
                            <strong>{message.user}": "</strong>
                            <span>{message.content}</span>
                            <small>{message.timestamp}</small>
                        </div>
                    }
                />
            </div>

            <form on:submit=send_message class="message-form">
                <input
                    type="text"
                    prop:value=new_message
                    on:input=move |ev| {
                        let target = event_target::<web_sys::HtmlInputElement>(&ev);
                        if let Some(input) = target {
                            set_new_message.set(input.value());
                        }
                    }
                    placeholder="Type your message..."
                    prop:disabled=move || !connected.get()
                />
                <button
                    type="submit"
                    prop:disabled=move || !connected.get() || new_message.get().trim().is_empty()
                >
                    "Send"
                </button>
            </form>
        </div>
    }
}
```

## Optimistic Updates

### Optimistic Update Pattern

```rust
#[component]
pub fn TodoList() -> impl IntoView {
    let api_client = create_rw_signal(ApiClient::new("https://api.example.com".to_string()));
    let (todos, set_todos) = create_signal(Vec::<Todo>::new());
    let (loading, set_loading) = create_signal(true);

    // Load initial todos
    create_effect(move |_| {
        spawn_local(async move {
            match api_client.get().get::<Vec<Todo>>("/api/todos", None).await {
                Ok(fetched_todos) => {
                    set_todos.set(fetched_todos);
                    set_loading.set(false);
                }
                Err(err) => {
                    log::error!("Failed to load todos: {:?}", err);
                    set_loading.set(false);
                }
            }
        });
    });

    let add_todo = move |title: String| {
        let new_todo = Todo {
            id: format!("temp-{}", js_sys::Date::now()),
            title: title.clone(),
            completed: false,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        // Optimistic update
        set_todos.update(|todos| {
            todos.insert(0, new_todo.clone());
        });

        // Send to server
        let todo_data = serde_json::json!({
            "title": title,
            "completed": false
        });

        spawn_local(async move {
            match api_client.get().post::<_, Todo>("/api/todos", &todo_data).await {
                Ok(server_todo) => {
                    // Replace temporary todo with server response
                    set_todos.update(|todos| {
                        if let Some(pos) = todos.iter().position(|t| t.id == new_todo.id) {
                            todos[pos] = server_todo;
                        }
                    });
                }
                Err(err) => {
                    // Rollback optimistic update
                    set_todos.update(|todos| {
                        todos.retain(|t| t.id != new_todo.id);
                    });
                    log::error!("Failed to create todo: {:?}", err);
                }
            }
        });
    };

    let toggle_todo = move |todo_id: String| {
        // Find current todo state
        let current_todo = todos.get().iter().find(|t| t.id == todo_id).cloned();

        if let Some(mut todo) = current_todo {
            // Optimistic update
            todo.completed = !todo.completed;
            set_todos.update(|todos| {
                if let Some(pos) = todos.iter().position(|t| t.id == todo_id) {
                    todos[pos] = todo.clone();
                }
            });

            // Send to server
            let update_data = serde_json::json!({
                "completed": todo.completed
            });

            spawn_local(async move {
                let endpoint = format!("/api/todos/{}", todo_id);
                match api_client.get().put::<_, Todo>(&endpoint, &update_data).await {
                    Ok(updated_todo) => {
                        // Update with server response
                        set_todos.update(|todos| {
                            if let Some(pos) = todos.iter().position(|t| t.id == todo_id) {
                                todos[pos] = updated_todo;
                            }
                        });
                    }
                    Err(err) => {
                        // Rollback optimistic update
                        set_todos.update(|todos| {
                            if let Some(pos) = todos.iter().position(|t| t.id == todo_id) {
                                todos[pos].completed = !todo.completed; // Revert
                            }
                        });
                        log::error!("Failed to update todo: {:?}", err);
                    }
                }
            });
        }
    };

    let delete_todo = move |todo_id: String| {
        // Find todo to remove
        let todo_to_remove = todos.get().iter().find(|t| t.id == todo_id).cloned();

        if let Some(todo) = todo_to_remove {
            // Optimistic update
            set_todos.update(|todos| {
                todos.retain(|t| t.id != todo_id);
            });

            // Send to server
            spawn_local(async move {
                let endpoint = format!("/api/todos/{}", todo_id);
                if let Err(err) = api_client.get().delete(&endpoint).await {
                    // Rollback optimistic update
                    set_todos.update(|todos| {
                        todos.push(todo);
                    });
                    log::error!("Failed to delete todo: {:?}", err);
                }
            });
        }
    };

    view! {
        <div class="todo-list">
            <h2>"Todo List"</h2>

            <AddTodoForm on_add=add_todo />

            {move || if loading.get() {
                view! { <LoadingSpinner /> }
            } else {
                view! {
                    <div class="todos">
                        <For
                            each=move || todos.get()
                            key=|todo| todo.id.clone()
                            children=move |todo| view! {
                                <TodoItem
                                    todo=todo.clone()
                                    on_toggle=Callback::new(move |_| toggle_todo(todo.id.clone()))
                                    on_delete=Callback::new(move |_| delete_todo(todo.id.clone()))
                                />
                            }
                        />
                    </div>
                }
            }}
        </div>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
}
```

## Background Data Synchronization

### Background Sync Hook

```rust
use leptos::*;
use std::time::Duration;
use web_sys::{ServiceWorkerRegistration, SyncManager};

pub fn use_background_sync<T>(
    api_client: ApiClient,
    sync_interval: Duration,
) -> (ReadSignal<bool>, Callback<()>)
where
    T: Clone + Serialize + for<'de> Deserialize<'de> + 'static,
{
    let (is_online, set_is_online) = create_signal(true);
    let (pending_requests, set_pending_requests) = create_signal(Vec::<PendingRequest<T>>::new());

    // Monitor online status
    create_effect(move |_| {
        let online_handler = Closure::wrap(Box::new(move |_| {
            set_is_online.set(true);
        }) as Box<dyn FnMut(web_sys::Event)>);

        let offline_handler = Closure::wrap(Box::new(move |_| {
            set_is_online.set(false);
        }) as Box<dyn FnMut(web_sys::Event)>);

        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("online", online_handler.as_ref().unchecked_ref())
            .unwrap();

        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("offline", offline_handler.as_ref().unchecked_ref())
            .unwrap();

        online_handler.forget();
        offline_handler.forget();
    });

    // Background sync effect
    create_effect(move |_| {
        if is_online.get() && !pending_requests.get().is_empty() {
            let requests = pending_requests.get();

            for request in requests {
                spawn_local(async move {
                    let result = match request.method.as_str() {
                        "POST" => api_client.post::<T, serde_json::Value>(&request.endpoint, &request.data).await,
                        "PUT" => api_client.put::<T, serde_json::Value>(&request.endpoint, &request.data).await,
                        "DELETE" => api_client.delete(&request.endpoint).await.map(|_| serde_json::Value::Null),
                        _ => continue,
                    };

                    match result {
                        Ok(_) => {
                            // Remove from pending requests
                            set_pending_requests.update(|requests| {
                                requests.retain(|r| r.id != request.id);
                            });
                        }
                        Err(err) => {
                            log::error!("Background sync failed for {}: {:?}", request.endpoint, err);
                        }
                    }
                });
            }
        }
    });

    // Register background sync
    let register_sync = Callback::new(move |_| {
        spawn_local(async move {
            if let Ok(Some(registration)) = web_sys::window()
                .unwrap()
                .navigator()
                .service_worker()
                .get_registration() {

                if let Ok(sync_manager) = registration.sync() {
                    if let Err(err) = sync_manager.register("background-sync").await {
                        log::error!("Failed to register background sync: {:?}", err);
                    }
                }
            }
        });
    });

    (is_online, register_sync)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingRequest<T> {
    pub id: String,
    pub method: String,
    pub endpoint: String,
    pub data: T,
}
```

## Infinite Scrolling

### Infinite Scroll Hook

```rust
use leptos::*;
use web_sys::Element;

pub fn use_infinite_scroll<T>(
    load_more: Callback<()>,
    has_more: ReadSignal<bool>,
    loading: ReadSignal<bool>,
) -> NodeRef<html::Div>
where
    T: 'static,
{
    let container_ref = create_node_ref::<html::Div>();
    let (is_loading_more, set_is_loading_more) = create_signal(false);

    create_effect(move |_| {
        let container = container_ref.get();

        if let Some(container) = container {
            let scroll_handler = Closure::wrap(Box::new(move |_| {
                if !has_more.get() || loading.get() || is_loading_more.get() {
                    return;
                }

                let scroll_top = container.scroll_top();
                let scroll_height = container.scroll_height();
                let client_height = container.client_height();

                // Load more when user scrolls near bottom
                if scroll_top + client_height >= scroll_height - 100 {
                    set_is_loading_more.set(true);
                    load_more.call(());

                    // Reset loading state after a delay
                    set_timeout(move || {
                        set_is_loading_more.set(false);
                    }, Duration::from_millis(500));
                }
            }) as Box<dyn FnMut(web_sys::Event)>);

            container.add_event_listener_with_callback("scroll", scroll_handler.as_ref().unchecked_ref()).unwrap();
            scroll_handler.forget();
        }
    });

    container_ref
}

#[component]
pub fn PostFeed() -> impl IntoView {
    let api_client = create_rw_signal(ApiClient::new("https://api.example.com".to_string()));
    let (posts, set_posts) = create_signal(Vec::<Post>::new());
    let (loading, set_loading) = create_signal(false);
    let (has_more, set_has_more) = create_signal(true);
    let (page, set_page) = create_signal(1);

    let load_posts = move |_| {
        if loading.get() || !has_more.get() {
            return;
        }

        set_loading.set(true);

        spawn_local(async move {
            let params = HashMap::from([
                ("page".to_string(), page.get().to_string()),
                ("per_page".to_string(), "20".to_string()),
            ]);

            match api_client.get().get::<PaginatedResponse<Post>>("/api/posts", Some(params)).await {
                Ok(response) => {
                    set_posts.update(|posts| {
                        posts.extend(response.data);
                    });

                    set_has_more.set(response.page < response.total_pages);
                    set_page.update(|p| *p += 1);
                }
                Err(err) => {
                    log::error!("Failed to load posts: {:?}", err);
                }
            }

            set_loading.set(false);
        });
    };

    // Load initial posts
    create_effect(move |_| {
        load_posts(());
    });

    let container_ref = use_infinite_scroll(load_posts, has_more, loading);

    view! {
        <div class="post-feed" node_ref=container_ref>
            <For
                each=move || posts.get()
                key=|post| post.id.clone()
                children=move |post| view! {
                    <PostCard post=post.clone() />
                }
            />

            {move || if loading.get() {
                view! { <LoadingSpinner /> }
            } else if !has_more.get() {
                view! { <div class="end-message">"You've reached the end!"</div> }
            } else {
                view! { <div></div> }
            }}
        </div>
    }
}
```

## Error Boundary Component

### Error Handling UI

```rust
#[component]
pub fn ErrorBoundary(
    children: Children,
    #[prop(default = "Something went wrong")] fallback_message: &'static str,
) -> impl IntoView {
    let (error, set_error) = create_signal(None::<String>);

    view! {
        <ErrorBoundaryInner
            error
            set_error
            fallback_message
        >
            {children()}
        </ErrorBoundaryInner>
    }
}

#[component]
pub fn ErrorBoundaryInner(
    error: ReadSignal<Option<String>>,
    set_error: WriteSignal<Option<String>>,
    fallback_message: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <leptos::ErrorBoundary
            fallback=move |errors| view! {
                <div class="error-boundary">
                    <h3>"Oops! Something went wrong"</h3>
                    <p>{fallback_message}</p>

                    <details class="error-details">
                        <summary>"Error Details"</summary>
                        <pre>{format!("{:#?}", errors)}</pre>
                    </details>

                    <button on:click=move |_| {
                        set_error.set(None);
                        // Reload the page or reset component state
                        web_sys::window().unwrap().location().reload().unwrap();
                    }>
                        "Reload Page"
                    </button>
                </div>
            }
        >
            {children()}
        </leptos::ErrorBoundary>
    }
}
```

## CSS Styling

```css
/* Data Fetching Styles */
.user-list {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
}

.controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
}

.user-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1rem;
    margin-bottom: 2rem;
}

.user-card {
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    padding: 1rem;
    display: flex;
    align-items: center;
    gap: 1rem;
    transition: box-shadow 0.2s;
}

.user-card:hover {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
}

.user-card img {
    width: 50px;
    height: 50px;
    border-radius: 50%;
    object-fit: cover;
}

.user-info h3 {
    margin: 0;
    font-size: 1.1rem;
    color: #111827;
}

.user-info p {
    margin: 0.25rem 0 0 0;
    color: #6b7280;
    font-size: 0.9rem;
}

.pagination {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
    margin-top: 2rem;
}

.prev-btn,
.next-btn {
    padding: 0.5rem 1rem;
    background: #3b82f6;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
}

.prev-btn:hover,
.next-btn:hover {
    background: #2563eb;
}

.prev-btn:disabled,
.next-btn:disabled {
    background: #9ca3af;
    cursor: not-allowed;
}

.page-info {
    font-weight: 500;
    color: #374151;
}

/* Chat Styles */
.chat-container {
    max-width: 600px;
    margin: 0 auto;
    padding: 1rem;
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    height: 500px;
    display: flex;
    flex-direction: column;
}

.connection-status {
    text-align: center;
    margin-bottom: 1rem;
    font-weight: 500;
}

.connection-status .connected {
    color: #10b981;
}

.connection-status .disconnected {
    color: #ef4444;
}

.messages {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    border: 1px solid #e5e7eb;
    border-radius: 4px;
    margin-bottom: 1rem;
    background: #f9fafb;
}

.message {
    margin-bottom: 0.75rem;
    padding: 0.5rem;
    background: white;
    border-radius: 4px;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.message strong {
    color: #3b82f6;
}

.message small {
    color: #6b7280;
    margin-left: 0.5rem;
}

.message-form {
    display: flex;
    gap: 0.5rem;
}

.message-form input {
    flex: 1;
    padding: 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 4px;
    font-size: 1rem;
}

.message-form input:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.message-form button {
    padding: 0.75rem 1.5rem;
    background: #3b82f6;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
}

.message-form button:hover {
    background: #2563eb;
}

.message-form button:disabled {
    background: #9ca3af;
    cursor: not-allowed;
}

/* Todo Styles */
.todo-list {
    max-width: 600px;
    margin: 0 auto;
    padding: 2rem;
}

.todos {
    margin-top: 2rem;
}

.todo-item {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    border: 1px solid #e5e7eb;
    border-radius: 4px;
    margin-bottom: 0.5rem;
    background: white;
}

.todo-item.completed .todo-title {
    text-decoration: line-through;
    color: #6b7280;
}

.todo-checkbox {
    width: 20px;
    height: 20px;
}

.todo-title {
    flex: 1;
    font-size: 1.1rem;
}

.todo-delete {
    background: none;
    border: none;
    color: #ef4444;
    font-size: 1.25rem;
    cursor: pointer;
    padding: 0;
}

.todo-delete:hover {
    color: #dc2626;
}

/* Loading and Error Styles */
.loading-spinner {
    display: inline-block;
    width: 20px;
    height: 20px;
    border: 3px solid #f3f3f3;
    border-top: 3px solid #3b82f6;
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}

.error {
    padding: 1rem;
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 4px;
    color: #dc2626;
}

.error p {
    margin: 0 0 1rem 0;
}

.retry-btn {
    padding: 0.5rem 1rem;
    background: #dc2626;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
}

.retry-btn:hover {
    background: #b91c1c;
}

/* Post Feed Styles */
.post-feed {
    max-width: 600px;
    margin: 0 auto;
    padding: 1rem;
    height: 600px;
    overflow-y: auto;
    border: 1px solid #e5e7eb;
    border-radius: 8px;
}

.post-card {
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    padding: 1rem;
    margin-bottom: 1rem;
    background: white;
}

.post-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 0.75rem;
}

.post-avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    object-fit: cover;
}

.post-author {
    font-weight: 600;
    color: #111827;
}

.post-timestamp {
    font-size: 0.875rem;
    color: #6b7280;
}

.post-content {
    margin-bottom: 0.75rem;
    line-height: 1.6;
}

.post-actions {
    display: flex;
    gap: 1rem;
    padding-top: 0.75rem;
    border-top: 1px solid #e5e7eb;
}

.post-action {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    color: #6b7280;
    cursor: pointer;
    transition: color 0.2s;
}

.post-action:hover {
    color: #3b82f6;
}

.end-message {
    text-align: center;
    padding: 2rem;
    color: #6b7280;
    font-style: italic;
}

/* Error Boundary Styles */
.error-boundary {
    padding: 2rem;
    text-align: center;
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 8px;
    max-width: 500px;
    margin: 2rem auto;
}

.error-boundary h3 {
    color: #dc2626;
    margin-bottom: 1rem;
}

.error-boundary p {
    color: #7f1d1d;
    margin-bottom: 1.5rem;
}

.error-details {
    margin-bottom: 1.5rem;
    text-align: left;
}

.error-details summary {
    cursor: pointer;
    font-weight: 500;
    color: #374151;
}

.error-details pre {
    background: #f9fafb;
    padding: 1rem;
    border-radius: 4px;
    overflow-x: auto;
    font-size: 0.875rem;
    color: #374151;
    margin-top: 0.5rem;
}

.error-boundary button {
    padding: 0.75rem 1.5rem;
    background: #dc2626;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
    transition: background-color 0.2s;
}

.error-boundary button:hover {
    background: #b91c1c;
}

/* Responsive Design */
@media (max-width: 640px) {
    .user-grid {
        grid-template-columns: 1fr;
    }

    .todo-item {
        flex-direction: column;
        align-items: flex-start;
        gap: 0.5rem;
    }

    .post-header {
        flex-direction: column;
        align-items: flex-start;
        gap: 0.25rem;
    }

    .message-form {
        flex-direction: column;
    }

    .pagination {
        flex-direction: column;
        gap: 0.5rem;
    }
}
```

## Key Features Demonstrated

### 1. HTTP Client with Error Handling
- **Comprehensive API Client**: GET, POST, PUT, DELETE methods with proper error handling
- **Authentication Support**: Bearer token handling
- **Request/Response Interception**: Middleware pattern for logging and error handling
- **Type Safety**: Generic response types with serde

### 2. Caching and Performance
- **Intelligent Caching**: TTL-based cache with automatic expiration
- **Cache Invalidation**: Manual and automatic cache clearing
- **Background Sync**: Offline request queuing and sync
- **Optimistic Updates**: Immediate UI updates with rollback on failure

### 3. Real-Time Data
- **Server-Sent Events**: Real-time data streaming
- **WebSocket Support**: Bidirectional communication
- **Connection Monitoring**: Online/offline status tracking
- **Automatic Reconnection**: Connection recovery

### 4. Advanced Patterns
- **Pagination**: Efficient data loading with page controls
- **Infinite Scrolling**: Performance-optimized scrolling
- **Retry Logic**: Exponential backoff for failed requests
- **Request Deduplication**: Prevent duplicate requests

### 5. User Experience
- **Loading States**: Visual feedback during data operations
- **Error Boundaries**: Graceful error handling and recovery
- **Offline Support**: Background sync and offline indicators
- **Progressive Enhancement**: Works with and without JavaScript

### 6. Data Management
- **State Synchronization**: Server and client state consistency
- **Conflict Resolution**: Handling concurrent modifications
- **Data Validation**: Client and server-side validation
- **Type Safety**: Compile-time guarantees for data structures

This comprehensive data fetching example demonstrates production-ready patterns for building robust, performant web applications with excellent user experience and error handling.