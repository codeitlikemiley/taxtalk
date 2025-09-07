use leptos::prelude::*;
use leptos::logging;
use web_sys::KeyboardEvent;
use crate::types::*;
use crate::storage::InputStorage;

#[component]
pub fn DualModeInput<F>(
    #[prop(optional)] initial_mode: Option<InputMode>,
    #[prop(optional)] on_submit: Option<F>,
    #[prop(optional)] show_history: bool,
    #[prop(optional)] persist_mode: bool,
) -> impl IntoView
where
    F: Fn(String, InputMode) + 'static + Clone,
{
    // Load persisted mode or use initial/default
    let starting_mode = if persist_mode {
        InputStorage::load_mode().or(initial_mode).unwrap_or(InputMode::AI)
    } else {
        initial_mode.unwrap_or(InputMode::AI)
    };
    
    let (input_value, set_input_value) = signal(String::new());
    let (mode, set_mode) = signal(starting_mode);
    let (valid_commands, _set_valid_commands) = signal(CommandInfo::default_commands());
    let (current_command, set_current_command) = signal::<Option<CommandInfo>>(None);
    let (command_token, set_command_token) = signal::<Option<CommandToken>>(None);
    let (entity_tokens, set_entity_tokens) = signal::<Vec<EntityToken>>(Vec::new());
    let (show_entity_selector, set_show_entity_selector) = signal(false);
    let (current_entity_type, set_current_entity_type) = signal::<Option<String>>(None);
    let (history, set_history) = signal(InputStorage::get_history());
    let (show_history_panel, set_show_history_panel) = signal(false);
    let (history_index, set_history_index) = signal::<Option<usize>>(None);
    
    // Save mode when it changes
    create_effect(move |_| {
        if persist_mode {
            InputStorage::save_mode(&mode.get());
        }
    });
    
    // Detect command in input and switch modes
    create_effect(move |_| {
        let value = input_value.get();
        let commands = valid_commands.get();
        
        // Don't parse if we already have a command token
        if command_token.get().is_some() {
            // Check for @ to trigger entity selection
            if value.ends_with('@') {
                if let Some(ref cmd) = current_command.get() {
                    for req in &cmd.required_tokens {
                        if req.token_type == "entity_ref" && entity_tokens.get().is_empty() {
                            set_current_entity_type.set(req.entity_type.clone().or(Some("entity".to_string())));
                            set_show_entity_selector.set(true);
                            break;
                        }
                    }
                }
            }
            return;
        }
        
        // Check if input contains a valid command
        let mut found_command = None;
        for cmd_info in &commands {
            // Check main command
            if value.to_lowercase().starts_with(&cmd_info.command.to_lowercase()) ||
               value.to_lowercase().contains(&format!(" {}", cmd_info.command.to_lowercase())) {
                found_command = Some(cmd_info.clone());
                break;
            }
            // Check aliases
            for alias in &cmd_info.aliases {
                if value.to_lowercase().starts_with(&alias.to_lowercase()) ||
                   value.to_lowercase().contains(&format!(" {}", alias.to_lowercase())) {
                    found_command = Some(cmd_info.clone());
                    break;
                }
            }
        }
        
        if let Some(cmd) = found_command {
            // Switch to tokenization mode
            set_mode.set(InputMode::Tokenization);
            set_current_command.set(Some(cmd.clone()));
            
            // Create command token
            set_command_token.set(Some(CommandToken {
                command: cmd.command.clone(),
                display: cmd.command.clone(),
            }));
            
            // Clear input after extracting command
            set_input_value.set(String::new());
        }
    });
    
    let handle_keydown = move |e: KeyboardEvent| {
        let key = e.key();
        
        match key.as_str() {
            "Enter" => {
                if e.shift_key() {
                    // Shift+Enter for new line in AI mode
                    if mode.get() == InputMode::AI {
                        return;
                    }
                }
                
                if e.meta_key() || e.ctrl_key() {
                    // Cmd+Enter or Ctrl+Enter to submit
                    e.prevent_default();
                    submit_input(&input_value, &mode, &command_token, &entity_tokens, &on_submit, &set_history);
                    clear_input(&set_input_value, &set_command_token, &set_entity_tokens, &set_current_command, &set_mode);
                }
            },
            "ArrowUp" => {
                if show_history && (e.meta_key() || e.ctrl_key()) {
                    e.prevent_default();
                    navigate_history(true, &history, &history_index, &set_history_index, &set_input_value);
                }
            },
            "ArrowDown" => {
                if show_history && (e.meta_key() || e.ctrl_key()) {
                    e.prevent_default();
                    navigate_history(false, &history, &history_index, &set_history_index, &set_input_value);
                }
            },
            "Escape" => {
                if show_entity_selector.get() {
                    e.prevent_default();
                    set_show_entity_selector.set(false);
                } else if command_token.get().is_some() {
                    e.prevent_default();
                    clear_input(&set_input_value, &set_command_token, &set_entity_tokens, &set_current_command, &set_mode);
                }
            },
            _ => {}
        }
    };
    
    let remove_command_token = move |_| {
        set_command_token.set(None);
        set_current_command.set(None);
        set_mode.set(InputMode::AI);
        set_entity_tokens.set(vec![]);
        set_input_value.set(String::new());
    };
    
    let remove_entity_token = move |index: usize| {
        set_entity_tokens.update(|tokens| {
            if index < tokens.len() {
                tokens.remove(index);
            }
        });
    };
    
    let add_sample_entity = move |entity_type: &str, name: &str| {
        let entity = Entity {
            id: format!("sample_{}", chrono::Utc::now().timestamp_millis()),
            entity_type: entity_type.to_string(),
            name: name.to_string(),
            display_name: name.to_string(),
        };
        
        set_entity_tokens.update(|tokens| {
            tokens.push(EntityToken {
                entity_type: entity_type.to_string(),
                entity,
            });
        });
        
        set_show_entity_selector.set(false);
        set_input_value.set(String::new());
    };
    
    let toggle_mode = move |_| {
        let new_mode = match mode.get() {
            InputMode::AI => InputMode::Tokenization,
            InputMode::Tokenization => InputMode::AI,
        };
        set_mode.set(new_mode);
        
        // Clear tokenization state when switching to AI
        if new_mode == InputMode::AI {
            set_command_token.set(None);
            set_entity_tokens.set(vec![]);
            set_current_command.set(None);
        }
    };
    
    let use_history_item = move |item: String| {
        set_input_value.set(item);
        set_show_history_panel.set(false);
        set_history_index.set(None);
    };
    
    view! {
        <div class="dmi-container">
            <style>{include_str!("styles.css")}</style>
            
            // Mode Toggle
            <div class="dmi-header">
                <button
                    class="dmi-mode-toggle"
                    on:click=toggle_mode
                >
                    <span class=move || if mode.get() == InputMode::AI {
                        "dmi-mode-active"
                    } else {
                        "dmi-mode-inactive"
                    }>
                        "🤖 AI Mode"
                    </span>
                    <span class="dmi-mode-separator">" | "</span>
                    <span class=move || if mode.get() == InputMode::Tokenization {
                        "dmi-mode-active"
                    } else {
                        "dmi-mode-inactive"
                    }>
                        "⚡ Token Mode"
                    </span>
                </button>
                
                <Show when=move || show_history>
                    <button
                        class="dmi-history-toggle"
                        on:click=move |_| set_show_history_panel.update(|v| *v = !*v)
                    >
                        "📜 History"
                    </button>
                </Show>
            </div>
            
            // Current mode display
            {move || match mode.get() {
                InputMode::Tokenization => view! {
                    <div class="dmi-tokenization-mode">
                        // Token display
                        <div class="dmi-token-display">
                            <Show when=move || command_token.get().is_some()>
                                {move || command_token.get().map(|token| view! {
                                    <span class="dmi-command-tag">
                                        <span class="dmi-tag-icon">"⚡"</span>
                                        {token.display}
                                        <button 
                                            class="dmi-remove-tag"
                                            on:click=remove_command_token
                                        >
                                            "×"
                                        </button>
                                    </span>
                                })}
                            </Show>
                            
                            <For
                                each=move || entity_tokens.get().into_iter().enumerate()
                                key=|(i, _)| *i
                                children=move |(i, token)| {
                                    view! {
                                        <span class=move || format!("dmi-entity-tag dmi-entity-{}", token.entity_type)>
                                            <span class="dmi-tag-icon">"@"</span>
                                            {format!("{}: {}", token.entity_type, token.entity.display_name)}
                                            <button 
                                                class="dmi-remove-tag"
                                                on:click=move |_| remove_entity_token(i)
                                            >
                                                "×"
                                            </button>
                                        </span>
                                    }
                                }
                            />
                        </div>
                        
                        // Entity selector
                        <Show when=move || show_entity_selector.get()>
                            <div class="dmi-entity-selector">
                                <div class="dmi-entity-header">
                                    "Select " {move || current_entity_type.get().unwrap_or_else(|| "entity".to_string())}":"
                                </div>
                                <div class="dmi-entity-options">
                                    <button
                                        class="dmi-entity-option"
                                        on:click=move |_| add_sample_entity("client", "Juan Dela Cruz")
                                    >
                                        "👤 Juan Dela Cruz"
                                    </button>
                                    <button
                                        class="dmi-entity-option"
                                        on:click=move |_| add_sample_entity("client", "ABC Corporation")
                                    >
                                        "🏢 ABC Corporation"
                                    </button>
                                    <button
                                        class="dmi-entity-option"
                                        on:click=move |_| add_sample_entity("supplier", "Office Depot")
                                    >
                                        "📦 Office Depot"
                                    </button>
                                    <button
                                        class="dmi-entity-option"
                                        on:click=move |_| add_sample_entity("product", "Coffee 1kg")
                                    >
                                        "☕ Coffee 1kg"
                                    </button>
                                </div>
                                <div class="dmi-entity-hint">
                                    "Press Esc to cancel"
                                </div>
                            </div>
                        </Show>
                        
                        // Input field
                        <input
                            type="text"
                            class="dmi-input dmi-tokenized-input"
                            placeholder=move || if command_token.get().is_some() {
                                "Type @ to add entities, or type parameters..."
                            } else {
                                "Start with a command: create, invoice, pay, report..."
                            }
                            prop:value=move || input_value.get()
                            on:input=move |e| {
                                set_input_value.set(event_target_value(&e));
                            }
                            on:keydown=handle_keydown
                        />
                    </div>
                }.into_view(),
                
                InputMode::AI => view! {
                    <div class="dmi-ai-mode">
                        <textarea
                            class="dmi-input dmi-ai-input"
                            placeholder="Ask anything or type commands like 'create invoice' to switch modes..."
                            prop:value=move || input_value.get()
                            on:input=move |e| {
                                set_input_value.set(event_target_value(&e));
                            }
                            on:keydown=handle_keydown
                            rows="3"
                        />
                    </div>
                }.into_view(),
            }}
            
            // History Panel
            <Show when=move || show_history && show_history_panel.get()>
                <div class="dmi-history-panel">
                    <div class="dmi-history-header">
                        <span>"Recent History"</span>
                        <button
                            class="dmi-clear-history"
                            on:click=move |_| {
                                InputStorage::clear_history();
                                set_history.set(vec![]);
                            }
                        >
                            "Clear"
                        </button>
                    </div>
                    <div class="dmi-history-list">
                        <For
                            each=move || history.get()
                            key=|item| item.clone()
                            children=move |item| {
                                let item_clone = item.clone();
                                view! {
                                    <button
                                        class="dmi-history-item"
                                        on:click=move |_| use_history_item(item_clone.clone())
                                    >
                                        {item}
                                    </button>
                                }
                            }
                        />
                    </div>
                </div>
            </Show>
            
            // Help text
            <div class="dmi-help">
                <span class="dmi-help-item">
                    <kbd>"Cmd+Enter"</kbd>" submit"
                </span>
                <Show when=move || mode.get() == InputMode::AI>
                    <span class="dmi-help-item">
                        <kbd>"Shift+Enter"</kbd>" new line"
                    </span>
                </Show>
                <Show when=move || show_history>
                    <span class="dmi-help-item">
                        <kbd>"Cmd+↑/↓"</kbd>" history"
                    </span>
                </Show>
                <span class="dmi-help-item">
                    <kbd>"Esc"</kbd>" clear"
                </span>
            </div>
        </div>
    }
}

