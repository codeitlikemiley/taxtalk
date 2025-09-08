# Todo App Example

## Overview

This example demonstrates a complete Todo application built with Leptos, showcasing:

- Reactive state management with signals
- Component composition and props
- Event handling for user interactions
- Local storage persistence
- Filtering and searching functionality
- Clean component architecture

## Project Structure

```
src/
├── main.rs              # Application entry point
├── components/
│   ├── todo_item.rs     # Individual todo item component
│   ├── todo_list.rs     # Todo list container
│   ├── todo_input.rs    # New todo input form
│   ├── todo_filters.rs  # Filter controls
│   └── todo_stats.rs    # Statistics display
├── models/
│   └── todo.rs          # Todo data structures
└── utils/
    └── storage.rs       # Local storage utilities
```

## Core Data Structures

### Todo Model

```rust
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}
```

## Main Application Component

```rust
#[component]
pub fn TodoApp() -> impl IntoView {
    // Application state
    let (todos, set_todos) = create_signal(Vec::<Todo>::new());
    let (filter, set_filter) = create_signal(Filter::All);
    let (next_id, set_next_id) = create_signal(1);

    // Load todos from localStorage on mount
    create_effect(move |_| {
        if let Ok(stored) = storage::load_todos() {
            set_todos(stored.todos);
            set_next_id(stored.next_id);
        }
    });

    // Save todos to localStorage whenever they change
    create_effect(move |_| {
        let current_todos = todos();
        let current_next_id = next_id();
        storage::save_todos(&current_todos, current_next_id);
    });

    // Computed values
    let filtered_todos = create_memo(move |_| {
        let current_filter = filter();
        todos().into_iter()
            .filter(|todo| match current_filter {
                Filter::All => true,
                Filter::Active => !todo.completed,
                Filter::Completed => todo.completed,
            })
            .collect::<Vec<_>>()
    });

    let active_count = create_memo(move |_| {
        todos().iter().filter(|todo| !todo.completed).count()
    });

    let completed_count = create_memo(move |_| {
        todos().iter().filter(|todo| todo.completed).count()
    });

    // Event handlers
    let add_todo = move |title: String| {
        let new_todo = Todo {
            id: next_id(),
            title: title.trim().to_string(),
            completed: false,
            created_at: chrono::Utc::now(),
        };

        set_todos.update(|todos| todos.push(new_todo));
        set_next_id.update(|id| *id += 1);
    };

    let toggle_todo = move |id: u32| {
        set_todos.update(|todos| {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = !todo.completed;
            }
        });
    };

    let delete_todo = move |id: u32| {
        set_todos.update(|todos| {
            todos.retain(|todo| todo.id != id);
        });
    };

    let clear_completed = move |_| {
        set_todos.update(|todos| {
            todos.retain(|todo| !todo.completed);
        });
    };

    let toggle_all = move |_| {
        let all_completed = active_count() == 0;
        set_todos.update(|todos| {
            for todo in todos {
                todo.completed = !all_completed;
            }
        });
    };

    view! {
        <div class="todo-app">
            <header class="header">
                <h1>"todos"</h1>
                <TodoInput on_add=add_todo />
            </header>

            <section class="main" class:hidden=move || todos().is_empty()>
                <input
                    id="toggle-all"
                    class="toggle-all"
                    type="checkbox"
                    prop:checked=move || active_count() == 0
                    on:input=toggle_all
                />
                <label for="toggle-all">"Mark all as complete"</label>

                <TodoList
                    todos=filtered_todos
                    on_toggle=toggle_todo
                    on_delete=delete_todo
                />
            </section>

            <footer class="footer" class:hidden=move || todos().is_empty()>
                <TodoStats
                    active_count=active_count
                    completed_count=completed_count
                />

                <TodoFilters
                    current_filter=filter
                    on_filter_change=set_filter
                />

                <button
                    class="clear-completed"
                    class:hidden=move || completed_count() == 0
                    on:click=clear_completed
                >
                    "Clear completed"
                </button>
            </footer>
        </div>
    }
}
```

