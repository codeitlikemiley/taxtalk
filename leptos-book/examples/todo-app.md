# Todo App Example

## Overview

This example demonstrates a complete todo application that showcases advanced Leptos patterns including complex state management, component composition, local storage persistence, and form handling.

## Complete Code

```rust
use leptos::*;
use serde::{Deserialize, Serialize};
use web_sys::Storage;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoItem {
    id: u32,
    text: String,
    completed: bool,
    created_at: String,
}

#[component]
fn TodoApp(cx: Scope) -> impl IntoView {
    // Main state
    let (todos, set_todos) = create_signal(cx, Vec::<TodoItem>::new());
    let (next_id, set_next_id) = create_signal(cx, 1u32);

    // Load from localStorage on mount
    create_effect(cx, move |_| {
        if let Ok(Some(storage)) = window().local_storage() {
            if let Ok(Some(data)) = storage.get_item("todos") {
                if let Ok(parsed_todos) = serde_json::from_str::<Vec<TodoItem>>(&data) {
                    set_todos.set(parsed_todos);
                    // Update next_id based on existing todos
                    if let Some(max_id) = parsed_todos.iter().map(|t| t.id).max() {
                        set_next_id.set(max_id + 1);
                    }
                }
            }
        }
    });

    // Save to localStorage whenever todos change
    create_effect(cx, move |_| {
        let todos_json = serde_json::to_string(&todos.get()).unwrap_or_default();
        if let Ok(Some(storage)) = window().local_storage() {
            let _ = storage.set_item("todos", &todos_json);
        }
    });

    // Computed values
    let total_count = create_memo(cx, move |_| todos.get().len());
    let completed_count = create_memo(cx, move |_| {
        todos.get().iter().filter(|t| t.completed).count()
    });
    let active_count = create_memo(cx, move |_| total_count.get() - completed_count.get());

    // Actions
    let add_todo = move |text: String| {
        if !text.trim().is_empty() {
            let new_todo = TodoItem {
                id: next_id.get(),
                text: text.trim().to_string(),
                completed: false,
                created_at: js_sys::Date::now().to_string(),
            };

            set_todos.update(|todos| todos.push(new_todo));
            set_next_id.update(|id| *id += 1);
        }
    };

    let toggle_todo = move |id: u32| {
        set_todos.update(|todos| {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.completed = !todo.completed;
            }
        });
    };

    let delete_todo = move |id: u32| {
        set_todos.update(|todos| todos.retain(|t| t.id != id));
    };

    let clear_completed = move |_| {
        set_todos.update(|todos| todos.retain(|t| !t.completed));
    };

    let toggle_all = move |_| {
        let all_completed = active_count.get() == 0;
        set_todos.update(|todos| {
            for todo in todos {
                todo.completed = !all_completed;
            }
        });
    };

    view! { cx,
        div(class="todo-app") {
            header(class="header") {
                h1 { "todos" }
                TodoInput(add_todo=add_todo)
            }

            (if total_count.get() > 0 {
                view! { cx,
                    section(class="main") {
                        input(
                            id="toggle-all",
                            class="toggle-all",
                            type="checkbox",
                            checked=active_count.get() == 0,
                            on:change=toggle_all
                        )
                        label(for="toggle-all") { "Mark all as complete" }

                        ul(class="todo-list") {
                            (todos.get().into_iter().map(|todo| {
                                view! { cx,
                                    TodoItemView(
                                        todo=todo,
                                        toggle=Callback::new(cx, move |_| toggle_todo(todo.id)),
                                        delete=Callback::new(cx, move |_| delete_todo(todo.id))
                                    )
                                }
                            }).collect::<Vec<_>>())
                        }
                    }

                    footer(class="footer") {
                        span(class="todo-count") {
                            strong { (active_count.get().to_string()) }
                            " item" (if active_count.get() == 1 { "" } else { "s" }) " left"
                        }

                        (if completed_count.get() > 0 {
                            view! { cx,
                                button(
                                    class="clear-completed",
                                    on:click=clear_completed
                                ) {
                                    "Clear completed"
                                }
                            }
                        } else {
                            view! { cx, }
                        })
                    }
                }.into_view(cx)
            } else {
                view! { cx, }.into_view(cx)
            })
        }
    }
}

#[component]
fn TodoInput(cx: Scope, add_todo: Callback<String>) -> impl IntoView {
    let (input_value, set_input_value) = create_signal(cx, String::new());

    let on_submit = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            ev.prevent_default();
            let value = input_value.get();
            if !value.trim().is_empty() {
                add_todo.call(value.clone());
                set_input_value.set(String::new());
            }
        }
    };

    view! { cx,
        input(
            class="new-todo",
            placeholder="What needs to be done?",
            value=input_value,
            on:input=move |ev| set_input_value.set(event_target_value(&ev)),
            on:keydown=on_submit,
            autofocus=true
        )
    }
}

#[component]
fn TodoItemView(
    cx: Scope,
    todo: TodoItem,
    toggle: Callback<()>,
    delete: Callback<()>
) -> impl IntoView {
    let (editing, set_editing) = create_signal(cx, false);
    let (edit_value, set_edit_value) = create_signal(cx, todo.text.clone());

    let start_edit = move |_| {
        set_editing.set(true);
        set_edit_value.set(todo.text.clone());
    };

    let save_edit = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            ev.prevent_default();
            let new_text = edit_value.get().trim().to_string();
            if !new_text.is_empty() {
                // In a real app, you'd update the todo here
                set_editing.set(false);
            }
        } else if ev.key() == "Escape" {
            set_edit_value.set(todo.text.clone());
            set_editing.set(false);
        }
    };

    let cancel_edit = move |_| {
        set_edit_value.set(todo.text.clone());
        set_editing.set(false);
    };

    view! { cx,
        li(class=move || {
            let mut classes = vec!["todo"];
            if todo.completed {
                classes.push("completed");
            }
            if editing.get() {
                classes.push("editing");
            }
            classes.join(" ")
        }) {
            div(class="view") {
                input(
                    class="toggle",
                    type="checkbox",
                    checked=todo.completed,
                    on:change=move |_| toggle.call(())
                )
                label(on:dblclick=start_edit) { (todo.text) }
                button(
                    class="destroy",
                    on:click=move |_| delete.call(())
                )
            }

            (if editing.get() {
                view! { cx,
                    input(
                        class="edit",
                        value=edit_value,
                        on:input=move |ev| set_edit_value.set(event_target_value(&ev)),
                        on:keydown=save_edit,
                        on:blur=cancel_edit
                    )
                }
            } else {
                view! { cx, }
            })
        }
    }
}

fn main() {
    mount_to_body(|| view! { <TodoApp/> })
}
```