fn submit_input<F>(
    input_value: &ReadSignal<String>,
    mode: &ReadSignal<InputMode>,
    command_token: &ReadSignal<Option<CommandToken>>,
    entity_tokens: &ReadSignal<Vec<EntityToken>>,
    on_submit: &Option<F>,
    set_history: &WriteSignal<Vec<String>>,
) where
    F: Fn(String, InputMode) + 'static + Clone,
{
    let mode_val = mode.get();
    let submission = if mode_val == InputMode::Tokenization {
        // Build tokenized command
        let mut parts = Vec::new();
        
        if let Some(cmd) = command_token.get() {
            parts.push(cmd.command);
        }
        
        for token in entity_tokens.get() {
            parts.push(format!("@{} {}", token.entity_type, token.entity.name));
        }
        
        let additional = input_value.get();
        if !additional.is_empty() {
            parts.push(additional);
        }
        
        parts.join(" ")
    } else {
        input_value.get()
    };
    
    if !submission.is_empty() {
        // Save to history
        InputStorage::save_to_history(&submission);
        set_history.update(|h| {
            h.insert(0, submission.clone());
            if h.len() > 50 {
                h.truncate(50);
            }
        });
        
        // Call submit handler
        if let Some(ref on_submit_fn) = on_submit {
            on_submit_fn(submission, mode_val);
        }
        
        logging::log!("Submitted ({}): {}", 
            if mode_val == InputMode::AI { "AI" } else { "Token" },
            submission
        );
    }
}

