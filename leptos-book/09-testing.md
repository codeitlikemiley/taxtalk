# 09: Testing

## Overview

Comprehensive testing is essential for building reliable Leptos applications. This chapter covers the testing ecosystem, strategies, and best practices for testing Leptos components, server functions, and full application workflows.

## Testing Ecosystem

### Leptos Testing Tools

Leptos provides several testing utilities and integrations:

- **leptos_test**: Core testing utilities for components and server functions
- **leptos_router**: Testing utilities for routing
- **leptos_meta**: Testing utilities for meta tags
- **wasm-bindgen-test**: For testing in browser environment
- **tokio-test**: For async testing

### Cargo.toml Configuration

Set up testing dependencies:

```toml
[dev-dependencies]
leptos_test = "0.6"
wasm-bindgen-test = "0.3"
tokio-test = "0.4"
serde_json = "1.0"
pretty_assertions = "1.4"
rstest = "0.18"
```

## Component Testing

### Basic Component Testing

Test individual components in isolation:

```rust
use leptos::*;
use leptos_test::*;

#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    
    view! {
        <div>
            <p>"Count: " {count}</p>
            <button on:click=move |_| set_count.update(|c| *c + 1)>
                "Increment"
            </button>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos_test::*;
    
    #[test]
    fn test_counter_initial_state() {
        let mut cx = TestingContext::new();
        
        // Mount the component
        let view = cx.mount(|| view! { <Counter/> });
        
        // Check initial state
        let count_text = view.find_by_text("Count: 0");
        assert!(count_text.is_some());
    }
    
    #[test]
    fn test_counter_increment() {
        let mut cx = TestingContext::new();
        
        // Mount the component
        let view = cx.mount(|| view! { <Counter/> });
        
        // Find and click the button
        let button = view.find_by_text("Increment").unwrap();
        button.click();
        
        // Check updated state
        let count_text = view.find_by_text("Count: 1");
        assert!(count_text.is_some());
    }
}
```

### Advanced Component Testing

Test components with complex state and effects:

```rust
use leptos::*;
use leptos_test::*;

#[component]
pub fn TodoList() -> impl IntoView {
    let (todos, set_todos) = create_signal(Vec::<String>::new());
    let (new_todo, set_new_todo) = create_signal(String::new());
    
    let add_todo = move |_| {
        let todo = new_todo.get();
        if !todo.is_empty() {
            set_todos.update(|todos| todos.push(todo.clone()));
            set_new_todo.set(String::new());
        }
    };
    
    view! {
        <div>
            <input
                type="text"
                prop:value=new_todo
                on:input=move |ev| set_new_todo.set(event_target_value(&ev))
            />
            <button on:click=add_todo>"Add Todo"</button>
            
            <ul>
                {move || todos.get().into_iter().map(|todo| view! {
                    <li>{todo}</li>
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos_test::*;
    
    #[test]
    fn test_add_todo() {
        let mut cx = TestingContext::new();
        
        let view = cx.mount(|| view! { <TodoList/> });
        
        // Find input and button
        let input = view.find_by_selector("input[type='text']").unwrap();
        let button = view.find_by_text("Add Todo").unwrap();
        
        // Type in the input
        input.set_value("Test todo");
        
        // Click add button
        button.click();
        
        // Check that todo was added
        let todo_item = view.find_by_text("Test todo");
        assert!(todo_item.is_some());
        
        // Check that input was cleared
        let input_element = view.find_by_selector("input[type='text']").unwrap();
        assert_eq!(input_element.value(), "");
    }
    
    #[test]
    fn test_empty_todo_not_added() {
        let mut cx = TestingContext::new();
        
        let view = cx.mount(|| view! { <TodoList/> });
        
        let button = view.find_by_text("Add Todo").unwrap();
        button.click();
        
        // Should not add empty todo
        let todos = view.find_all_by_selector("li");
        assert_eq!(todos.len(), 0);
    }
}
```

### Testing with Resources

Test components that use resources:

```rust
use leptos::*;
use leptos_test::*;

#[component]
pub fn UserProfile(user_id: u32) -> impl IntoView {
    let user = create_resource(
        move || user_id,
        |id| async move {
            // Simulate API call
            fetch_user(id).await
        }
    );
    
    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            {move || user.get().map(|user| view! {
                <div>
                    <h1>{&user.name}</h1>
                    <p>{&user.email}</p>
                </div>
            })}
        </Suspense>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos_test::*;
    
    #[test]
    fn test_user_profile_loading() {
        let mut cx = TestingContext::new();
        
        let view = cx.mount(|| view! { <UserProfile user_id=1/> });
        
        // Should show loading state initially
        let loading = view.find_by_text("Loading...");
        assert!(loading.is_some());
    }
    
    #[test]
    fn test_user_profile_loaded() {
        let mut cx = TestingContext::new();
        
        // Mock the fetch_user function to return immediately
        cx.mock_async(|_| async {
            Ok(User {
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
            })
        });
        
        let view = cx.mount(|| view! { <UserProfile user_id=1/> });
        
        // Should show user data
        let name = view.find_by_text("John Doe");
        let email = view.find_by_text("john@example.com");
        
        assert!(name.is_some());
        assert!(email.is_some());
    }
}
```

