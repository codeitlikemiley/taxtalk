use leptos::prelude::*;
use leptos::html::Textarea;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenHint {
    pub token: String,
    pub description: String,
    pub example: String,
    pub category: String,
}

#[component]
pub fn TokenAutocomplete(
    input_ref: NodeRef<Textarea>,
    on_token_select: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (show_hints, set_show_hints) = signal(false);
    let (search_query, set_search_query) = signal(String::new());
    let (selected_index, set_selected_index) = signal(0);
    let (cursor_position, set_cursor_position) = signal(0);
    
    // Available tokens based on context
    let available_tokens = vec![
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
    
    // Clone for closures to avoid move errors
    let available_tokens_keydown = available_tokens.clone();
    let available_tokens_view = available_tokens.clone();
    
    // Monitor input for @ character
    Effect::new(move |_| {
        if let Some(input) = input_ref.get() {
            let value = input.value();
            let selection_start = input.selection_start()
                .ok()
                .flatten()
                .unwrap_or(0);
            
            set_cursor_position.set(selection_start as usize);
            
            // Check if we're at @ position
            if selection_start > 0 && value.chars().nth((selection_start - 1) as usize) == Some('@') {
                set_show_hints.set(true);
                set_search_query.set(String::new());
                set_selected_index.set(0);
            } else if show_hints.get() {
                // Extract text after @ for filtering
                let at_pos = value[..(selection_start as usize)].rfind('@');
                if let Some(pos) = at_pos {
                    let query = &value[pos + 1..(selection_start as usize)];
                    set_search_query.set(query.to_string());
                } else {
                    set_show_hints.set(false);
                }
            }
        }
    });
    
    // Handle keyboard navigation
    let handle_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if !show_hints.get() {
            if ev.key() == "@" {
                set_show_hints.set(true);
                set_search_query.set(String::new());
                set_selected_index.set(0);
            }
            return;
        }
        
        let query = search_query.get().to_lowercase();
        let current_tokens = if query.is_empty() {
            available_tokens_keydown.clone()
        } else {
            available_tokens_keydown
                .iter()
                .filter(|t| {
                    t.token.to_lowercase().contains(&query) ||
                    t.description.to_lowercase().contains(&query)
                })
                .cloned()
                .collect::<Vec<_>>()
        };
        
        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                let max = current_tokens.len().saturating_sub(1);
                set_selected_index.set((selected_index.get() + 1).min(max));
            },
            "ArrowUp" => {
                ev.prevent_default();
                set_selected_index.set(selected_index.get().saturating_sub(1usize));
            },
            "Tab" | "Enter" => {
                ev.prevent_default();
                if let Some(token) = current_tokens.get(selected_index.get()) {
                    insert_token(input_ref, cursor_position.get(), &token.token);
                    on_token_select.set(Some(token.token.clone()));
                    set_show_hints.set(false);
                }
            },
            "Escape" => {
                ev.prevent_default();
                set_show_hints.set(false);
            },
            _ => {}
        }
    };
    
    
    view! {
        <div class="relative">
            {move || {
                let query = search_query.get().to_lowercase();
                let tokens = if query.is_empty() {
                    available_tokens_view.clone()
                } else {
                    available_tokens_view
                        .iter()
                        .filter(|t| {
                            t.token.to_lowercase().contains(&query) ||
                            t.description.to_lowercase().contains(&query)
                        })
                        .cloned()
                        .collect::<Vec<_>>()
                };
                
                if show_hints.get() && !tokens.is_empty() {
                    view! {
                        <div class="absolute bottom-full mb-2 left-0 w-96 bg-white rounded-xl shadow-2xl border border-gray-200 overflow-hidden z-50">
                            <div class="px-3 py-2 bg-gradient-to-r from-blue-50 to-indigo-50 border-b border-gray-200">
                                <p class="text-xs font-medium text-gray-700">
                                    "Available Tokens - Press Tab or Enter to complete"
                                </p>
                            </div>
                            <div class="max-h-64 overflow-y-auto">
                                <For
                                    each=move || {
                                        tokens.clone().into_iter().enumerate()
                                    }
                                key=|(_, t)| t.token.clone()
                                children=move |(index, token)| {
                                    let is_selected = move || selected_index.get() == index;
                                    let token_for_click = token.clone();
                                    let category_color = match token.category.as_str() {
                                        "entity" => "blue",
                                        "value" => "green",
                                        "modifier" => "purple",
                                        "text" => "gray",
                                        _ => "gray"
                                    };
                                    
                                    view! {
                                        <button
                                            class={move || if is_selected() {
                                                "w-full px-3 py-2 text-left bg-blue-50 hover:bg-blue-100 transition-colors flex items-start space-x-3"
                                            } else {
                                                "w-full px-3 py-2 text-left hover:bg-gray-50 transition-colors flex items-start space-x-3"
                                            }}
                                            on:click=move |_| {
                                                insert_token(input_ref, cursor_position.get(), &token_for_click.token);
                                                on_token_select.set(Some(token_for_click.token.clone()));
                                                set_show_hints.set(false);
                                            }
                                            on:mouseenter=move |_| set_selected_index.set(index)
                                        >
                                            <div class={format!("mt-1 w-2 h-2 bg-{}-500 rounded-full flex-shrink-0", category_color)}></div>
                                            <div class="flex-1">
                                                <div class="flex items-center space-x-2">
                                                    <code class="text-sm font-mono text-blue-600">{token.token.clone()}</code>
                                                    <span class={format!("text-xs px-2 py-0.5 bg-{}-100 text-{}-700 rounded", category_color, category_color)}>
                                                        {token.category.clone()}
                                                    </span>
                                                </div>
                                                <p class="text-xs text-gray-600 mt-0.5">{token.description.clone()}</p>
                                                <p class="text-xs text-gray-400 mt-0.5 font-mono">{token.example.clone()}</p>
                                            </div>
                                        </button>
                                    }
                                }
                            />
                        </div>
                    </div>
                }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}

fn insert_token(input_ref: NodeRef<Textarea>, cursor_pos: usize, token: &str) {
    if let Some(input) = input_ref.get() {
        let current_value = input.value();
        
        // Find the @ position before cursor
        let at_pos = current_value[..cursor_pos].rfind('@').unwrap_or(cursor_pos);
        
        // Build new value
        let before = &current_value[..at_pos];
        let after = &current_value[cursor_pos..];
        let new_value = format!("{}{} {}", before, token, after);
        
        // Set the new value
        input.set_value(&new_value);
        
        // Set cursor position after the token
        let new_cursor_pos = before.len() + token.len() + 1;
        input.set_selection_start(Some(new_cursor_pos as u32)).ok();
        input.set_selection_end(Some(new_cursor_pos as u32)).ok();
        
        // Focus the input
        input.focus().ok();
    }
}