fn clear_input(
    set_input_value: &WriteSignal<String>,
    set_command_token: &WriteSignal<Option<CommandToken>>,
    set_entity_tokens: &WriteSignal<Vec<EntityToken>>,
    set_current_command: &WriteSignal<Option<CommandInfo>>,
    set_mode: &WriteSignal<InputMode>,
) {
    set_input_value.set(String::new());
    set_command_token.set(None);
    set_entity_tokens.set(vec![]);
    set_current_command.set(None);
    set_mode.set(InputMode::AI);
}

fn navigate_history(
    up: bool,
    history: &ReadSignal<Vec<String>>,
    history_index: &ReadSignal<Option<usize>>,
    set_history_index: &WriteSignal<Option<usize>>,
    set_input_value: &WriteSignal<String>,
) {
    let history_items = history.get();
    if history_items.is_empty() {
        return;
    }
    
    let new_index = if up {
        match history_index.get() {
            None => Some(0),
            Some(i) if i < history_items.len() - 1 => Some(i + 1),
            Some(i) => Some(i),
        }
    } else {
        match history_index.get() {
            Some(0) => None,
            Some(i) => Some(i - 1),
            None => None,
        }
    };
    
    set_history_index.set(new_index);
    
    if let Some(idx) = new_index {
        if let Some(item) = history_items.get(idx) {
            set_input_value.set(item.clone());
        }
    } else {
        set_input_value.set(String::new());
    }
}