## Server Function Testing

### Testing Server Functions

Test server functions in isolation:

```rust
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[server(GetUser, "/api")]
pub async fn get_user(id: u32) -> Result<User, ServerFnError> {
    // Simulate database query
    if id == 1 {
        Ok(User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        })
    } else {
        Err(ServerFnError::ServerError("User not found".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos_test::*;
    
    #[tokio::test]
    async fn test_get_user_success() {
        let result = get_user(1).await;
        
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
    }
    
    #[tokio::test]
    async fn test_get_user_not_found() {
        let result = get_user(999).await;
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("User not found"));
    }
}
```

### Testing with Database

Test server functions that interact with databases:

```rust
use leptos::*;
use sqlx::SqlitePool;

#[server(CreateUser, "/api")]
pub async fn create_user(name: String, email: String) -> Result<User, ServerFnError> {
    let pool = get_db_pool()?;
    
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email) VALUES (?, ?) RETURNING id, name, email",
        name,
        email
    )
    .fetch_one(&pool)
    .await?;
    
    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Sqlite;
    use sqlx::migrate::MigrateDatabase;
    
    #[sqlx::test]
    async fn test_create_user(pool: SqlitePool) {
        // Set up test database
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        
        let result = create_user(
            "Jane Doe".to_string(),
            "jane@example.com".to_string()
        ).await;
        
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Jane Doe");
        assert_eq!(user.email, "jane@example.com");
    }
}
```

## Integration Testing

### Full Application Testing

Test complete application workflows:

```rust
use leptos::*;
use leptos_router::*;
use leptos_test::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
                <Route path="/users/:id" view=UserPage/>
            </Routes>
        </Router>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div>
            <h1>"Home Page"</h1>
            <a href="/users/1">"View User 1"</a>
        </div>
    }
}

#[component]
pub fn UserPage() -> impl IntoView {
    let params = use_params_map();
    let user_id = move || params.get("id").unwrap_or_default();
    
    let user = create_resource(
        move || user_id(),
        |id| async move {
            get_user(id.parse().unwrap_or(0)).await.ok()
        }
    );
    
    view! {
        <div>
            <h1>"User Page"</h1>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || user.get().flatten().map(|user| view! {
                    <div>
                        <h2>{&user.name}</h2>
                        <p>{&user.email}</p>
                    </div>
                })}
            </Suspense>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos_test::*;
    
    #[test]
    fn test_navigation() {
        let mut cx = TestingContext::new();
        
        let view = cx.mount(|| view! { <App/> });
        
        // Should start on home page
        let home_title = view.find_by_text("Home Page");
        assert!(home_title.is_some());
        
        // Navigate to user page
        cx.navigate("/users/1");
        
        // Should show user page
        let user_title = view.find_by_text("User Page");
        assert!(user_title.is_some());
    }
    
    #[test]
    fn test_user_page_with_mock() {
        let mut cx = TestingContext::new();
        
        // Mock the server function
        cx.mock_server_fn(|_| async {
            Ok(User {
                id: 1,
                name: "Test User".to_string(),
                email: "test@example.com".to_string(),
            })
        });
        
        let view = cx.mount(|| view! { <UserPage/> });
        
        // Should show user data
        let name = view.find_by_text("Test User");
        let email = view.find_by_text("test@example.com");
        
        assert!(name.is_some());
        assert!(email.is_some());
    }
}
```

## Testing Utilities

### Custom Testing Helpers

Create reusable testing utilities:

```rust
use leptos::*;
use leptos_test::*;

// Custom testing utilities
pub struct TestApp {
    cx: TestingContext,
    view: View,
}

impl TestApp {
    pub fn new() -> Self {
        let mut cx = TestingContext::new();
        let view = cx.mount(|| view! { <App/> });
        
        Self { cx, view }
    }
    
    pub fn navigate(&mut self, path: &str) {
        self.cx.navigate(path);
    }
    
    pub fn find_by_text(&self, text: &str) -> Option<Element> {
        self.view.find_by_text(text)
    }
    
    pub fn click(&mut self, selector: &str) {
        if let Some(element) = self.view.find_by_selector(selector) {
            element.click();
        }
    }
    
    pub fn type_text(&mut self, selector: &str, text: &str) {
        if let Some(element) = self.view.find_by_selector(selector) {
            element.set_value(text);
        }
    }
}

// Usage in tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_workflow() {
        let mut app = TestApp::new();
        
        // Navigate to a page
        app.navigate("/users");
        
        // Fill out a form
        app.type_text("input[name='name']", "John Doe");
        app.type_text("input[name='email']", "john@example.com");
        
        // Submit the form
        app.click("button[type='submit']");
        
        // Check that user was created
        let success_message = app.find_by_text("User created successfully");
        assert!(success_message.is_some());
    }
}
```

