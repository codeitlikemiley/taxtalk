# 05: Event Handling

## Overview

Leptos provides a comprehensive event handling system that integrates seamlessly with the reactive system. This chapter covers user interactions, event propagation, custom events, and best practices for managing user input effectively.

## Basic Event Handling

### Click Events

```rust
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

### Multiple Event Handlers

```rust
#[component]
pub fn MultiEventButton() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (message, set_message) = create_signal("Click me!".to_string());

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n + 1);
                set_message("Clicked!".to_string());
            }
            on:mouseenter=move |_| set_message("Hovering...".to_string())
            on:mouseleave=move |_| set_message("Click me!".to_string())
        >
            {message}
        </button>
    }
}
```

### Event Object Access

```rust
#[component]
pub fn EventDetails() -> impl IntoView {
    let (click_info, set_click_info) = create_signal("No clicks yet".to_string());

    let handle_click = move |ev: web_sys::MouseEvent| {
        let x = ev.client_x();
        let y = ev.client_y();
        let button = ev.button();
        set_click_info(format!("Clicked at ({}, {}) with button {}", x, y, button));
    };

    view! {
        <div>
            <button on:click=handle_click>"Click me"</button>
            <p>{click_info}</p>
        </div>
    }
}
```

## Form Event Handling

### Input Events

```rust
#[component]
pub fn TextInput() -> impl IntoView {
    let (value, set_value) = create_signal("".to_string());

    view! {
        <div>
            <input
                type="text"
                prop:value=value
                on:input=move |ev| {
                    let target = event_target::<web_sys::HtmlInputElement>(&ev);
                    if let Some(input) = target {
                        set_value(input.value());
                    }
                }
            />
            <p>"You typed: " {value}</p>
        </div>
    }
}
```

### Controlled Components

```rust
#[component]
pub fn ControlledForm() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (submitted, set_submitted) = create_signal(false);

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default(); // Prevent default form submission
        set_submitted(true);
    };

    view! {
        <form on:submit=handle_submit>
            <div>
                <label>"Name:"</label>
                <input
                    type="text"
                    prop:value=name
                    on:input=move |ev| {
                        let target = event_target::<web_sys::HtmlInputElement>(&ev);
                        if let Some(input) = target {
                            set_name(input.value());
                        }
                    }
                />
            </div>
            <div>
                <label>"Email:"</label>
                <input
                    type="email"
                    prop:value=email
                    on:input=move |ev| {
                        let target = event_target::<web_sys::HtmlInputElement>(&ev);
                        if let Some(input) = target {
                            set_email(input.value());
                        }
                    }
                />
            </div>
            <button type="submit">"Submit"</button>
        </form>
        <Show
            when=move || submitted()
            fallback=|| view! { <div></div> }
        >
            <p>"Form submitted with name: " {name} " and email: " {email}</p>
        </Show>
    }
}
```

### Checkbox and Radio Buttons

```rust
#[component]
pub fn CheckboxGroup() -> impl IntoView {
    let (options, set_options) = create_signal(vec![
        ("option1".to_string(), "Option 1".to_string(), false),
        ("option2".to_string(), "Option 2".to_string(), false),
        ("option3".to_string(), "Option 3".to_string(), false),
    ]);

    let handle_checkbox = move |index: usize| {
        move |ev: web_sys::Event| {
            let target = event_target::<web_sys::HtmlInputElement>(&ev);
            if let Some(checkbox) = target {
                set_options.update(|opts| {
                    if let Some(opt) = opts.get_mut(index) {
                        opt.2 = checkbox.checked();
                    }
                });
            }
        }
    };

    view! {
        <div>
            <For
                each=move || options()
                key=|opt| opt.0.clone()
                children=move |(id, label, checked), index| {
                    view! {
                        <div>
                            <input
                                type="checkbox"
                                id=id.clone()
                                prop:checked=checked
                                on:change=handle_checkbox(index)
                            />
                            <label for=id>{label}</label>
                        </div>
                    }
                }
            />
            <p>
                "Selected: " {
                    options()
                        .into_iter()
                        .filter(|(_, _, checked)| *checked)
                        .map(|(_, label, _)| label)
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            </p>
        </div>
    }
}
```

### Select Dropdown

```rust
#[component]
pub fn SelectDropdown() -> impl IntoView {
    let (selected, set_selected) = create_signal("".to_string());
    let options = vec![
        ("", "Choose an option"),
        ("option1", "Option 1"),
        ("option2", "Option 2"),
        ("option3", "Option 3"),
    ];

    view! {
        <div>
            <select
                prop:value=selected
                on:change=move |ev| {
                    let target = event_target::<web_sys::HtmlSelectElement>(&ev);
                    if let Some(select) = target {
                        set_selected(select.value());
                    }
                }
            >
                {options.into_iter().map(|(value, label)| view! {
                    <option value=value>{label}</option>
                }).collect::<Vec<_>>()}
            </select>
            <p>"Selected: " {selected}</p>
        </div>
    }
}
```

## Keyboard Events

### Key Press Handling

```rust
#[component]
pub fn KeyboardInput() -> impl IntoView {
    let (input_value, set_input_value) = create_signal("".to_string());
    let (last_key, set_last_key) = create_signal("".to_string());

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        let key = ev.key();
        set_last_key(key.clone());

        // Handle special keys
        match key.as_str() {
            "Enter" => {
                logging::log!("Enter pressed with value: {}", input_value());
            }
            "Escape" => {
                set_input_value("".to_string());
            }
            "ArrowUp" => {
                // Handle up arrow
            }
            "ArrowDown" => {
                // Handle down arrow
            }
            _ => {}
        }
    };

    view! {
        <div>
            <input
                type="text"
                prop:value=input_value
                on:input=move |ev| {
                    let target = event_target::<web_sys::HtmlInputElement>(&ev);
                    if let Some(input) = target {
                        set_input_value(input.value());
                    }
                }
                on:keydown=handle_keydown
            />
            <p>"Current value: " {input_value}</p>
            <p>"Last key pressed: " {last_key}</p>
        </div>
    }
}
```

### Auto-complete with Keyboard Navigation

```rust
#[component]
pub fn AutocompleteInput() -> impl IntoView {
    let (input_value, set_input_value) = create_signal("".to_string());
    let (suggestions, set_suggestions) = create_signal(vec!["Apple", "Banana", "Cherry", "Date"]);
    let (filtered_suggestions, set_filtered_suggestions) = create_signal(Vec::<String>::new());
    let (selected_index, set_selected_index) = create_signal(0);
    let (show_suggestions, set_show_suggestions) = create_signal(false);

    // Filter suggestions based on input
    create_effect(move |_| {
        let current_input = input_value();
        if current_input.is_empty() {
            set_filtered_suggestions(vec![]);
            set_show_suggestions(false);
        } else {
            let filtered = suggestions()
                .into_iter()
                .filter(|s| s.to_lowercase().contains(&current_input.to_lowercase()))
                .collect::<Vec<_>>();
            set_filtered_suggestions(filtered);
            set_show_suggestions(true);
            set_selected_index(0);
        }
    });

    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                set_selected_index.update(|i| {
                    let max_index = filtered_suggestions().len().saturating_sub(1);
                    *i = (*i + 1).min(max_index);
                });
            }
            "ArrowUp" => {
                ev.prevent_default();
                set_selected_index.update(|i| *i = i.saturating_sub(1));
            }
            "Enter" => {
                if show_suggestions() && !filtered_suggestions().is_empty() {
                    ev.prevent_default();
                    if let Some(selected) = filtered_suggestions().get(selected_index()) {
                        set_input_value(selected.clone());
                        set_show_suggestions(false);
                    }
                }
            }
            "Escape" => {
                set_show_suggestions(false);
            }
            _ => {}
        }
    };

    view! {
        <div class="autocomplete-container">
            <input
                type="text"
                prop:value=input_value
                on:input=move |ev| {
                    let target = event_target::<web_sys::HtmlInputElement>(&ev);
                    if let Some(input) = target {
                        set_input_value(input.value());
                    }
                }
                on:focus=move |_| {
                    if !filtered_suggestions().is_empty() {
                        set_show_suggestions(true);
                    }
                }
                on:keydown=handle_keydown
            />
            <Show
                when=move || show_suggestions() && !filtered_suggestions().is_empty()
                fallback=|| view! { <div></div> }
            >
                <ul class="suggestions-list">
                    <For
                        each=move || filtered_suggestions()
                        key=|s| s.clone()
                        children=move |suggestion, index| {
                            let is_selected = move || index == selected_index();
                            view! {
                                <li
                                    class=move || if is_selected() { "selected" } else { "" }
                                    on:click=move |_| {
                                        set_input_value(suggestion.clone());
                                        set_show_suggestions(false);
                                    }
                                >
                                    {suggestion}
                                </li>
                            }
                        }
                    />
                </ul>
            </Show>
        </div>
    }
}
```

## Custom Events

### Creating Custom Event Handlers

```rust
use leptos::*;