## Key Concepts Demonstrated

### 1. Complex State Management
```rust
let (todos, set_todos) = create_signal(cx, Vec::<TodoItem>::new());
let (next_id, set_next_id) = create_signal(cx, 1u32);
```
- Managing multiple related pieces of state
- Using signals for complex data structures

### 2. Local Storage Persistence
```rust
create_effect(cx, move |_| {
    let todos_json = serde_json::to_string(&todos.get()).unwrap_or_default();
    if let Ok(Some(storage)) = window().local_storage() {
        let _ = storage.set_item("todos", &todos_json);
    }
});
```
- Automatic persistence to localStorage
- Loading state on component mount
- JSON serialization/deserialization

### 3. Computed Values (Memos)
```rust
let total_count = create_memo(cx, move |_| todos.get().len());
let completed_count = create_memo(cx, move |_| {
    todos.get().iter().filter(|t| t.completed).count()
});
let active_count = create_memo(cx, move |_| total_count.get() - completed_count.get());
```
- Derived state that automatically updates
- Complex computations based on multiple signals

### 4. Component Communication
```rust
#[component]
fn TodoInput(cx: Scope, add_todo: Callback<String>) -> impl IntoView {
    // ...
    let on_submit = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            // ...
            add_todo.call(value.clone());
        }
    };
}
```
- Using Callbacks for parent-child communication
- Passing functions as props

