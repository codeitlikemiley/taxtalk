# 06: Data Flow

## Overview

Effective data flow management is crucial for building maintainable Leptos applications. This chapter covers the fundamental patterns for passing data between components, managing application state, and implementing proper data flow architectures. We'll explore props, context, stores, and reactive data patterns that enable clean, predictable data flow throughout your application.

## Component Props

### Basic Props

Props are the primary mechanism for passing data from parent to child components. Leptos provides a type-safe props system:

```rust
use leptos::*;

// Define props struct
#[derive(Clone, Debug)]
pub struct UserCardProps {
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub is_online: bool,
}

// Component with props
#[component]
pub fn UserCard(props: UserCardProps) -> impl IntoView {
    view! {
        <div class="user-card">
            <div class="avatar">
                {props.avatar_url
                    .map(|url| view! { <img src=url alt="Avatar" /> })
                    .unwrap_or_else(|| view! { <div class="default-avatar">"U"</div> })}
            </div>
            <div class="user-info">
                <h3>{props.name}</h3>
                <p>{props.email}</p>
                <span class=move || if props.is_online { "online" } else { "offline" }>
                    {move || if props.is_online { "Online" } else { "Offline" }}
                </span>
            </div>
        </div>
    }
}
```

### Optional Props

Handle optional props gracefully:

```rust
use leptos::*;

// Props with optional fields
#[derive(Clone, Debug)]
pub struct ProductCardProps {
    pub name: String,
    pub price: f64,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub in_stock: bool,
    pub discount_percentage: Option<u8>,
}

#[component]
pub fn ProductCard(props: ProductCardProps) -> impl IntoView {
    let discounted_price = move || {
        props.discount_percentage.map(|discount| {
            let discount_amount = props.price * (discount as f64 / 100.0);
            props.price - discount_amount
        })
    };

    view! {
        <div class="product-card">
            {props.image_url
                .map(|url| view! { <img src=url alt=props.name.clone() /> })
                .unwrap_or_else(|| view! { <div class="no-image">"No Image"</div> })}
            
            <h3>{props.name}</h3>
            
            <div class="price-section">
                {move || discounted_price().map(|discounted| view! {
                    <div>
                        <span class="original-price">"${}" {props.price}</span>
                        <span class="discounted-price">"${}" {format!("{:.2}", discounted)}</span>
                        <span class="discount-badge>"-{props.discount_percentage.unwrap()}%</span>
                    </div>
                }).unwrap_or_else(|| view! {
                    <span class="price">"${}" {format!("{:.2}", props.price)}</span>
                })}
            </div>
            
            {props.description.map(|desc| view! {
                <p class="description">{desc}</p>
            })}
            
            <button 
                class="add-to-cart"
                disabled=move || !props.in_stock
            >
                {move || if props.in_stock { "Add to Cart" } else { "Out of Stock" }}
            </button>
        </div>
    }
}
```

### Props with Callbacks

Pass callback functions as props for child-to-parent communication:

```rust
use leptos::*;

// Props with callback
#[derive(Clone)]
pub struct TodoItemProps {
    pub id: u32,
    pub text: String,
    pub completed: bool,
    pub on_toggle: Action<u32, ()>,
    pub on_delete: Action<u32, ()>,
    pub on_edit: Action<(u32, String), ()>,
}

#[component]
pub fn TodoItem(props: TodoItemProps) -> impl IntoView {
    let (is_editing, set_is_editing) = create_signal(false);
    let (edit_text, set_edit_text) = create_signal(props.text.clone());

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        props.on_edit.dispatch((props.id, edit_text.get()));
        set_is_editing.set(false);
    };

    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Escape" {
            set_edit_text.set(props.text.clone());
            set_is_editing.set(false);
        }
    };

    view! {
        <li class="todo-item" class:completed=props.completed>
            {move || if is_editing.get() {
                view! {
                    <form on:submit=handle_submit>
                        <input
                            type="text"
                            prop:value=edit_text
                            on:input=move |ev| set_edit_text.set(event_target_value(&ev))
                            on:keydown=handle_keydown
                            autofocus
                        />
                    </form>
                }
            } else {
                view! {
                    <div>
                        <input
                            type="checkbox"
                            prop:checked=props.completed
                            on:change=move |_| props.on_toggle.dispatch(props.id)
                        />
                        <span 
                            class="todo-text"
                            on:dblclick=move |_| set_is_editing.set(true)
                        >
                            {props.text}
                        </span>
                        <button on:click=move |_| props.on_delete.dispatch(props.id)>
                            "Delete"
                        </button>
                    </div>
                }
            }}
        </li>
    }
}
```

