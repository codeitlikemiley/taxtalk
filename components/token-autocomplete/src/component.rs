use leptos::prelude::*;
use leptos::html::{Textarea, Input};
use leptos::ev::KeyboardEvent;
use crate::types::*;

/// TokenAutocomplete component - provides intelligent token suggestions
#[component]
pub fn TokenAutocomplete(
    /// Reference to the textarea element
    input_ref: NodeRef<Textarea>,
    
    /// Callback when a token is selected
    on_token_select: Callback<TokenHint, ()>,
    
    /// Optional custom token hints
    #[prop(optional)] custom_hints: Option<Vec<TokenHint>>,
    
    /// Configuration for autocomplete behavior
    #[prop(optional)] config: Option<TokenAutocompleteConfig>,
    
    /// Optional CSS class
    #[prop(optional)] class: Option<String>,
    
    /// Optional trigger to force updates
    #[prop(optional)] trigger: Option<Signal<i32>>,
    
    /// Optional controlled visibility for external management
    #[prop(optional)] controlled_visibility: Option<Signal<bool>>,
) -> impl IntoView {
    // Component state
    let (internal_show_hints, set_internal_show_hints) = signal(false);
    let (search_query, set_search_query) = signal(String::new());
    let (internal_index, set_internal_index) = signal(0);
    let (cursor_position, set_cursor_position) = signal(0);
    let filter_input_ref = NodeRef::<Input>::new();
    
    // Use controlled values if provided, otherwise use internal
    // For index, always use internal when we have our own filter input
    let selected_index = Signal::derive(move || internal_index.get());
    let show_hints = controlled_visibility.unwrap_or(Signal::derive(move || internal_show_hints.get()));
    
    // Configuration with defaults
    let config = config.unwrap_or_default();
    
    // Default token hints
    let default_hints = vec![
        TokenHint {
            token: "@client".to_string(),
            description: "Select or search for a client".to_string(),
            example: "@client ABC Corporation".to_string(),
            category: "entity".to_string(),
        },
        TokenHint {
            token: "@supplier".to_string(),
            description: "Select or search for a supplier".to_string(),
            example: "@supplier Tech Solutions Inc".to_string(),
            category: "entity".to_string(),
        },
        TokenHint {
            token: "@product".to_string(),
            description: "Select or search for a product".to_string(),
            example: "@product Rice 5kg".to_string(),
            category: "entity".to_string(),
        },
        TokenHint {
            token: "@amount".to_string(),
            description: "Enter an amount".to_string(),
            example: "@amount 5000".to_string(),
            category: "value".to_string(),
        },
        TokenHint {
            token: "@date".to_string(),
            description: "Specify a date".to_string(),
            example: "@date tomorrow".to_string(),
            category: "value".to_string(),
        },
        TokenHint {
            token: "@invoice".to_string(),
            description: "Reference an invoice".to_string(),
            example: "@invoice INV-2024-001".to_string(),
            category: "entity".to_string(),
        },
        TokenHint {
            token: "@payment".to_string(),
            description: "Reference a payment".to_string(),
            example: "@payment PAY-2024-001".to_string(),
            category: "entity".to_string(),
        },
        TokenHint {
            token: "@vat".to_string(),
            description: "VAT calculation mode".to_string(),
            example: "@vat inclusive".to_string(),
            category: "modifier".to_string(),
        },
        TokenHint {
            token: "@discount".to_string(),
            description: "Apply a discount".to_string(),
            example: "@discount senior 20%".to_string(),
            category: "modifier".to_string(),
        },
        TokenHint {
            token: "@notes".to_string(),
            description: "Add notes or description".to_string(),
            example: "@notes Professional services rendered".to_string(),
            category: "text".to_string(),
        },
    ];
    
    let available_hints = custom_hints.unwrap_or(default_hints);
    
    // Monitor input changes
    Effect::new(move |_| {
        // Subscribe to trigger if provided
        if let Some(trig) = trigger {
            trig.get();
        }
        
        // Only manage visibility internally if not controlled externally
        if controlled_visibility.is_none() {
            if let Some(textarea) = input_ref.get() {
                let value = textarea.value();
                let cursor_pos = textarea.selection_start()
                    .ok()
                    .flatten()
                    .unwrap_or(0) as usize;
                
                set_cursor_position.set(cursor_pos);
                
                // Check if we should show hints
                if cursor_pos > 0 {
                    let text_before = &value[..cursor_pos];
                    
                    // Find the last @ character
                    if let Some(at_pos) = text_before.rfind('@') {
                        let text_after_at = &text_before[at_pos + 1..];
                        
                        // Check if there's no space or newline after @
                        if !text_after_at.contains(' ') && !text_after_at.contains('\n') {
                            set_internal_show_hints.set(true);
                            set_search_query.set(String::new()); // Clear filter when opening
                            set_internal_index.set(0);
                            return;
                        }
                    }
                }
                
                set_internal_show_hints.set(false);
            }
        } else {
            // When controlled externally, just update cursor position
            if let Some(textarea) = input_ref.get() {
                let value = textarea.value();
                let cursor_pos = textarea.selection_start()
                    .ok()
                    .flatten()
                    .unwrap_or(0) as usize;
                
                set_cursor_position.set(cursor_pos);
            }
        }
    });
    
    // Auto-scroll selected item into view
    Effect::new(move |_| {
        let idx = selected_index.get();
        if show_hints.get() {
            // Use web_sys to scroll the element into view
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.get_element_by_id(&format!("tui-ta-item-{}", idx)) {
                        // Scroll the element into view with smooth scrolling
                        let scroll_options = web_sys::ScrollIntoViewOptions::new();
                        scroll_options.set_block(web_sys::ScrollLogicalPosition::Nearest);
                        scroll_options.set_behavior(web_sys::ScrollBehavior::Smooth);
                        element.scroll_into_view_with_scroll_into_view_options(&scroll_options);
                    }
                }
            }
        }
    });
    
    // Auto-focus filter input when dropdown opens
    Effect::new(move |_| {
        if show_hints.get() {
            // Small delay to ensure the element is rendered
            if let Some(input) = filter_input_ref.get() {
                let _ = input.focus();
            }
        }
    });
    
    // Filter hints based on query
    let filtered_hints = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        let hints = available_hints.clone();
        
        if query.is_empty() {
            hints
        } else {
            hints
                .into_iter()
                .filter(|h| {
                    h.token.to_lowercase().contains(&query) ||
                    h.description.to_lowercase().contains(&query)
                })
                .take(config.max_suggestions)
                .collect::<Vec<_>>()
        }
    });
    
    let custom_class = class.unwrap_or_default();
    
    view! {
        <div class={format!("tui-ta-wrapper {}", custom_class)}>
            {move || {
                let hints = filtered_hints.get();
                let showing = show_hints.get();
                
                if showing && !hints.is_empty() {
                    let hints_for_render = hints.clone();
                    
                    view! {
                        <div class="tui-ta-dropdown">
                            <div class="tui-ta-header">
                                <p class="tui-ta-title">
                                    "Available Tokens"
                                </p>
                                <p class="tui-ta-shortcuts">
                                    "Ctrl+P/N to navigate • Enter to select • Esc to close"
                                </p>
                            </div>
                            
                            <div class="tui-ta-filter">
                                <input
                                    node_ref=filter_input_ref
                                    type="text"
                                    class="tui-ta-filter-input"
                                    placeholder="Type to filter tokens..."
                                    value=move || search_query.get()
                                    on:input=move |ev| {
                                        let value = event_target_value(&ev);
                                        set_search_query.set(value);
                                        set_internal_index.set(0); // Reset selection when filtering
                                    }
                                    on:keydown=move |ev: KeyboardEvent| {
                                        let key = ev.key();
                                        let ctrl = ev.ctrl_key();
                                        
                                        let hints = filtered_hints.get();
                                        let max_index = hints.len().saturating_sub(1);
                                        
                                        match key.as_str() {
                                            "n" if ctrl => {
                                                ev.prevent_default();
                                                set_internal_index.update(|idx| {
                                                    *idx = (*idx + 1).min(max_index);
                                                });
                                            },
                                            "p" if ctrl => {
                                                ev.prevent_default();
                                                set_internal_index.update(|idx| {
                                                    *idx = idx.saturating_sub(1);
                                                });
                                            },
                                            "Enter" => {
                                                ev.prevent_default();
                                                if let Some(hint) = hints.get(selected_index.get()) {
                                                    // Insert token and notify parent
                                                    insert_token(input_ref, cursor_position.get(), &hint.token);
                                                    on_token_select.run(hint.clone());
                                                    
                                                    // Clear filter and hide
                                                    set_search_query.set(String::new());
                                                    set_internal_show_hints.set(false);
                                                    
                                                    // Return focus to textarea
                                                    if let Some(textarea) = input_ref.get() {
                                                        let _ = textarea.focus();
                                                    }
                                                }
                                            },
                                            "Escape" => {
                                                ev.prevent_default();
                                                set_search_query.set(String::new());
                                                set_internal_show_hints.set(false);
                                                
                                                // Return focus to textarea
                                                if let Some(textarea) = input_ref.get() {
                                                    let _ = textarea.focus();
                                                }
                                            },
                                            _ => {}
                                        }
                                    }
                                />
                            </div>
                            
                            <div class="tui-ta-list">
                                <For
                                    each=move || hints_for_render.clone().into_iter().enumerate()
                                    key=|(_, h)| h.token.clone()
                                    children=move |(index, hint)| {
                                        let is_selected = move || selected_index.get() == index;
                                        let hint_for_click = hint.clone();
                                        let category = TokenCategory::from_str(&hint.category);
                                        let color = category.as_ref().map(|c| c.color()).unwrap_or("gray");
                                        
                                        view! {
                                            <button
                                                id={format!("tui-ta-item-{}", index)}
                                                class={move || if is_selected() {
                                                    "tui-ta-item tui-ta-item-selected"
                                                } else {
                                                    "tui-ta-item"
                                                }}
                                                on:click=move |ev| {
                                                    ev.prevent_default();
                                                    // Always insert the token
                                                    insert_token(input_ref, cursor_position.get(), &hint_for_click.token);
                                                    
                                                    // Notify parent of selection
                                                    on_token_select.run(hint_for_click.clone());
                                                    
                                                    // Hide hints if not controlled externally
                                                    if controlled_visibility.is_none() {
                                                        set_internal_show_hints.set(false);
                                                    }
                                                }
                                            >
                                                <div class={format!("tui-ta-indicator tui-ta-indicator-{}", color)}></div>
                                                <div class="tui-ta-content">
                                                    <div class="tui-ta-token-line">
                                                        <code class="tui-ta-token">{hint.token.clone()}</code>
                                                        <span class={format!("tui-ta-category tui-ta-category-{}", color)}>
                                                            {hint.category.clone()}
                                                        </span>
                                                    </div>
                                                    <p class="tui-ta-description">{hint.description.clone()}</p>
                                                    <p class="tui-ta-example">{hint.example.clone()}</p>
                                                </div>
                                                {move || if is_selected() {
                                                    view! {
                                                        <div class="tui-ta-selected-marker">
                                                            "▶"
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <div></div> }.into_any()
                                                }}
                                            </button>
                                        }
                                    }
                                />
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div style="display: none;"></div> }.into_any()
                }
            }}
        </div>
    }
}

/// Insert a token at the current cursor position
fn insert_token(input_ref: NodeRef<Textarea>, cursor_pos: usize, token: &str) {
    if let Some(textarea) = input_ref.get() {
        let current_value = textarea.value();
        
        // Find the @ position before cursor
        let text_before = &current_value[..cursor_pos];
        let at_pos = text_before.rfind('@').unwrap_or(cursor_pos);
        
        // Build new value
        let before = &current_value[..at_pos];
        let after = &current_value[cursor_pos..];
        let new_value = format!("{}{} {}", before, token, after);
        
        // Set the new value
        textarea.set_value(&new_value);
        
        // Set cursor position after the token
        let new_cursor_pos = before.len() + token.len() + 1;
        let _ = textarea.set_selection_start(Some(new_cursor_pos as u32));
        let _ = textarea.set_selection_end(Some(new_cursor_pos as u32));
        
        // Focus the textarea
        let _ = textarea.focus();
        
        // Trigger input event to update the component
        textarea.dispatch_event(&web_sys::Event::new("input").unwrap()).ok();
    }
}