// Define custom event data
#[derive(Clone, Debug)]
pub struct CustomEventData {
    pub action: String,
    pub payload: serde_json::Value,
}

// Custom event callback type
pub type CustomEventHandler = Callback<CustomEventData, ()>;

#[component]
pub fn CustomEventEmitter(
    on_custom_event: CustomEventHandler,
) -> impl IntoView {
    let emit_event = move |action: &str, payload: serde_json::Value| {
        on_custom_event(CustomEventData {
            action: action.to_string(),
            payload,
        });
    };

    view! {
        <div>
            <button on:click=move |_| emit_event("increment", serde_json::json!(1))>
                "Increment"
            </button>
            <button on:click=move |_| emit_event("decrement", serde_json::json!(-1))>
                "Decrement"
            </button>
            <button on:click=move |_| emit_event("reset", serde_json::json!(0))>
                "Reset"
            </button>
        </div>
    }
}
```

### Event Delegation

```rust
#[component]
pub fn EventDelegation() -> impl IntoView {
    let (items, set_items) = create_signal(vec![
        "Item 1".to_string(),
        "Item 2".to_string(),
        "Item 3".to_string(),
    ]);

    let handle_item_click = move |ev: web_sys::MouseEvent| {
        // Use event delegation to handle clicks on any item
        let target = ev.target().unwrap();
        let element = target.dyn_ref::<web_sys::Element>().unwrap();

        if let Some(item_element) = element.closest("[data-item-id]") {
            if let Ok(Some(item_id)) = item_element.get_attribute("data-item-id") {
                logging::log!("Clicked item: {}", item_id);
                // Handle item click logic here
            }
        }
    };

    view! {
        <ul on:click=handle_item_click>
            <For
                each=move || items()
                key=|item| item.clone()
                children=move |item, index| {
                    view! {
                        <li data-item-id=format!("item-{}", index)>
                            {item}
                        </li>
                    }
                }
            />
        </ul>
    }
}
```

## Event Propagation and Prevention

### Stopping Event Propagation

```rust
#[component]
pub fn EventPropagation() -> impl IntoView {
    let (outer_clicks, set_outer_clicks) = create_signal(0);
    let (inner_clicks, set_inner_clicks) = create_signal(0);

    view! {
        <div
            class="outer"
            on:click=move |_| {
                set_outer_clicks.update(|n| *n + 1);
                logging::log!("Outer div clicked");
            }
        >
            <div
                class="inner"
                on:click=move |ev| {
                    ev.stop_propagation(); // Prevent event from bubbling up
                    set_inner_clicks.update(|n| *n + 1);
                    logging::log!("Inner div clicked");
                }
            >
                "Click me!"
            </div>
            <p>"Outer clicks: " {outer_clicks}</p>
            <p>"Inner clicks: " {inner_clicks}</p>
        </div>
    }
}
```

### Preventing Default Behavior

```rust
#[component]
pub fn PreventDefault() -> impl IntoView {
    view! {
        <div>
            <a
                href="https://example.com"
                on:click=move |ev| {
                    ev.prevent_default(); // Prevent navigation
                    logging::log!("Link clicked but navigation prevented");
                }
            >
                "This link won't navigate"
            </a>
        </div>
    }
}
```

## Advanced Event Patterns

### Debounced Events

```rust
use std::time::Duration;