## Context API

### Creating and Providing Context

Context allows you to share data across the component tree without explicitly passing props:

```rust
use leptos::*;

// Define context type
#[derive(Clone, Debug)]
pub struct UserContext {
    pub current_user: Signal<Option<User>>,
    pub login: Action<LoginCredentials, Result<User, String>>,
    pub logout: Action<(), ()>,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub role: UserRole,
}

#[derive(Clone, Debug)]
pub enum UserRole {
    Admin,
    User,
    Guest,
}

#[derive(Clone, Debug)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

// Context provider component
#[component]
pub fn App() -> impl IntoView {
    // Create reactive state
    let (current_user, set_current_user) = create_signal(None::<User>);
    
    // Create actions
    let login = create_action(move |credentials: &LoginCredentials| {
        let creds = credentials.clone();
        async move {
            // Simulate API call
            if creds.email == "admin@example.com" && creds.password == "password" {
                Ok(User {
                    id: 1,
                    name: "Admin User".to_string(),
                    email: creds.email,
                    role: UserRole::Admin,
                })
            } else {
                Err("Invalid credentials".to_string())
            }
        }
    });
    
    let logout = create_action(move |_: &()| async move {
        // Simulate logout
        set_current_user.set(None);
    });
    
    // Update user when login succeeds
    create_effect(move |_| {
        if let Some(result) = login.value().get() {
            match result {
                Ok(user) => set_current_user.set(Some(user)),
                Err(_) => set_current_user.set(None),
            }
        }
    });
    
    // Create context value
    let context = UserContext {
        current_user: current_user.into(),
        login,
        logout,
    };
    
    // Provide context to children
    provide_context(context);
    
    view! {
        <div class="app">
            <Header />
            <main>
                <Router>
                    <Routes>
                        <Route path="/" view=HomePage />
                        <Route path="/dashboard" view=Dashboard />
                        <Route path="/admin" view=AdminPanel />
                    </Routes>
                </Router>
            </main>
        </div>
    }
}
```

### Consuming Context

Access context in child components:

```rust
use leptos::*;

// Component that consumes context
#[component]
pub fn Header() -> impl IntoView {
    // Get context
    let user_context = use_context::<UserContext>()
        .expect("UserContext should be provided");
    
    let current_user = user_context.current_user;
    let logout = user_context.logout;
    
    view! {
        <header>
            <h1>"My App"</h1>
            <nav>
                <a href="/">"Home"</a>
                <a href="/dashboard">"Dashboard"</a>
                {move || current_user.get().and_then(|user| {
                    if matches!(user.role, UserRole::Admin) {
                        Some(view! { <a href="/admin">"Admin"</a> })
                    } else {
                        None
                    }
                })}
            </nav>
            <div class="user-section">
                {move || current_user.get().map(|user| view! {
                    <div class="user-info">
                        <span>"Welcome, " {user.name}</span>
                        <button on:click=move |_| logout.dispatch(())>
                            "Logout"
                        </button>
                    </div>
                }).unwrap_or_else(|| view! {
                    <LoginForm />
                })}
            </div>
        </header>
    }
}

// Login form component
#[component]
pub fn LoginForm() -> impl IntoView {
    let user_context = use_context::<UserContext>()
        .expect("UserContext should be provided");
    
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let login = user_context.login;
    
    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        login.dispatch(LoginCredentials {
            email: email.get(),
            password: password.get(),
        });
    };
    
    view! {
        <form on:submit=handle_submit class="login-form">
            <input
                type="email"
                placeholder="Email"
                prop:value=email
                on:input=move |ev| set_email.set(event_target_value(&ev))
            />
            <input
                type="password"
                placeholder="Password"
                prop:value=password
                on:input=move |ev| set_password.set(event_target_value(&ev))
            />
            <button type="submit" disabled=login.pending()>
                {move || if login.pending().get() { "Logging in..." } else { "Login" }}
            </button>
            {move || login.value().get().and_then(|result| {
                result.as_ref().err().map(|error| view! {
                    <p class="error">{error}</p>
                })
            })}
        </form>
    }
}
```