## Todo Input Component

```rust
#[component]
pub fn TodoInput(
    #[prop(into)] on_add: Callback<String>
) -> impl IntoView {
    let (input_value, set_input_value) = create_signal(String::new());

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        let value = input_value().trim().to_string();
        if !value.is_empty() {
            on_add(value.clone());
            set_input_value(String::new());
        }
    };

    let handle_input = move |ev| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Some(input) = target {
            set_input_value(input.value());
        }
    };

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Escape" {
            set_input_value(String::new());
        }
    };

    view! {
        <form on:submit=handle_submit>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                prop:value=input_value
                on:input=handle_input
                on:keydown=handle_keydown
                autofocus
            />
        </form>
    }
}
```

## Todo List Component

```rust
#[component]
pub fn TodoList(
    #[prop(into)] todos: MaybeSignal<Vec<Todo>>,
    #[prop(into)] on_toggle: Callback<u32>,
    #[prop(into)] on_delete: Callback<u32>,
) -> impl IntoView {
    view! {
        <ul class="todo-list">
            <For
                each=move || todos()
                key=|todo| todo.id
                children=move |todo| view! {
                    <TodoItem
                        todo
                        on_toggle=on_toggle
                        on_delete=on_delete
                    />
                }
            />
        </ul>
    }
}
```

## Todo Item Component

```rust
#[component]
pub fn TodoItem(
    #[prop(into)] todo: MaybeSignal<Todo>,
    #[prop(into)] on_toggle: Callback<u32>,
    #[prop(into)] on_delete: Callback<u32>,
) -> impl IntoView {
    let (editing, set_editing) = create_signal(false);
    let (edit_value, set_edit_value) = create_signal(String::new());

    let current_todo = move || todo();

    let handle_toggle = move |_| {
        on_toggle(current_todo().id);
    };

    let handle_delete = move |_| {
        on_delete(current_todo().id);
    };

    let start_editing = move |_| {
        set_editing(true);
        set_edit_value(current_todo().title);
    };

    let cancel_editing = move |_| {
        set_editing(false);
        set_edit_value(String::new());
    };

    let save_edit = move |_| {
        let new_title = edit_value().trim().to_string();
        if !new_title.is_empty() {
            // In a real app, you'd have an on_edit callback
            // For now, we'll just stop editing
            set_editing(false);
        }
    };

    let handle_edit_keydown = move |ev: web_sys::KeyboardEvent| {
        match ev.key().as_str() {
            "Enter" => save_edit(()),
            "Escape" => cancel_editing(()),
            _ => {}
        }
    };

    view! {
        <li class=move || {
            let mut classes = vec!["todo"];
            if current_todo().completed {
                classes.push("completed");
            }
            if editing() {
                classes.push("editing");
            }
            classes.join(" ")
        }>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=move || current_todo().completed
                    on:input=handle_toggle
                />
                <label on:dblclick=start_editing>
                    {move || current_todo().title}
                </label>
                <button class="destroy" on:click=handle_delete></button>
            </div>

            <Show when=editing>
                <input
                    class="edit"
                    prop:value=edit_value
                    on:input=move |ev| {
                        let target = event_target::<web_sys::HtmlInputElement>(&ev);
                        if let Some(input) = target {
                            set_edit_value(input.value());
                        }
                    }
                    on:keydown=handle_edit_keydown
                    on:blur=save_edit
                />
            </Show>
        </li>
    }
}
```

## Todo Filters Component

