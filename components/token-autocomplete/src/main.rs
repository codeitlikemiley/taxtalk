use leptos::prelude::*;
use leptos::html::Textarea;
use token_autocomplete::{TokenAutocomplete, TokenHint, TokenAutocompleteConfig};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let textarea_ref = NodeRef::<Textarea>::new();
    let (selected_tokens, set_selected_tokens) = signal(Vec::<TokenHint>::new());
    let (command_text, set_command_text) = signal(String::new());
    let (event_log, set_event_log) = signal(Vec::<String>::new());
    
    // Trigger state for forcing the autocomplete to check
    let (trigger_check, set_trigger_check) = signal(0);
    
    let log_event = move |msg: String| {
        set_event_log.update(|log| {
            log.push(msg);
            if log.len() > 10 {
                log.remove(0);
            }
        });
    };
    
    // State for managing autocomplete - moved before handle_token_select
    let (autocomplete_showing, set_autocomplete_showing) = signal(false);
    let (available_hints, set_available_hints) = signal(Vec::<TokenHint>::new());
    
    let handle_token_select = move |token: TokenHint| {
        log_event(format!("Selected token: {} ({})", token.token, token.category));
        set_selected_tokens.update(|tokens| {
            tokens.push(token);
        });
        
        // Hide the autocomplete dropdown after selection
        set_autocomplete_showing.set(false);
        
        // Force trigger update to re-evaluate the input
        set_trigger_check.update(|v| *v += 1);
    };
    
    // Custom configuration
    let config = TokenAutocompleteConfig {
        max_suggestions: 8,
        debounce_ms: 200,
        enable_fuzzy_search: true,
        show_examples: true,
        show_categories: true,
    };
    
    // Default hints
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
    
    // Force the TokenAutocomplete to update when input changes
    Effect::new(move |_| {
        command_text.get(); // Subscribe to changes
        trigger_check.get(); // Subscribe to trigger
        
        // Log what we're typing
        if let Some(textarea) = textarea_ref.get() {
            let value = textarea.value();
            let cursor_pos = textarea.selection_start().ok().flatten().unwrap_or(0) as usize;
            
            // Check if we should show autocomplete
            if cursor_pos > 0 {
                let text_before = &value[..cursor_pos];
                
                // Find the last @ character before cursor
                if let Some(at_pos) = text_before.rfind('@') {
                    let text_after_at = &text_before[at_pos + 1..];
                    
                    // Check if this is a complete token (has a space after it)
                    // or if we're still typing (no space)
                    if text_after_at.contains(' ') {
                        // This is a complete token, don't show autocomplete
                        if autocomplete_showing.get() {
                            log_event("Hiding autocomplete - token is complete".to_string());
                            set_autocomplete_showing.set(false);
                        }
                    } else if !text_after_at.contains('\n') {
                        // Still typing a token, show autocomplete
                        if !autocomplete_showing.get() {
                            log_event(format!("Showing autocomplete at position: {}", cursor_pos));
                            set_autocomplete_showing.set(true);
                            set_available_hints.set(default_hints.clone());
                        }
                        return;
                    }
                } else {
                    // No @ character found, hide autocomplete
                    if autocomplete_showing.get() {
                        log_event("Hiding autocomplete - no @ character".to_string());
                        set_autocomplete_showing.set(false);
                    }
                }
            } else {
                // Cursor at position 0, hide autocomplete
                if autocomplete_showing.get() {
                    log_event("Hiding autocomplete - cursor at start".to_string());
                    set_autocomplete_showing.set(false);
                }
            }
        }
    });
    
    view! {
        <div class="demo-container">
            <style>{include_str!("styles.css")}</style>
            
            <h1>"TokenAutocomplete Component Demo"</h1>
            <p class="subtitle">"Type @ to see available tokens for natural language commands"</p>
            
            <div class="demo-section">
                <h2>"Command Input"</h2>
                <div class="input-wrapper">
                    <textarea
                        node_ref=textarea_ref
                        class="command-input"
                        placeholder="Try typing '@' to see available tokens..."
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_command_text.set(value.clone());
                            set_trigger_check.update(|v| *v += 1); // Force update
                            log_event(format!("Input changed: {}", value));
                        }
                        on:keydown=move |ev: leptos::ev::KeyboardEvent| {
                            // Just trigger update to check for @ character
                            set_trigger_check.update(|v| *v += 1);
                        }
                        rows=5
                    />
                    <TokenAutocomplete
                        input_ref=textarea_ref
                        on_token_select=Callback::new(handle_token_select)
                        config=config
                        trigger=Signal::derive(move || trigger_check.get())
                        controlled_visibility=Signal::derive(move || autocomplete_showing.get())
                    />
                </div>
                <div class="command-preview">
                    <strong>"Current Command:"</strong>
                    <code>{move || command_text.get()}</code>
                </div>
                
                <div class="debug-info" style="margin-top: 1rem; padding: 0.5rem; background: #f0f0f0; border-radius: 4px; font-size: 0.875rem;">
                    <strong>"Debug Info:"</strong>
                    <div>"Trigger count: "{move || trigger_check.get()}</div>
                    <div>"Text length: "{move || command_text.get().len()}</div>
                    <div>"Contains @: "{move || if command_text.get().contains('@') { "Yes" } else { "No" }}</div>
                </div>
            </div>
            
            <div class="demo-section">
                <h2>"Selected Tokens"</h2>
                <div class="token-list">
                    <Show
                        when=move || !selected_tokens.get().is_empty()
                        fallback=|| view! { <p class="empty-state">"No tokens selected yet"</p> }
                    >
                        <For
                            each=move || selected_tokens.get()
                            key=|token| format!("{}-{}", token.token, token.description)
                            children=move |token| {
                                let color = match token.category.as_str() {
                                    "entity" => "#3b82f6",
                                    "value" => "#10b981",
                                    "modifier" => "#8b5cf6",
                                    "text" => "#6b7280",
                                    _ => "#6b7280"
                                };
                                
                                view! {
                                    <div class="selected-token" style={format!("border-left-color: {}", color)}>
                                        <div class="token-header">
                                            <code>{token.token.clone()}</code>
                                            <span class="token-category">{token.category.clone()}</span>
                                        </div>
                                        <p class="token-desc">{token.description.clone()}</p>
                                    </div>
                                }
                            }
                        />
                    </Show>
                </div>
            </div>
            
            <div class="demo-section">
                <h2>"Event Log"</h2>
                <div class="event-log">
                    <Show
                        when=move || !event_log.get().is_empty()
                        fallback=|| view! { <p class="empty-state">"No events yet..."</p> }
                    >
                        <For
                            each=move || event_log.get()
                            key=|entry| entry.clone()
                            children=move |entry| {
                                view! {
                                    <div class="log-entry">
                                        {entry}
                                    </div>
                                }
                            }
                        />
                    </Show>
                </div>
            </div>
            
            <div class="demo-section">
                <h2>"Usage Examples"</h2>
                <div class="examples">
                    <div class="example">
                        <strong>"Sales Transaction:"</strong>
                        <code>"@client ABC Corp bought @product Rice 5kg for @amount 1000"</code>
                    </div>
                    <div class="example">
                        <strong>"Invoice with VAT:"</strong>
                        <code>"@invoice to @client Juan with @vat inclusive @amount 5000"</code>
                    </div>
                    <div class="example">
                        <strong>"Payment with Discount:"</strong>
                        <code>"@payment from @client Maria @amount 4500 @discount senior 20%"</code>
                    </div>
                </div>
            </div>
        </div>
    }
}