### Snapshot Testing

Test component output with snapshots:

```rust
use leptos::*;
use leptos_test::*;
use std::fs;

#[component]
pub fn UserCard(user: User) -> impl IntoView {
    view! {
        <div class="user-card">
            <img src=&user.avatar_url alt=&user.name/>
            <h3>{&user.name}</h3>
            <p>{&user.email}</p>
            <div class="stats">
                <span>"Posts: " {&user.post_count}</span>
                <span>"Followers: " {&user.follower_count}</span>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_card_snapshot() {
        let user = User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            avatar_url: "https://example.com/avatar.jpg".to_string(),
            post_count: 42,
            follower_count: 1337,
        };
        
        let mut cx = TestingContext::new();
        let view = cx.mount(|| view! { <UserCard user=user/> });
        
        // Generate HTML snapshot
        let html = view.html();
        
        // In a real implementation, you'd compare against a stored snapshot
        // For now, just check that it contains expected content
        assert!(html.contains("John Doe"));
        assert!(html.contains("john@example.com"));
        assert!(html.contains("Posts: 42"));
        assert!(html.contains("Followers: 1337"));
    }
}
```

## Testing Best Practices

### Test Organization

Organize tests by functionality:

```
src/
├── components/
│   ├── button.rs
│   ├── form.rs
│   └── modal.rs
└── lib.rs

tests/
├── unit/
│   ├── components/
│   │   ├── button_test.rs
│   │   ├── form_test.rs
│   │   └── modal_test.rs
│   └── lib_test.rs
├── integration/
│   ├── user_workflow_test.rs
│   └── admin_workflow_test.rs
└── e2e/
    ├── user_journey_test.rs
    └── admin_journey_test.rs
```

### Test Naming Conventions

Use descriptive test names:

```rust
#[cfg(test)]
mod tests {
    // Good test names
    #[test]
    fn increments_counter_when_button_clicked() {}
    
    #[test]
    fn shows_error_message_when_form_validation_fails() {}
    
    #[test]
    fn redirects_to_login_when_user_not_authenticated() {}
    
    #[test]
    fn loads_user_data_from_api_on_mount() {}
    
    // Bad test names (too vague)
    #[test]
    fn test_counter() {}  // What aspect of counter?
    
    #[test]
    fn test_form() {}     // What behavior?
    
    #[test]
    fn test_user() {}     // What user functionality?
}
```

### Test Data Management

Use factories for test data:

```rust
#[cfg(test)]
pub mod factories {
    use super::*;
    
    pub fn create_test_user() -> User {
        User {
            id: 1,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            avatar_url: "https://example.com/avatar.jpg".to_string(),
            post_count: 10,
            follower_count: 100,
        }
    }
    
    pub fn create_test_post() -> Post {
        Post {
            id: 1,
            title: "Test Post".to_string(),
            content: "This is a test post content.".to_string(),
            author_id: 1,
            created_at: chrono::Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use factories::*;
    
    #[test]
    fn test_user_display() {
        let user = create_test_user();
        let mut cx = TestingContext::new();
        
        let view = cx.mount(|| view! { <UserCard user=user/> });
        
        assert!(view.find_by_text("Test User").is_some());
    }
}
```

## Performance Testing

### Component Performance Testing

Test component rendering performance:

```rust
use leptos::*;
use leptos_test::*;
use std::time::Instant;

#[component]
pub fn LargeList(items: Vec<String>) -> impl IntoView {
    view! {
        <ul>
            {items.into_iter().map(|item| view! {
                <li>{item}</li>
            }).collect::<Vec<_>>()}
        </ul>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_large_list_performance() {
        let items: Vec<String> = (0..1000)
            .map(|i| format!("Item {}", i))
            .collect();
        
        let start = Instant::now();
        
        let mut cx = TestingContext::new();
        let _view = cx.mount(|| view! { <LargeList items=items/> });
        
        let duration = start.elapsed();
        
        // Should render within reasonable time
        assert!(duration.as_millis() < 100, "Rendering took too long: {:?}", duration);
    }
}
```

## End-to-End Testing

### Browser Testing with Playwright

Set up end-to-end tests:

```rust
// In a separate crate or test file
use playwright::api::Page;

#[tokio::test]
async fn test_user_registration() {
    let playwright = Playwright::initialize().await.unwrap();
    let browser = playwright.chromium().launch().await.unwrap();
    let page = browser.new_page().await.unwrap();
    
    // Navigate to the app
    page.goto("http://localhost:3000").await.unwrap();
    
    // Fill out registration form
    page.fill("input[name='name']", "John Doe").await.unwrap();
    page.fill("input[name='email']", "john@example.com").await.unwrap();
    page.fill("input[name='password']", "password123").await.unwrap();
    
    // Submit form
    page.click("button[type='submit']").await.unwrap();
    
    // Wait for success message
    page.wait_for_selector("text=Registration successful").await.unwrap();
    
    // Verify user is logged in
    let user_menu = page.query_selector("text=John Doe").await.unwrap();
    assert!(user_menu.is_some());
    
    browser.close().await.unwrap();
}
```

## Testing Checklist

### Component Testing Checklist

- [ ] Tests initial render state
- [ ] Tests user interactions (clicks, typing, etc.)
- [ ] Tests state updates and reactive behavior
- [ ] Tests error states and edge cases
- [ ] Tests accessibility features
- [ ] Tests responsive behavior (if applicable)
- [ ] Tests performance with large datasets
- [ ] Tests cleanup and memory leaks

### Server Function Testing Checklist

- [ ] Tests successful execution
- [ ] Tests error conditions
- [ ] Tests input validation
- [ ] Tests database interactions
- [ ] Tests external API calls
- [ ] Tests authentication and authorization
- [ ] Tests rate limiting
- [ ] Tests concurrent access

### Integration Testing Checklist

- [ ] Tests complete user workflows
- [ ] Tests navigation and routing
- [ ] Tests data flow between components
- [ ] Tests server-client communication
- [ ] Tests error handling across boundaries
- [ ] Tests performance under load
- [ ] Tests browser compatibility

### E2E Testing Checklist

- [ ] Tests critical user journeys
- [ ] Tests cross-browser compatibility
- [ ] Tests mobile responsiveness
- [ ] Tests accessibility compliance
- [ ] Tests performance in real browsers
- [ ] Tests error recovery
- [ ] Tests offline functionality (if applicable)

## Continuous Integration

### GitHub Actions Setup

Set up CI for testing:

```yaml
# .github/workflows/test.yml
name: Test

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable
      
    - name: Cache dependencies
      uses: Swatinem/rust-cache@v2
      
    - name: Run tests
      run: cargo test
      
    - name: Run tests with coverage
      run: |
        cargo install cargo-tarpaulin
        cargo tarpaulin --out Html
        
    - name: Upload coverage
      uses: codecov/codecov-action@v3
      with:
        file: ./tarpaulin-report.html
```

### Test Coverage

Configure test coverage:

```toml
# In Cargo.toml
[package]
name = "my-leptos-app"
version = "0.1.0"
edition = "2021"

[features]
default = ["csr"]
csr = []
hydrate = ["leptos/hydrate"]
ssr = ["leptos/ssr"]

[dev-dependencies]
leptos_test = "0.6"
wasm-bindgen-test = "0.3"
tokio-test = "0.4"
serde_json = "1.0"
pretty_assertions = "1.4"
rstest = "0.18"
```

## Debugging Tests

### Common Testing Issues

Debug common testing problems:

```rust
#[cfg(test)]
mod debug_tests {
    use super::*;
    use leptos_test::*;
    
    // Debug test for component rendering
    #[test]
    fn debug_component_rendering() {
        let mut cx = TestingContext::new();
        
        let view = cx.mount(|| view! { <MyComponent/> });
        
        // Print the HTML for debugging
        println!("Rendered HTML: {}", view.html());
        
        // Print all text content
        let all_text = view.find_all_by_selector("*")
            .iter()
            .filter_map(|el| el.text_content())
            .collect::<Vec<_>>();
        println!("All text: {:?}", all_text);
        
        // This test will always pass but helps with debugging
        assert!(true);
    }
    
    // Test with detailed assertions
    #[test]
    fn test_with_detailed_logging() {
        let mut cx = TestingContext::new();
        
        let view = cx.mount(|| view! { <ComplexComponent/> });
        
        // Log what we find
        if let Some(button) = view.find_by_text("Submit") {
            println!("Found submit button");
            button.click();
            println!("Clicked submit button");
        } else {
            println!("Submit button not found");
            let all_buttons = view.find_all_by_selector("button");
            println!("Found {} buttons", all_buttons.len());
            for button in all_buttons {
                println!("Button text: {:?}", button.text_content());
            }
        }
        
        assert!(true);
    }
}
```

---

**Next:** [10: Deployment](10-deployment.md) - Learn about building and deploying Leptos applications.