```rust
#[component]
pub fn TodoFilters(
    #[prop(into)] current_filter: MaybeSignal<Filter>,
    #[prop(into)] on_filter_change: WriteSignal<Filter>,
) -> impl IntoView {
    let filters = vec![
        ("All", Filter::All),
        ("Active", Filter::Active),
        ("Completed", Filter::Completed),
    ];

    view! {
        <ul class="filters">
            <For
                each=move || filters.clone()
                key=|(_, filter)| format!("{:?}", filter)
                children=move |(label, filter)| {
                    let is_selected = move || current_filter() == filter;
                    view! {
                        <li>
                            <a
                                href="#"
                                class=move || if is_selected() { "selected" } else { "" }
                                on:click=move |ev| {
                                    ev.prevent_default();
                                    on_filter_change(filter.clone());
                                }
                            >
                                {label}
                            </a>
                        </li>
                    }
                }
            />
        </ul>
    }
}
```

## Todo Stats Component

```rust
#[component]
pub fn TodoStats(
    #[prop(into)] active_count: MaybeSignal<usize>,
    #[prop(into)] completed_count: MaybeSignal<usize>,
) -> impl IntoView {
    let item_text = move || {
        let count = active_count();
        match count {
            0 => "No items left".to_string(),
            1 => "1 item left".to_string(),
            n => format!("{} items left", n),
        }
    };

    view! {
        <span class="todo-count">
            <strong>{move || active_count()}</strong>
            " " {item_text}
        </span>
    }
}
```

## Local Storage Utilities

```rust
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StoredData {
    pub todos: Vec<super::Todo>,
    pub next_id: u32,
}

pub fn save_todos(todos: &[super::Todo], next_id: u32) {
    let data = StoredData {
        todos: todos.to_vec(),
        next_id,
    };

    if let Ok(json) = serde_json::to_string(&data) {
        if let Ok(Some(storage)) = window().local_storage() {
            let _ = storage.set_item("leptos-todos", &json);
        }
    }
}

pub fn load_todos() -> Result<StoredData, Box<dyn std::error::Error>> {
    let storage = window().local_storage()?;
    let storage = storage.ok_or("No local storage available")?;

    let json = storage.get_item("leptos-todos")?;
    let json = json.ok_or("No stored todos found")?;

    let data: StoredData = serde_json::from_str(&json)?;
    Ok(data)
}
```

## CSS Styling