### 5. Conditional Rendering
```rust
(if total_count.get() > 0 {
    view! { cx,
        section(class="main") {
            // Main content
        }
    }.into_view(cx)
} else {
    view! { cx, }.into_view(cx)
})
```
- Dynamic UI based on state
- Converting views to IntoView for conditional rendering

### 6. List Rendering with Keys
```rust
ul(class="todo-list") {
    (todos.get().into_iter().map(|todo| {
        view! { cx,
            TodoItemView(
                todo=todo,
                toggle=Callback::new(cx, move |_| toggle_todo(todo.id)),
                delete=Callback::new(cx, move |_| delete_todo(todo.id))
            )
        }
    }).collect::<Vec<_>>())
}
```
- Rendering dynamic lists
- Passing individual items to child components

### 7. Event Handling
```rust
input(
    on:input=move |ev| set_input_value.set(event_target_value(&ev)),
    on:keydown=on_submit,
)
```
- Multiple event handlers on the same element
- Keyboard event handling
- Form input handling

### 8. Dynamic Classes
```rust
li(class=move || {
    let mut classes = vec!["todo"];
    if todo.completed {
        classes.push("completed");
    }
    if editing.get() {
        classes.push("editing");
    }
    classes.join(" ")
})
```
- Reactive CSS classes
- Multiple conditional classes

## Advanced Patterns

### State Updates with Closures
```rust
let toggle_todo = move |id: u32| {
    set_todos.update(|todos| {
        if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
    });
};
```
- Updating specific items in a collection
- Using closures to modify state immutably

### Effect Dependencies
```rust
create_effect(cx, move |_| {
    // This effect runs whenever todos changes
    let todos_json = serde_json::to_string(&todos.get()).unwrap_or_default();
    // Save to localStorage
});
```
- Effects that react to signal changes
- Side effects like saving to storage

### Component State Isolation
```rust
#[component]
fn TodoItemView(cx: Scope, /* ... */) -> impl IntoView {
    let (editing, set_editing) = create_signal(cx, false);
    let (edit_value, set_edit_value) = create_signal(cx, todo.text.clone());
    // Local state for editing
}
```
- Each component instance has its own local state
- Isolation between different todo items

## Variations

### Todo App with Categories

```rust
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoItem {
    id: u32,
    text: String,
    completed: bool,
    category: String,
    priority: Priority,
    created_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[component]
fn TodoAppWithCategories(cx: Scope) -> impl IntoView {
    let (todos, set_todos) = create_signal(cx, Vec::<TodoItem>::new());
    let (selected_category, set_selected_category) = create_signal(cx, "all".to_string());

    let filtered_todos = create_memo(cx, move |_| {
        let category = selected_category.get();
        todos.get().into_iter().filter(|todo| {
            category == "all" || todo.category == category
        }).collect::<Vec<_>>()
    });

    // ... rest of implementation
}
```

### Todo App with Drag and Drop

```rust
#[component]
fn DraggableTodoItem(cx: Scope, todo: TodoItem, /* ... */) -> impl IntoView {
    let (is_dragging, set_is_dragging) = create_signal(cx, false);

    view! { cx,
        li(
            class=move || {
                let mut classes = vec!["todo"];
                if is_dragging.get() {
                    classes.push("dragging");
                }
                classes.join(" ")
            },
            draggable=true,
            on:dragstart=move |_| set_is_dragging.set(true),
            on:dragend=move |_| set_is_dragging.set(false),
            // ... drag and drop handlers
        ) {
            // ... todo content
        }
    }
}
```

## Best Practices

### 1. State Structure
```rust
// ✅ Good: Flat state structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoItem {
    id: u32,
    text: String,
    completed: bool,
    created_at: String,
}

// ❌ Avoid: Deeply nested state
#[derive(Clone, Debug)]
pub struct AppState {
    todos: Vec<TodoItem>,
    ui: UiState,
}
```