### Multiple Contexts

Use multiple contexts for different concerns:

```rust
use leptos::*;

// Theme context
#[derive(Clone, Debug)]
pub struct ThemeContext {
    pub theme: Signal<Theme>,
    pub set_theme: WriteSignal<Theme>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

// Notification context
#[derive(Clone, Debug)]
pub struct NotificationContext {
    pub notifications: Signal<Vec<Notification>>,
    pub add_notification: Action<Notification, ()>,
    pub remove_notification: Action<u32, ()>,
}

#[derive(Clone, Debug)]
pub struct Notification {
    pub id: u32,
    pub message: String,
    pub type_: NotificationType,
    pub duration: Option<std::time::Duration>,
}

#[derive(Clone, Debug)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

// App with multiple contexts
#[component]
pub fn AppWithContexts() -> impl IntoView {
    // Theme context
    let (theme, set_theme) = create_signal(Theme::Auto);
    provide_context(ThemeContext {
        theme: theme.into(),
        set_theme,
    });
    
    // Notification context
    let (notifications, set_notifications) = create_signal(vec![]);
    let next_id = create_rw_signal(0);
    
    let add_notification = create_action(move |notification: &Notification| {
        let mut notif = notification.clone();
        notif.id = next_id.get();
        next_id.update(|id| *id += 1);
        
        set_notifications.update(|notifs| notifs.push(notif.clone()));
        
        // Auto-remove after duration
        if let Some(duration) = notification.duration {
            set_timeout_with_handle(
                move || {
                    set_notifications.update(|notifs| {
                        notifs.retain(|n| n.id != notification.id);
                    });
                },
                duration
            );
        }
    });
    
    let remove_notification = create_action(move |id: &u32| {
        set_notifications.update(|notifs| {
            notifs.retain(|n| n.id != *id);
        });
    });
    
    provide_context(NotificationContext {
        notifications: notifications.into(),
        add_notification,
        remove_notification,
    });
    
    view! {
        <div class="app">
            <ThemeProvider>
                <NotificationProvider>
                    <Header />
                    <main>
                        <Router>
                            <Routes>
                                <Route path="/" view=HomePage />
                                <Route path="/settings" view=SettingsPage />
                            </Routes>
                        </Router>
                    </main>
                    <NotificationContainer />
                </NotificationProvider>
            </ThemeProvider>
        </div>
    }
}
```

## Reactive Stores

### Basic Store Pattern

Stores provide a more structured way to manage complex state:

```rust
use leptos::*;

// Define store structure
#[derive(Clone, Debug)]
pub struct TodoStore {
    pub todos: Signal<Vec<Todo>>,
    pub filter: Signal<TodoFilter>,
    pub add_todo: Action<String, ()>,
    pub toggle_todo: Action<u32, ()>,
    pub delete_todo: Action<u32, ()>,
    pub set_filter: Action<TodoFilter, ()>,
    pub clear_completed: Action<(), ()>,
}

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TodoFilter {
    All,
    Active,
    Completed,
}

// Store implementation
pub fn create_todo_store() -> TodoStore {
    let (todos, set_todos) = create_signal(vec![]);
    let (filter, set_filter) = create_signal(TodoFilter::All);
    let next_id = create_rw_signal(0);
    
    let add_todo = create_action(move |text: &String| {
        let id = next_id.get();
        next_id.update(|n| *n += 1);
        
        let todo = Todo {
            id,
            text: text.clone(),
            completed: false,
            created_at: chrono::Utc::now(),
        };
        
        set_todos.update(|todos| todos.push(todo));
    });
    
    let toggle_todo = create_action(move |id: &u32| {
        set_todos.update(|todos| {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == *id) {
                todo.completed = !todo.completed;
            }
        });
    });
    
    let delete_todo = create_action(move |id: &u32| {
        set_todos.update(|todos| {
            todos.retain(|t| t.id != *id);
        });
    });
    
    let set_filter_action = create_action(move |new_filter: &TodoFilter| {
        set_filter.set(new_filter.clone());
    });
    
    let clear_completed = create_action(move |_: &()| {
        set_todos.update(|todos| {
            todos.retain(|t| !t.completed);
        });
    });
    
    TodoStore {
        todos: todos.into(),
        filter: filter.into(),
        add_todo,
        toggle_todo,
        delete_todo,
        set_filter: set_filter_action,
        clear_completed,
    }
}

// Store provider component
#[component]
pub fn TodoProvider(children: Children) -> impl IntoView {
    let store = create_todo_store();
    provide_context(store);
    
    children()
}

// Component using the store
#[component]
pub fn TodoApp() -> impl IntoView {
    view! {
        <TodoProvider>
            <TodoHeader />
            <TodoList />
            <TodoFooter />
        </TodoProvider>
    }
}

#[component]
pub fn TodoHeader() -> impl IntoView {
    let store = use_context::<TodoStore>()
        .expect("TodoStore should be provided");
    
    let (new_todo, set_new_todo) = create_signal(String::new());
    
    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let text = new_todo.get();
        if !text.trim().is_empty() {
            store.add_todo.dispatch(text);
            set_new_todo.set(String::new());
        }
    };
    
    view! {
        <header class="header">
            <h1>"todos"</h1>
            <form on:submit=handle_submit>
                <input
                    class="new-todo"
                    placeholder="What needs to be done?"
                    prop:value=new_todo
                    on:input=move |ev| set_new_todo.set(event_target_value(&ev))
                    autofocus
                />
            </form>
        </header>
    }
}

#[component]
pub fn TodoList() -> impl IntoView {
    let store = use_context::<TodoStore>()
        .expect("TodoStore should be provided");
    
    let filtered_todos = create_memo(move |_| {
        let todos = store.todos.get();
        let filter = store.filter.get();
        
        todos.into_iter().filter(|todo| match filter {
            TodoFilter::All => true,
            TodoFilter::Active => !todo.completed,
            TodoFilter::Completed => todo.completed,
        }).collect::<Vec<_>>()
    });
    
    view! {
        <section class="main">
            <ul class="todo-list">
                {move || filtered_todos.get()
                    .into_iter()
                    .map(|todo| view! {
                        <TodoItem todo />
                    })
                    .collect::<Vec<_>>()}
            </ul>
        </section>
    }
}

#[component]
pub fn TodoItem(todo: Todo) -> impl IntoView {
    let store = use_context::<TodoStore>()
        .expect("TodoStore should be provided");
    
    let (is_editing, set_is_editing) = create_signal(false);
    let (edit_text, set_edit_text) = create_signal(todo.text.clone());
    
    let handle_toggle = move |_| {
        store.toggle_todo.dispatch(todo.id);
    };
    
    let handle_delete = move |_| {
        store.delete_todo.dispatch(todo.id);
    };
    
    let handle_edit = move |_| {
        set_is_editing.set(true);
    };
    
    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let new_text = edit_text.get().trim().to_string();
        if new_text.is_empty() {
            store.delete_todo.dispatch(todo.id);
        } else {
            // In a real app, you'd have an edit action
            // For now, we'll just update locally
            set_is_editing.set(false);
        }
    };
    
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Escape" {
            set_edit_text.set(todo.text.clone());
            set_is_editing.set(false);
        }
    };
    
    view! {
        <li class="todo" class:completed=todo.completed class:editing=is_editing>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=todo.completed
                    on:change=handle_toggle
                />
                <label on:dblclick=handle_edit>{todo.text}</label>
                <button class="destroy" on:click=handle_delete></button>
            </div>
            {move || is_editing.get().then(|| view! {
                <form on:submit=handle_submit>
                    <input
                        class="edit"
                        prop:value=edit_text
                        on:input=move |ev| set_edit_text.set(event_target_value(&ev))
                        on:keydown=handle_keydown
                        on:blur=move |_| set_is_editing.set(false)
                        autofocus
                    />
                </form>
            })}
        </li>
    }
}

#[component]
pub fn TodoFooter() -> impl IntoView {
    let store = use_context::<TodoStore>()
        .expect("TodoStore should be provided");
    
    let todos = store.todos;
    let filter = store.filter;
    
    let total_count = create_memo(move |_| todos.get().len());
    let active_count = create_memo(move |_| {
        todos.get().iter().filter(|t| !t.completed).count()
    });
    let completed_count = create_memo(move |_| {
        todos.get().iter().filter(|t| t.completed).count()
    });
    
    let handle_clear_completed = move |_| {
        store.clear_completed.dispatch(());
    };
    
    view! {
        {move || (total_count.get() > 0).then(|| view! {
            <footer class="footer">
                <span class="todo-count">
                    <strong>{active_count}</strong>
                    {move || if active_count.get() == 1 { " item" } else { " items" }}
                    " left"
                </span>
                <ul class="filters">
                    <li>
                        <a 
                            href="#"
                            class=move || if matches!(filter.get(), TodoFilter::All) { "selected" } else { "" }
                            on:click=move |ev| {
                                ev.prevent_default();
                                store.set_filter.dispatch(TodoFilter::All);
                            }
                        >
                            "All"
                        </a>
                    </li>
                    <li>
                        <a 
                            href="#"
                            class=move || if matches!(filter.get(), TodoFilter::Active) { "selected" } else { "" }
                            on:click=move |ev| {
                                ev.prevent_default();
                                store.set_filter.dispatch(TodoFilter::Active);
                            }
                        >
                            "Active"
                        </a>
                    </li>
                    <li>
                        <a 
                            href="#"
                            class=move || if matches!(filter.get(), TodoFilter::Completed) { "selected" } else { "" }
                            on:click=move |ev| {
                                ev.prevent_default();
                                store.set_filter.dispatch(TodoFilter::Completed);
                            }
                        >
                            "Completed"
                        </a>
                    </li>
                </ul>
                {move || (completed_count.get() > 0).then(|| view! {
                    <button class="clear-completed" on:click=handle_clear_completed>
                        "Clear completed"
                    </button>
                })}
            </footer>
        })}
    }
}
```

