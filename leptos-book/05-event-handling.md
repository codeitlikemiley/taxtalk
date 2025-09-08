# 05: Event Handling

## Overview

Event handling is a crucial aspect of interactive web applications. Leptos provides a comprehensive event system that allows you to respond to user interactions, handle form submissions, manage keyboard input, and create custom events. This chapter covers the fundamental patterns for handling events in Leptos applications.

## Basic Event Handling

### Click Events

The most common event is the click event. Leptos uses the `on:click` directive:

```rust
use leptos::*;

// Basic click event
#[component]
pub fn ClickButton() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button on:click=move |_| set_count.update(|n| *n + 1)>
            "Clicked " {count} " times"
        </button>
    }
}
```

**Key Points:**
- Use `on:click` directive
- Handler receives `_|` for unused event parameter
- Use `move |_|` to capture variables by value
- Update signals using `.update()` or `.set()`

### Event Object Access

Access the event object for additional information:

```rust
use leptos::*;

// Accessing event properties
#[component]
pub fn EventInfo() -> impl IntoView {
    let (event_info, set_event_info) = create_signal(String::new());

    view! {
        <button on:click=move |ev| {
            let target = event_target::<web_sys::HtmlButtonElement>(&ev);
            if let Some(button) = target {
                set_event_info.set(format!(
                    "Button clicked: {}x{}",
                    button.offset_width(),
                    button.offset_height()
                ));
            }
        }>
            "Click for info"
        </button>
        <p>{event_info}</p>
    }
}
```

### Preventing Default Behavior

Prevent default browser behavior when needed:

```rust
use leptos::*;

// Preventing default behavior
#[component]
pub fn PreventDefaultExample() -> impl IntoView {
    let (message, set_message) = create_signal(String::new());

    view! {
        <form on:submit=move |ev| {
            ev.prevent_default(); // Prevent form submission
            set_message.set("Form submitted (but not really)".to_string());
        }>
            <input type="text" placeholder="Type something" />
            <button type="submit">"Submit (won't reload page)"</button>
        </form>
        <p>{message}</p>
    }
}
```

## Form Events

### Input Events

Handle text input changes:

```rust
use leptos::*;

// Text input handling
#[component]
pub fn TextInput() -> impl IntoView {
    let (text, set_text) = create_signal(String::new());

    view! {
        <div>
            <input
                type="text"
                on:input=move |ev| set_text.set(event_target_value(&ev))
                prop:value=text
            />
            <p>"You typed: " {text}</p>
        </div>
    }
}
```

### Checkbox Events

Handle checkbox state changes:

```rust
use leptos::*;

// Checkbox handling
#[component]
pub fn CheckboxExample() -> impl IntoView {
    let (checked, set_checked) = create_signal(false);

    view! {
        <div>
            <label>
                <input
                    type="checkbox"
                    on:change=move |ev| set_checked.set(event_target_checked(&ev))
                    prop:checked=checked
                />
                " I agree to the terms"
            </label>
            <p>
                {move || if checked.get() {
                    "Thank you for agreeing!"
                } else {
                    "Please agree to continue"
                }}
            </p>
        </div>
    }
}
```

### Select Events

Handle dropdown selection changes:

```rust
use leptos::*;

// Select dropdown handling
#[component]
pub fn SelectExample() -> impl IntoView {
    let (selected, set_selected) = create_signal("apple".to_string());

    view! {
        <div>
            <select on:change=move |ev| set_selected.set(event_target_value(&ev))>
                <option value="apple">"Apple"</option>
                <option value="banana">"Banana"</option>
                <option value="orange">"Orange"</option>
            </select>
            <p>"Selected: " {selected}</p>
        </div>
    }
}
```

### Form Submission

Handle complete form submissions:

```rust
use leptos::*;

// Form submission
#[derive(Clone, Debug)]
pub struct FormData {
    name: String,
    email: String,
    message: String,
}

#[component]
pub fn ContactForm() -> impl IntoView {
    let (form_data, set_form_data) = create_signal(FormData {
        name: String::new(),
        email: String::new(),
        message: String::new(),
    });
    let (submitted, set_submitted) = create_signal(false);

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        set_submitted.set(true);
        // Here you would typically send the data to a server
        log::info!("Form submitted: {:?}", form_data.get());
    };

    view! {
        <form on:submit=handle_submit>
            <div>
                <label>"Name:"</label>
                <input
                    type="text"
                    on:input=move |ev| {
                        set_form_data.update(|data| {
                            data.name = event_target_value(&ev);
                        });
                    }
                />
            </div>
            <div>
                <label>"Email:"</label>
                <input
                    type="email"
                    on:input=move |ev| {
                        set_form_data.update(|data| {
                            data.email = event_target_value(&ev);
                        });
                    }
                />
            </div>
            <div>
                <label>"Message:"</label>
                <textarea
                    on:input=move |ev| {
                        set_form_data.update(|data| {
                            data.message = event_target_value(&ev);
                        });
                    }
                ></textarea>
            </div>
            <button type="submit">"Send Message"</button>
        </form>
        {move || submitted.get().then(|| view! { <p>"Thank you for your message!"</p> })}
    }
}
```

## Keyboard Events

### Key Press Detection

Handle specific key presses:

```rust
use leptos::*;

// Keyboard event handling
#[component]
pub fn KeyboardExample() -> impl IntoView {
    let (pressed_key, set_pressed_key) = create_signal(String::new());

    view! {
        <div>
            <input
                type="text"
                placeholder="Press any key"
                on:keydown=move |ev| {
                    set_pressed_key.set(format!("Key: {}", ev.key()));
                }
            />
            <p>"Last key pressed: " {pressed_key}</p>
        </div>
    }
}
```

### Enter Key Handling

Common pattern for form submission on Enter:

```rust
use leptos::*;

// Enter key handling
#[component]
pub fn SearchInput() -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let (results, set_results) = create_signal(vec![]);

    let perform_search = move || {
        let search_term = query.get();
        if !search_term.is_empty() {
            // Simulate search
            set_results.set(vec![
                format!("Result 1 for '{}'", search_term),
                format!("Result 2 for '{}'", search_term),
            ]);
        }
    };

    view! {
        <div>
            <input
                type="text"
                placeholder="Search..."
                on:input=move |ev| set_query.set(event_target_value(&ev))
                on:keydown=move |ev| {
                    if ev.key() == "Enter" {
                        perform_search();
                    }
                }
            />
            <button on:click=move |_| perform_search()>"Search"</button>
            <ul>
                {move || results.get()
                    .into_iter()
                    .map(|result| view! { <li>{result}</li> })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
```

### Keyboard Shortcuts

Implement keyboard shortcuts:

```rust
use leptos::*;

// Keyboard shortcuts
#[component]
pub fn KeyboardShortcuts() -> impl IntoView {
    let (message, set_message) = create_signal(String::new());

    let handle_keydown = move |ev: ev::KeyboardEvent| {
        let key = ev.key();
        let ctrl = ev.ctrl_key();

        if ctrl && key == "s" {
            ev.prevent_default();
            set_message.set("Ctrl+S: Save".to_string());
        } else if ctrl && key == "z" {
            ev.prevent_default();
            set_message.set("Ctrl+Z: Undo".to_string());
        } else if key == "Escape" {
            set_message.set("Escape: Cancel".to_string());
        }
    };

    view! {
        <div on:keydown=handle_keydown tabindex="0">
            <p>"Press Ctrl+S to save, Ctrl+Z to undo, or Escape to cancel"</p>
            <p>{message}</p>
        </div>
    }
}
```

## Mouse Events

### Mouse Position Tracking

Track mouse movement:

```rust
use leptos::*;

// Mouse position tracking
#[component]
pub fn MouseTracker() -> impl IntoView {
    let (position, set_position) = create_signal((0, 0));

    view! {
        <div
            style="width: 300px; height: 200px; border: 1px solid black; position: relative;"
            on:mousemove=move |ev| {
                set_position.set((ev.client_x(), ev.client_y()));
            }
        >
            <p style="position: absolute; top: 10px; left: 10px;">
                "Mouse position: " {move || format!("({}, {})", position.get().0, position.get().1)}
            </p>
        </div>
    }
}
```

### Mouse Button Events

Handle different mouse buttons:

```rust
use leptos::*;

// Mouse button events
#[component]
pub fn MouseButtons() -> impl IntoView {
    let (button_info, set_button_info) = create_signal(String::new());

    view! {
        <div>
            <button
                on:mousedown=move |ev| {
                    let button = match ev.button() {
                        0 => "Left",
                        1 => "Middle",
                        2 => "Right",
                        _ => "Other",
                    };
                    set_button_info.set(format!("{} button pressed", button));
                }
                on:mouseup=move |_| {
                    set_button_info.set("Button released".to_string());
                }
            >
                "Click with different buttons"
            </button>
            <p>{button_info}</p>
        </div>
    }
}
```

## Custom Events

### Creating Custom Events

Define and dispatch custom events:

```rust
use leptos::*;
use web_sys::{CustomEvent, CustomEventInit};

// Custom event creation
#[component]
pub fn CustomEventExample() -> impl IntoView {
    let (messages, set_messages) = create_signal(vec![]);

    let dispatch_custom_event = move |message: String| {
        let mut init = CustomEventInit::new();
        init.detail(&wasm_bindgen::JsValue::from_str(&message));

        let event = CustomEvent::new_with_event_init_dict("my-custom-event", &init)
            .unwrap();

        // Dispatch on the window or a specific element
        web_sys::window()
            .unwrap()
            .dispatch_event(&event)
            .unwrap();
    };

    // Listen for custom events
    create_effect(move |_| {
        let closure = Closure::wrap(Box::new(move |event: CustomEvent| {
            let detail = event.detail().as_string().unwrap_or_default();
            set_messages.update(|msgs| msgs.push(format!("Custom event: {}", detail)));
        }) as Box<dyn FnMut(CustomEvent)>);

        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("my-custom-event", closure.as_ref().unchecked_ref())
            .unwrap();

        closure.forget(); // Keep the closure alive
    });

    view! {
        <div>
            <button on:click=move |_| dispatch_custom_event("Hello!".to_string())>
                "Send Custom Event"
            </button>
            <ul>
                {move || messages.get()
                    .into_iter()
                    .map(|msg| view! { <li>{msg}</li> })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
```

## Event Delegation

### Parent Element Event Handling

Handle events on parent elements for dynamic children:

```rust
use leptos::*;

// Event delegation
#[component]
pub fn TodoList() -> impl IntoView {
    let (todos, set_todos) = create_signal(vec![
        "Buy groceries".to_string(),
        "Walk the dog".to_string(),
        "Do laundry".to_string(),
    ]);

    let handle_click = move |ev: ev::MouseEvent| {
        let target = event_target::<web_sys::Element>(&ev);
        if let Some(element) = target {
            if let Some(id) = element.get_attribute("data-id") {
                if let Ok(index) = id.parse::<usize>() {
                    set_todos.update(|todos| {
                        if index < todos.len() {
                            todos.remove(index);
                        }
                    });
                }
            }
        }
    };

    view! {
        <ul on:click=handle_click>
            {move || todos.get()
                .into_iter()
                .enumerate()
                .map(|(index, todo)| view! {
                    <li data-id=index>
                        {todo}
                        <button>"Delete"</button>
                    </li>
                })
                .collect::<Vec<_>>()}
        </ul>
    }
}
```

## Event Propagation

### Stopping Event Propagation

Control event bubbling:

```rust
use leptos::*;

// Event propagation control
#[component]
pub fn EventPropagation() -> impl IntoView {
    let (messages, set_messages) = create_signal(vec![]);

    view! {
        <div on:click=move |_| set_messages.update(|msgs| msgs.push("Outer div clicked".to_string()))>
            <div on:click=move |_| set_messages.update(|msgs| msgs.push("Inner div clicked".to_string()))>
                <button
                    on:click=move |ev| {
                        ev.stop_propagation();
                        set_messages.update(|msgs| msgs.push("Button clicked".to_string()));
                    }
                >
                    "Click me (stops propagation)"
                </button>
            </div>
        </div>
        <ul>
            {move || messages.get()
                .into_iter()
                .rev() // Show newest first
                .take(5) // Show last 5 messages
                .map(|msg| view! { <li>{msg}</li> })
                .collect::<Vec<_>>()}
        </ul>
    }
}
```

## Debouncing and Throttling

### Debounced Input

Implement debounced search input:

```rust
use leptos::*;
use std::time::Duration;

// Debounced input
#[component]
pub fn DebouncedSearch() -> impl IntoView {
    let (query, set_query) = create_signal(String::new());
    let (debounced_query, set_debounced_query) = create_signal(String::new());

    // Debounce the input
    create_effect(move |_| {
        let current_query = query.get();
        let timeout_id = set_timeout_with_handle(
            move || {
                set_debounced_query.set(current_query);
            },
            Duration::from_millis(300)
        );

        // Cancel previous timeout if it exists
        if let Ok(id) = timeout_id {
            on_cleanup(move || {
                id.clear();
            });
        }
    });

    // Perform search when debounced query changes
    create_effect(move |_| {
        let search_term = debounced_query.get();
        if !search_term.is_empty() {
            log::info!("Searching for: {}", search_term);
            // Perform actual search here
        }
    });

    view! {
        <div>
            <input
                type="text"
                placeholder="Search..."
                on:input=move |ev| set_query.set(event_target_value(&ev))
            />
            <p>"Searching for: " {debounced_query}</p>
        </div>
    }
}
```

## Event Handler Patterns

### Event Handler Composition

Create reusable event handlers:

```rust
use leptos::*;

// Composable event handlers
fn with_logging<F, T>(handler: F) -> impl Fn(T)
where
    F: Fn(T),
    T: std::fmt::Debug + Clone,
{
    move |event| {
        log::info!("Event triggered: {:?}", event);
        handler(event);
    }
}

#[component]
pub fn ComposableHandlers() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let increment = move |_| set_count.update(|n| *n + 1);
    let increment_with_logging = with_logging(increment);

    view! {
        <button on:click=increment_with_logging>
            "Increment (with logging): " {count}
        </button>
    }
}
```

### Conditional Event Handlers

Enable/disable event handlers based on conditions:

```rust
use leptos::*;

// Conditional event handlers
#[component]
pub fn ConditionalButton() -> impl IntoView {
    let (is_enabled, set_is_enabled) = create_signal(true);
    let (click_count, set_click_count) = create_signal(0);

    let handle_click = move |_| {
        set_click_count.update(|n| *n + 1);
    };

    view! {
        <div>
            <button
                on:click=move |ev| {
                    if is_enabled.get() {
                        handle_click(ev);
                    }
                }
                class=move || if is_enabled.get() { "enabled" } else { "disabled" }
            >
                "Click me: " {click_count}
            </button>
            <br />
            <label>
                <input
                    type="checkbox"
                    prop:checked=is_enabled
                    on:change=move |ev| set_is_enabled.set(event_target_checked(&ev))
                />
                " Enable button"
            </label>
        </div>
    }
}
```

## Error Handling in Events

### Event Error Handling

Handle errors that occur during event processing:

```rust
use leptos::*;

// Error handling in events
#[component]
pub fn SafeEventHandler() -> impl IntoView {
    let (result, set_result) = create_signal(String::new());

    let handle_click = move |_| {
        match perform_risky_operation() {
            Ok(success) => set_result.set(format!("Success: {}", success)),
            Err(error) => {
                log::error!("Operation failed: {}", error);
                set_result.set(format!("Error: {}", error));
            }
        }
    };

    view! {
        <div>
            <button on:click=handle_click>
                "Perform Risky Operation"
            </button>
            <p>{result}</p>
        </div>
    }
}

fn perform_risky_operation() -> Result<String, String> {
    // Simulate an operation that might fail
    if rand::random::<bool>() {
        Ok("Operation completed successfully".to_string())
    } else {
        Err("Something went wrong".to_string())
    }
}
```

## Performance Considerations

### Event Handler Optimization

Avoid creating new closures on every render:

```rust
use leptos::*;

// Optimized event handlers
#[component]
pub fn OptimizedHandlers() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    // Create handler once, outside the view
    let increment = move |_| set_count.update(|n| *n + 1);

    view! {
        <div>
            <button on:click=increment.clone()>"Button 1"</button>
            <button on:click=increment.clone()>"Button 2"</button>
            <button on:click=increment>"Button 3"</button>
            <p>"Count: " {count}</p>
        </div>
    }
}
```

### Memoized Event Handlers

Use memoization for complex event handlers:

```rust
use leptos::*;

// Memoized event handlers
#[component]
pub fn MemoizedHandler() -> impl IntoView {
    let (items, set_items) = create_signal(vec![1, 2, 3, 4, 5]);

    // Memoize the expensive handler
    let remove_item_handler = create_memo(move |_| {
        let current_items = items.get();
        move |index: usize| {
            set_items.set(
                current_items.iter()
                    .enumerate()
                    .filter(|(i, _)| *i != index)
                    .map(|(_, item)| *item)
                    .collect()
            );
        }
    });

    view! {
        <ul>
            {move || items.get()
                .into_iter()
                .enumerate()
                .map(|(index, item)| {
                    let handler = remove_item_handler.get();
                    view! {
                        <li>
                            {item}
                            <button on:click=move |_| handler(index)>"Remove"</button>
                        </li>
                    }
                })
                .collect::<Vec<_>>()}
        </ul>
    }
}
```

## Testing Event Handlers

### Event Handler Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_button_click_increments_counter() {
        let mut app = create_runtime();

        let (count, set_count) = create_signal(0);
        let increment = move |_| set_count.update(|n| *n + 1);

        // Simulate button click
        increment(());

        assert_eq!(count.get(), 1);

        app.dispose();
    }

    #[test]
    fn test_form_submission_prevents_default() {
        let mut app = create_runtime();

        let (submitted, set_submitted) = create_signal(false);

        let handle_submit = move |ev: ev::SubmitEvent| {
            ev.prevent_default();
            set_submitted.set(true);
        };

        // In a real test, you'd need to create a mock event
        // This is a simplified example

        app.dispose();
    }
}
```

## Summary

Leptos provides a comprehensive event handling system that integrates seamlessly with its reactive model. Key concepts covered:

1. **Basic Events**: Click, input, change events with `on:` directives
2. **Form Handling**: Text inputs, checkboxes, selects, and form submission
3. **Keyboard Events**: Key detection, shortcuts, and special key handling
4. **Mouse Events**: Position tracking, button detection, and movement
5. **Custom Events**: Creating and dispatching custom events
6. **Event Delegation**: Handling events on parent elements
7. **Event Propagation**: Controlling event bubbling with `stop_propagation()`
8. **Debouncing**: Implementing delayed event handling
9. **Performance**: Optimizing event handlers and avoiding unnecessary re-renders
10. **Error Handling**: Graceful error handling in event handlers

Best practices:
- Use `move |_|` for simple handlers, `move |ev|` when you need the event
- Prevent default behavior with `ev.prevent_default()` when needed
- Use `event_target_value()` and `event_target_checked()` for form inputs
- Implement debouncing for search inputs and other frequent events
- Memoize expensive event handlers
- Handle errors gracefully in event handlers
- Test event handlers thoroughly

---

**Next:** [06: Data Flow](06-data-flow.md) - Learn about props, context, and state management patterns.