### 2. Component Size
```rust
// ✅ Good: Break down large components
#[component]
fn TodoApp(cx: Scope) -> impl IntoView {
    view! { cx,
        <TodoHeader/>
        <TodoList/>
        <TodoFooter/>
    }
}

// ❌ Avoid: Monolithic components
#[component]
fn HugeTodoApp(cx: Scope) -> impl IntoView {
    // 500+ lines of JSX
}
```

### 3. Performance Optimization
```rust
// ✅ Good: Use Keyed for dynamic lists
Keyed(
    iterable=todos,
    key=|todo| todo.id,
    view=render_todo
)

// ✅ Good: Memoize expensive computations
let expensive_calc = create_memo(cx, move |_| {
    complex_calculation(todos.get())
});
```

### 4. Error Handling
```rust
// ✅ Good: Handle localStorage errors gracefully
create_effect(cx, move |_| {
    let todos_json = serde_json::to_string(&todos.get()).unwrap_or_default();
    if let Ok(Some(storage)) = window().local_storage() {
        let _ = storage.set_item("todos", &todos_json); // Ignore errors
    }
});
```

## Testing the Todo App

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_add_todo() {
        let runtime = create_runtime();
        let scope = create_scope(runtime, |cx| {
            let (todos, _) = create_signal(cx, Vec::<TodoItem>::new());
            let (next_id, _) = create_signal(cx, 1u32);

            let add_todo = move |text: String| {
                if !text.trim().is_empty() {
                    let new_todo = TodoItem {
                        id: next_id.get(),
                        text: text.trim().to_string(),
                        completed: false,
                        created_at: "test".to_string(),
                    };
                    // Add to todos
                }
            };

            add_todo("Test todo".to_string());
            assert_eq!(todos.get().len(), 1);
            assert_eq!(todos.get()[0].text, "Test todo");
        });
        dispose_runtime(runtime);
    }

    #[test]
    fn test_toggle_todo() {
        let runtime = create_runtime();
        let scope = create_scope(runtime, |cx| {
            let (todos, set_todos) = create_signal(cx, vec![
                TodoItem {
                    id: 1,
                    text: "Test".to_string(),
                    completed: false,
                    created_at: "test".to_string(),
                }
            ]);

            let toggle_todo = move |id: u32| {
                set_todos.update(|todos| {
                    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                        todo.completed = !todo.completed;
                    }
                });
            };

            toggle_todo(1);
            assert!(todos.get()[0].completed);

            toggle_todo(1);
            assert!(!todos.get()[0].completed);
        });
        dispose_runtime(runtime);
    }
}
```

## Performance Considerations

### 1. List Virtualization
For large todo lists, consider implementing virtual scrolling:

```rust
#[component]
fn VirtualTodoList(cx: Scope, todos: ReadSignal<Vec<TodoItem>>) -> impl IntoView {
    let container_height = 400.0;
    let item_height = 50.0;

    let visible_range = create_memo(cx, move |_| {
        // Calculate which items should be visible
        (0..(todos.get().len().min(10))).collect::<Vec<_>>()
    });

    // Only render visible items
    // Implementation details...
}
```

### 2. Debounced Updates
```rust
#[component]
fn DebouncedTodoInput(cx: Scope) -> impl IntoView {
    let (input_value, set_input_value) = create_signal(cx, String::new());
    let (debounced_value, set_debounced_value) = create_signal(cx, String::new());

    create_effect(cx, move |_| {
        let value = input_value.get();
        let timeout = gloo_timers::callback::Timeout::new(300, move || {
            set_debounced_value.set(value);
        });

        on_cleanup(cx, move || {
            timeout.cancel();
        });
    });

    // Use debounced_value for expensive operations
}
```

## Related Examples

- [Basic Counter Example](basic-counter.md) - Simple state management
- [Form Handling Example](form-handling.md) - Controlled form inputs
- [Data Fetching Example](data-fetching.md) - Async operations with loading states

## Next Steps

1. Add categories/tags to todos
2. Implement drag-and-drop reordering
3. Add due dates and reminders
4. Create a shared todo list with real-time updates
5. Add search and filtering capabilities

This todo app example demonstrates how to build complex, interactive applications with Leptos, including state management, persistence, and component composition.