## Data Flow Patterns

### Unidirectional Data Flow

Implement clean unidirectional data flow:

```rust
use leptos::*;

// Parent component managing state
#[component]
pub fn DataFlowExample() -> impl IntoView {
    let (items, set_items) = create_signal(vec![
        Item { id: 1, name: "Item 1".to_string(), quantity: 5 },
        Item { id: 2, name: "Item 2".to_string(), quantity: 3 },
    ]);
    
    let add_item = move |name: String| {
        set_items.update(|items| {
            let id = items.len() as u32 + 1;
            items.push(Item { id, name, quantity: 1 });
        });
    };
    
    let update_quantity = move |(id, quantity): (u32, u32)| {
        set_items.update(|items| {
            if let Some(item) = items.iter_mut().find(|i| i.id == id) {
                item.quantity = quantity;
            }
        });
    };
    
    let remove_item = move |id: u32| {
        set_items.update(|items| {
            items.retain(|i| i.id != id);
        });
    };
    
    view! {
        <div class="data-flow-example">
            <ItemList 
                items=items
                on_add=add_item
                on_update=update_quantity
                on_remove=remove_item
            />
        </div>
    }
}

#[derive(Clone, Debug)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub quantity: u32,
}

// Child component receiving data and callbacks
#[component]
pub fn ItemList(
    items: Signal<Vec<Item>>,
    on_add: Action<String, ()>,
    on_update: Action<(u32, u32), ()>,
    on_remove: Action<u32, ()>,
) -> impl IntoView {
    let (new_item_name, set_new_item_name) = create_signal(String::new());
    
    let handle_add = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let name = new_item_name.get().trim().to_string();
        if !name.is_empty() {
            on_add.dispatch(name);
            set_new_item_name.set(String::new());
        }
    };
    
    view! {
        <div class="item-list">
            <form on:submit=handle_add>
                <input
                    type="text"
                    placeholder="Add new item"
                    prop:value=new_item_name
                    on:input=move |ev| set_new_item_name.set(event_target_value(&ev))
                />
                <button type="submit">"Add"</button>
            </form>
            
            <ul>
                {move || items.get()
                    .into_iter()
                    .map(|item| view! {
                        <ItemRow 
                            item
                            on_update=on_update
                            on_remove=on_remove
                        />
                    })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

#[component]
pub fn ItemRow(
    item: Item,
    on_update: Action<(u32, u32), ()>,
    on_remove: Action<u32, ()>,
) -> impl IntoView {
    let (quantity, set_quantity) = create_signal(item.quantity);
    
    // Sync local quantity with item changes
    create_effect(move |_| {
        set_quantity.set(item.quantity);
    });
    
    let handle_quantity_change = move |ev| {
        let new_quantity = event_target_value(&ev).parse().unwrap_or(item.quantity);
        set_quantity.set(new_quantity);
        on_update.dispatch((item.id, new_quantity));
    };
    
    let handle_remove = move |_| {
        on_remove.dispatch(item.id);
    };
    
    view! {
        <li class="item-row">
            <span class="item-name">{item.name}</span>
            <input
                type="number"
                min="1"
                prop:value=quantity
                on:input=handle_quantity_change
            />
            <button on:click=handle_remove>"Remove"</button>
        </li>
    }
}
```