```css
/* Todo App Styles */
.todo-app {
    background: #fff;
    margin: 130px 0 40px 0;
    position: relative;
    box-shadow: 0 2px 4px 0 rgba(0, 0, 0, 0.2), 0 25px 50px 0 rgba(0, 0, 0, 0.1);
}

.todo-app input::-webkit-input-placeholder {
    font-style: italic;
    font-weight: 300;
    color: #e6e6e6;
}

.todo-app input::-moz-placeholder {
    font-style: italic;
    font-weight: 300;
    color: #e6e6e6;
}

.todo-app input::input-placeholder {
    font-style: italic;
    font-weight: 300;
    color: #e6e6e6;
}

.todo-app h1 {
    position: absolute;
    top: -155px;
    width: 100%;
    font-size: 100px;
    font-weight: 100;
    text-align: center;
    color: rgba(175, 47, 47, 0.15);
    -webkit-text-rendering: optimizeLegibility;
    -moz-text-rendering: optimizeLegibility;
    text-rendering: optimizeLegibility;
}

/* Header */
.header {
    padding-top: 40px;
    border-radius: inherit;
}

.new-todo,
.edit {
    position: relative;
    margin: 0;
    width: 100%;
    font-size: 24px;
    font-family: inherit;
    font-weight: inherit;
    line-height: 1.4em;
    border: 0;
    color: inherit;
    padding: 6px;
    border: 1px solid #999;
    box-shadow: inset 0 -1px 5px 0 rgba(0, 0, 0, 0.2);
    box-sizing: border-box;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
}

.new-todo {
    padding: 16px 16px 16px 60px;
    border: none;
    background: rgba(0, 0, 0, 0.003);
    box-shadow: inset 0 -2px 1px rgba(0,0,0,0.03);
}

/* Main section */
.main {
    position: relative;
    z-index: 2;
    border-top: 1px solid #e6e6e6;
}

.toggle-all {
    width: 1px;
    height: 1px;
    border: none;
    opacity: 0;
    position: absolute;
    right: 100%;
    bottom: 100%;
}

.toggle-all + label {
    width: 60px;
    height: 34px;
    font-size: 0;
    position: absolute;
    top: -52px;
    left: -13px;
    -webkit-transform: rotate(90deg);
    transform: rotate(90deg);
}

.toggle-all + label:before {
    content: '❯';
    font-size: 22px;
    color: #e6e6e6;
    padding: 10px 27px 10px 27px;
}

.toggle-all:checked + label:before {
    color: #737373;
}

/* Todo list */
.todo-list {
    margin: 0;
    padding: 0;
    list-style: none;
}

.todo-list li {
    position: relative;
    font-size: 24px;
    border-bottom: 1px solid #ededed;
}

.todo-list li:last-child {
    border-bottom: none;
}

.todo-list li.editing {
    border-bottom: none;
    padding: 0;
}

.todo-list li.editing .edit {
    display: block;
    width: 506px;
    padding: 12px 16px;
    margin: 0 0 0 43px;
}

.todo-list li.editing .view {
    display: none;
}

.todo-list li .toggle {
    text-align: center;
    width: 40px;
    height: auto;
    position: absolute;
    top: 0;
    bottom: 0;
    margin: auto 0;
    border: none;
    -webkit-appearance: none;
    appearance: none;
}

.todo-list li .toggle {
    opacity: 0;
}

.todo-list li .toggle + label {
    background-image: url('data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23ededed%22%20stroke-width%3D%223%22/%3E%3C/svg%3E');
    background-repeat: no-repeat;
    background-position: center left;
}

.todo-list li .toggle:checked + label {
    background-image: url('data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23bddad5%22%20stroke-width%3D%223%22/%3E%3Cpath%20fill%3D%22%235dc2af%22%20d%3D%22m72%2025l-12%2017-13-10z%22/%3E%3C/svg%3E');
}

.todo-list li label {
    word-break: break-all;
    padding: 15px 15px 15px 60px;
    display: block;
    line-height: 1.2;
    transition: color 0.4s;
}

.todo-list li.completed label {
    color: #d9d9d9;
    text-decoration: line-through;
}

.todo-list li .destroy {
    display: none;
    position: absolute;
    top: 0;
    right: 10px;
    bottom: 0;
    width: 40px;
    height: 40px;
    margin: auto 0;
    font-size: 30px;
    color: #cc9a9a;
    margin-bottom: 11px;
    transition: color 0.2s ease-out;
}

.todo-list li .destroy:hover {
    color: #af5b5e;
}

.todo-list li .destroy:after {
    content: "×";
}

.todo-list li:hover .destroy {
    display: block;
}

.todo-list li .edit {
    display: none;
}

.todo-list li.editing:last-child {
    margin-bottom: -1px;
}

/* Footer */
.footer {
    color: #777;
    padding: 10px 15px;
    height: 20px;
    text-align: center;
    border-top: 1px solid #e6e6e6;
}

.footer:before {
    content: '';
    position: absolute;
    right: 0;
    bottom: 0;
    left: 0;
    height: 50px;
    overflow: hidden;
    box-shadow: 0 1px 1px rgba(0, 0, 0, 0.2), 0 8px 0 -3px #f6f6f6, 0 9px 1px -3px rgba(0, 0, 0, 0.2), 0 16px 0 -6px #f6f6f6, 0 17px 2px -6px rgba(0, 0, 0, 0.2);
}

.todo-count {
    float: left;
    text-align: left;
}

.todo-count strong {
    font-weight: 300;
}

.filters {
    margin: 0;
    padding: 0;
    list-style: none;
    position: absolute;
    right: 0;
    left: 0;
}

.filters li {
    display: inline;
}

.filters li a {
    color: inherit;
    margin: 3px;
    padding: 3px 7px;
    text-decoration: none;
    border: 1px solid transparent;
    border-radius: 3px;
}

.filters li a:hover {
    border-color: rgba(175, 47, 47, 0.1);
}

.filters li a.selected {
    border-color: rgba(175, 47, 47, 0.2);
}

.clear-completed,
html .clear-completed:active {
    float: right;
    position: relative;
    line-height: 20px;
    text-decoration: none;
    cursor: pointer;
}

.clear-completed:hover {
    text-decoration: underline;
}

/* Responsive */
@media screen and (-webkit-min-device-pixel-ratio:0) {
    .toggle-all,
    .todo-list li .toggle {
        background: none;
    }

    .todo-list li .toggle {
        height: 40px;
    }
}

@media (max-width: 430px) {
    .footer {
        height: 50px;
    }

    .filters {
        bottom: 10px;
    }
}
```