#[component]
pub fn DebouncedSearch() -> impl IntoView {
    let (search_term, set_search_term) = create_signal("".to_string());
    let (debounced_term, set_debounced_term) = create_signal("".to_string());

    // Debounce the search term
    create_effect(move |_| {
        let current_term = search_term();
        let timeout_id = set_timeout(
            move || set_debounced_term(current_term),
            Duration::from_millis(300),
        );

        on_cleanup(move || clear_timeout(timeout_id));
    });

    // Perform search when debounced term changes
    create_effect(move |_| {
        let term = debounced_term();
        if !term.is_empty() {
            logging::log!("Searching for: {}", term);
            // Perform actual search here
        }
    });

    view! {
        <div>
            <input
                type="text"
                placeholder="Search..."
                prop:value=search_term
                on:input=move |ev| {
                    let target = event_target::<web_sys::HtmlInputElement>(&ev);
                    if let Some(input) = target {
                        set_search_term(input.value());
                    }
                }
            />
            <p>"Searching for: " {debounced_term}</p>
        </div>
    }
}
```

### Throttled Events

```rust
#[component]
pub fn ThrottledScroll() -> impl IntoView {
    let (scroll_position, set_scroll_position) = create_signal(0);
    let (last_update, set_last_update) = create_signal(0u64);

    let handle_scroll = move |_| {
        let now = js_sys::Date::now() as u64;
        let time_since_last_update = now - last_update();

        // Only update if at least 100ms have passed
        if time_since_last_update >= 100 {
            if let Ok(Some(window)) = web_sys::window() {
                if let Ok(scroll_y) = window.scroll_y() {
                    set_scroll_position(scroll_y as i32);
                    set_last_update(now);
                }
            }
        }
    };

    // Attach scroll listener on mount
    create_effect(move |_| {
        if let Ok(Some(window)) = web_sys::window() {
            let closure = Closure::wrap(Box::new(handle_scroll) as Box<dyn FnMut()>);
            let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            closure.forget(); // Leak the closure to keep it alive
        }
    });

    view! {
        <div>
            <div style="height: 2000px; padding: 20px;">
                "Scroll down to see the position update (throttled)"
            </div>
            <div style="position: fixed; bottom: 20px; right: 20px; background: white; padding: 10px; border: 1px solid black;">
                "Scroll position: " {scroll_position}
            </div>
        </div>
    }
}
```

## File Upload Events

### Single File Upload

```rust
#[component]
pub fn FileUpload() -> impl IntoView {
    let (file_name, set_file_name) = create_signal("No file selected".to_string());
    let (file_size, set_file_size) = create_signal(0);

    let handle_file_change = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Some(input) = target {
            if let Ok(Some(file_list)) = input.files() {
                if file_list.length() > 0 {
                    if let Ok(Some(file)) = file_list.get(0) {
                        set_file_name(file.name());
                        set_file_size(file.size() as usize);
                    }
                }
            }
        }
    };

    view! {
        <div>
            <input
                type="file"
                on:change=handle_file_change
            />
            <p>"Selected file: " {file_name}</p>
            <p>"File size: " {file_size} " bytes"</p>
        </div>
    }
}
```

### Multiple File Upload with Preview

```rust
#[component]
pub fn MultiFileUpload() -> impl IntoView {
    let (files, set_files) = create_signal(Vec::<(String, usize)>::new());

    let handle_files_change = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Some(input) = target {
            if let Ok(Some(file_list)) = input.files() {
                let mut new_files = Vec::new();
                for i in 0..file_list.length() {
                    if let Ok(Some(file)) = file_list.get(i) {
                        new_files.push((file.name(), file.size() as usize));
                    }
                }
                set_files(new_files);
            }
        }
    };

    view! {
        <div>
            <input
                type="file"
                multiple=true
                on:change=handle_files_change
            />
            <div>
                <h3>"Selected files:"</h3>
                <ul>
                    <For
                        each=move || files()
                        key=|(name, _)| name.clone()
                        children=move |(name, size)| {
                            view! {
                                <li>{name} " (" {size} " bytes)"</li>
                            }
                        }
                    />
                </ul>
            </div>
        </div>
    }
}
```

## Drag and Drop Events

### File Drag and Drop

```rust
#[component]
pub fn DragDropUpload() -> impl IntoView {
    let (is_dragging, set_is_dragging) = create_signal(false);
    let (dropped_files, set_dropped_files) = create_signal(Vec::<String>::new());

    let handle_drag_over = move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        set_is_dragging(true);
    };

    let handle_drag_leave = move |_| {
        set_is_dragging(false);
    };

    let handle_drop = move |ev: web_sys::DragEvent| {
        ev.prevent_default();
        set_is_dragging(false);

        if let Ok(Some(data_transfer)) = ev.data_transfer() {
            if let Ok(Some(file_list)) = data_transfer.files() {
                let mut file_names = Vec::new();
                for i in 0..file_list.length() {
                    if let Ok(Some(file)) = file_list.get(i) {
                        file_names.push(file.name());
                    }
                }
                set_dropped_files(file_names);
            }
        }
    };

    view! {
        <div
            class=move || if is_dragging() { "drop-zone dragging" } else { "drop-zone" }
            on:dragover=handle_drag_over
            on:dragleave=handle_drag_leave
            on:drop=handle_drop
        >
            <p>"Drop files here"</p>
            <div>
                <h4>"Dropped files:"</h4>
                <ul>
                    <For
                        each=move || dropped_files()
                        key=|name| name.clone()
                        children=move |name| {
                            view! { <li>{name}</li> }
                        }
                    />
                </ul>
            </div>
        </div>
    }
}
```

## Event Testing

### Testing Event Handlers

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_click_counter() {
        let (count, set_count) = create_signal(0);

        // Simulate click event
        set_count.update(|n| *n + 1);

        assert_eq!(count(), 1);
    }

    #[test]
    fn test_form_validation() {
        let (email, set_email) = create_signal("".to_string());
        let (is_valid, set_is_valid) = create_signal(false);

        // Simulate email input
        set_email("test@example.com".to_string());

        // Check if email is valid
        create_effect(move |_| {
            let email_pattern = regex::Regex::new(r"^[^@]+@[^@]+\.[^@]+$").unwrap();
            set_is_valid(email_pattern.is_match(&email()));
        });

        assert!(is_valid());
    }
}
```

## Best Practices

1. **Use event delegation**: For dynamic lists to improve performance
2. **Prevent default behavior**: When implementing custom form handling
3. **Stop propagation**: When nested elements have conflicting event handlers
4. **Debounce/throttle**: For performance-critical events like scroll and resize
5. **Handle errors gracefully**: Provide fallbacks for failed event handlers
6. **Clean up event listeners**: Use `on_cleanup` to prevent memory leaks
7. **Use appropriate event types**: Choose the most specific event for your use case
8. **Test event interactions**: Ensure event handlers work as expected
9. **Consider accessibility**: Make sure events work with keyboard navigation
10. **Document event APIs**: Clearly specify what events components emit

## Summary

Leptos event handling provides:

- **Reactive event binding**: Events automatically update reactive state
- **Type-safe event handling**: Strong typing for event objects and targets
- **Flexible event patterns**: Support for custom events and event delegation
- **Performance optimizations**: Debouncing, throttling, and efficient updates
- **Comprehensive event types**: Support for all DOM events and custom events
- **Clean integration**: Events work seamlessly with the reactive system

Mastering event handling is crucial for building interactive Leptos applications that provide excellent user experiences.