### Lifting State Up

Move state to the lowest common ancestor:

```rust
use leptos::*;

// State lifted to common ancestor
#[component]
pub fn LiftedStateExample() -> impl IntoView {
    let (left_count, set_left_count) = create_signal(0);
    let (right_count, set_right_count) = create_signal(0);
    
    let total = create_memo(move |_| left_count.get() + right_count.get());
    
    view! {
        <div class="lifted-state">
            <Counter 
                count=left_count
                set_count=set_left_count
                label="Left Counter"
            />
            <Counter 
                count=right_count
                set_count=set_right_count
                label="Right Counter"
            />
            <div class="total">
                "Total: " {total}
            </div>
        </div>
    }
}

#[component]
pub fn Counter(
    count: Signal<i32>,
    set_count: WriteSignal<i32>,
    label: String,
) -> impl IntoView {
    let increment = move |_| set_count.update(|c| *c + 1);
    let decrement = move |_| set_count.update(|c| *c - 1);
    
    view! {
        <div class="counter">
            <h3>{label}</h3>
            <div class="counter-controls">
                <button on:click=decrement>"-"</button>
                <span class="count">{count}</span>
                <button on:click=increment>"+"</button>
            </div>
        </div>
    }
}
```

## Performance Optimization

### Memoization

Use memoization to avoid unnecessary computations:

```rust
use leptos::*;

// Memoized expensive computations
#[component]
pub fn ExpensiveList() -> impl IntoView {
    let (items, set_items) = create_signal((0..1000).collect::<Vec<_>>());
    let (search_term, set_search_term) = create_signal(String::new());
    
    // Memoize filtered items
    let filtered_items = create_memo(move |_| {
        let term = search_term.get().to_lowercase();
        if term.is_empty() {
            items.get()
        } else {
            items.get().into_iter()
                .filter(|&item| item.to_string().contains(&term))
                .collect()
        }
    });
    
    // Memoize stats
    let stats = create_memo(move |_| {
        let filtered = filtered_items.get();
        let total = items.get().len();
        let visible = filtered.len();
        (total, visible)
    });
    
    view! {
        <div class="expensive-list">
            <div class="search">
                <input
                    type="text"
                    placeholder="Search..."
                    on:input=move |ev| set_search_term.set(event_target_value(&ev))
                />
            </div>
            
            <div class="stats">
                "Showing " {move || stats.get().1} " of " {move || stats.get().0} " items"
            </div>
            
            <ul class="item-list">
                {move || filtered_items.get()
                    .into_iter()
                    .map(|item| view! { <li>{item}</li> })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
```

### Selective Re-rendering

Use signals to control when components re-render:

```rust
use leptos::*;

// Selective re-rendering
#[component]
pub fn SelectiveRerender() -> impl IntoView {
    let (data, set_data) = create_signal(LargeData {
        items: (0..100).collect(),
        metadata: "Some metadata".to_string(),
        timestamp: chrono::Utc::now(),
    });
    
    let (filter, set_filter) = create_signal(String::new());
    
    // Only re-compute when filter changes
    let filtered_items = create_memo(move |_| {
        let filter_val = filter.get();
        if filter_val.is_empty() {
            data.get().items
        } else {
            data.get().items.into_iter()
                .filter(|&item| item.to_string().contains(&filter_val))
                .collect()
        }
    });
    
    view! {
        <div class="selective-rerender">
            <button on:click=move |_| {
                set_data.update(|d| {
                    d.timestamp = chrono::Utc::now();
                });
            }>
                "Update timestamp (doesn't affect filter)"
            </button>
            
            <input
                type="text"
                placeholder="Filter..."
                on:input=move |ev| set_filter.set(event_target_value(&ev))
            />
            
            <p>"Last updated: " {move || data.get().timestamp.to_string()}</p>
            
            <ul>
                {move || filtered_items.get()
                    .into_iter()
                    .map(|item| view! { <li>{item}</li> })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

#[derive(Clone, Debug)]
pub struct LargeData {
    pub items: Vec<i32>,
    pub metadata: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
```

## Testing Data Flow

### Testing Props and Callbacks

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;
    
    #[test]
    fn test_counter_component() {
        let mut app = create_runtime();
        
        let (count, set_count) = create_signal(0);
        
        // Test that component renders correctly
        let view = Counter(count.into(), set_count, "Test Counter".to_string());
        
        // In a real test, you'd check the rendered output
        // This is simplified for demonstration
        
        app.dispose();
    }
    
    #[test]
    fn test_context_provided() {
        let mut app = create_runtime();
        
        // Test that context is provided
        let result = use_context::<UserContext>();
        assert!(result.is_none()); // Should be None without provider
        
        app.dispose();
    }
    
    #[test]
    fn test_memo_updates() {
        let mut app = create_runtime();
        
        let (a, set_a) = create_signal(1);
        let (b, set_b) = create_signal(2);
        let sum = create_memo(move |_| a.get() + b.get());
        
        assert_eq!(sum.get(), 3);
        
        set_a.set(5);
        assert_eq!(sum.get(), 7);
        
        set_b.set(10);
        assert_eq!(sum.get(), 15);
        
        app.dispose();
    }
}
```

## Summary

Leptos provides several powerful patterns for managing data flow in your applications:

1. **Props**: Type-safe data passing from parent to child components
2. **Context**: Global state sharing across component trees
3. **Stores**: Structured state management for complex applications
4. **Signals**: Reactive primitives for fine-grained reactivity
5. **Actions**: Async operations with loading states and error handling

Key principles for effective data flow:
- Keep data flow unidirectional when possible
- Lift state up to the lowest common ancestor
- Use context sparingly and for truly global concerns
- Memoize expensive computations
- Test your data flow patterns thoroughly

---

**Next:** [07: Performance](07-performance.md) - Learn optimization techniques and performance best practices.