## Key Features Demonstrated

### 1. Reactive State Management
- **Signals**: `create_signal` for local state
- **Memos**: `create_memo` for computed values
- **Effects**: `create_effect` for side effects

### 2. Component Composition
- **Props**: Passing data between components
- **Callbacks**: Event communication between components
- **Children**: Flexible component composition

### 3. Event Handling
- **DOM Events**: Click, input, keydown, submit
- **Custom Callbacks**: Component communication
- **Event Delegation**: Efficient event handling

### 4. Local Storage Integration
- **Persistence**: Save/load application state
- **Serialization**: JSON handling with serde
- **Error Handling**: Graceful storage failures

### 5. Filtering and Search
- **Dynamic Filtering**: Real-time list updates
- **Multiple Views**: All, Active, Completed
- **Performance**: Efficient filtering with memos

### 6. User Experience
- **Optimistic Updates**: Immediate UI feedback
- **Keyboard Shortcuts**: Enter/Escape handling
- **Accessibility**: Proper labels and focus management

## Performance Optimizations

### 1. Memoization
```rust
let filtered_todos = create_memo(move |_| {
    // Only recalculates when todos or filter changes
    todos().into_iter()
        .filter(|todo| matches_filter(todo, filter()))
        .collect::<Vec<_>>()
});
```

### 2. Efficient Updates
```rust
let toggle_todo = move |id: u32| {
    set_todos.update(|todos| {
        // Only updates the specific todo
        if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
    });
};
```

### 3. Batching Updates
```rust
let clear_completed = move |_| {
    set_todos.update(|todos| {
        // Single update for multiple changes
        todos.retain(|todo| !todo.completed);
    });
};
```

## Testing the Application

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_add_todo() {
        let (todos, set_todos) = create_signal(Vec::new());
        let (next_id, set_next_id) = create_signal(1);

        let add_todo = move |title: String| {
            let new_todo = Todo {
                id: next_id(),
                title,
                completed: false,
                created_at: chrono::Utc::now(),
            };
            set_todos.update(|todos| todos.push(new_todo));
            set_next_id.update(|id| *id += 1);
        };

        add_todo("Test todo".to_string());

        assert_eq!(todos().len(), 1);
        assert_eq!(todos()[0].title, "Test todo");
        assert_eq!(next_id(), 2);
    }

    #[test]
    fn test_toggle_todo() {
        let todo = Todo {
            id: 1,
            title: "Test".to_string(),
            completed: false,
            created_at: chrono::Utc::now(),
        };

        let (todos, set_todos) = create_signal(vec![todo]);

        let toggle_todo = move |id: u32| {
            set_todos.update(|todos| {
                if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                    todo.completed = !todo.completed;
                }
            });
        };

        toggle_todo(1);
        assert!(todos()[0].completed);

        toggle_todo(1);
        assert!(!todos()[0].completed);
    }
}
```

## Running the Application

1. **Setup**: Create a new Leptos project
2. **Copy Files**: Add the components and utilities
3. **Run**: Use `trunk serve` to start the development server
4. **Build**: Use `trunk build --release` for production

This Todo application demonstrates the full power of Leptos for building interactive, reactive web applications with excellent performance and developer experience.