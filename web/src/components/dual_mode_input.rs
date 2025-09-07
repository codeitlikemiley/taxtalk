use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use web_sys::KeyboardEvent;

use crate::components::{
    multi_select_combobox::MultiSelectEntityCombobox,
    entity_combobox::EntityCombobox,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandInfo {
    pub command: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub plugin_id: String,
    pub required_tokens: Vec<TokenInfo>,
    pub optional_tokens: Vec<TokenInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub token_type: String,
    pub description: String,
    pub multiple: bool,
    pub entity_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Tokenization,
    AI,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: String,
    pub entity_type: String,
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Clone)]
pub struct CommandToken {
    pub command: String,
    pub display: String,
}

#[derive(Debug, Clone)]
pub struct EntityToken {
    pub entity_type: String,
    pub entity: Entity,
}

#[component]
pub fn DualModeInput(
    on_submit: WriteSignal<String>,
) -> impl IntoView {
    let (input_value, set_input_value) = signal("".to_string());
    let (mode, set_mode) = signal(InputMode::AI);
    let (valid_commands, set_valid_commands) = signal(Vec::<CommandInfo>::new());
    let (current_command, set_current_command) = signal(Option::<CommandInfo>::None);
    let (command_token, set_command_token) = signal(Option::<CommandToken>::None);
    let (entity_tokens, set_entity_tokens) = signal(Vec::<EntityToken>::new());
    let (show_entity_selector, set_show_entity_selector) = signal(false);
    let (current_entity_type, set_current_entity_type) = signal(Option::<String>::None);
    let (current_entity_multiple, set_current_entity_multiple) = signal(false);
    
    // Load valid commands on mount
    Effect::new(move |_| {
        spawn_local(async move {
            match reqwest::get("http://localhost:4000/api/commands").await {
                Ok(resp) => {
                    if let Ok(commands) = resp.json::<Vec<CommandInfo>>().await {
                        set_valid_commands.set(commands);
                    }
                }
                Err(_e) => {
                    // Failed to load commands
                }
            }
        });
    });
    
    // Detect command in input and switch modes
    Effect::new(move |_| {
        let value = input_value.get();
        let commands = valid_commands.get();
        
        // Check if input contains a valid command
        let mut found_command = None;
        for cmd_info in &commands {
            // Check main command
            if value.to_lowercase().contains(&cmd_info.command.to_lowercase()) {
                found_command = Some(cmd_info.clone());
                break;
            }
            // Check aliases
            for alias in &cmd_info.aliases {
                if value.to_lowercase().contains(&alias.to_lowercase()) {
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
            
            // Check if we need entity input
            if value.ends_with('@') {
                // Find what entity type is needed
                for req in &cmd.required_tokens {
                    if req.token_type == "entity_ref" {
                        let entity_type = req.entity_type.clone()
                            .unwrap_or_else(|| infer_entity_type(&req.description));
                        set_current_entity_type.set(Some(entity_type));
                        set_current_entity_multiple.set(req.multiple);
                        set_show_entity_selector.set(true);
                        break;
                    }
                }
            }
        } else {
            // Switch to AI mode
            set_mode.set(InputMode::AI);
            set_current_command.set(None);
            set_command_token.set(None);
        }
    });
    
    let handle_keydown = move |e: KeyboardEvent| {
        if e.key() == "Enter" && (e.meta_key() || e.ctrl_key()) {
            // Cmd+Enter or Ctrl+Enter to submit
            e.prevent_default();
            
            let mode_val = mode.get();
            if mode_val == InputMode::Tokenization {
                // Build tokenized command
                let mut command = String::new();
                
                if let Some(cmd_token) = command_token.get() {
                    command.push_str(&cmd_token.command);
                }
                
                for entity_token in entity_tokens.get() {
                    command.push_str(&format!(" @{} {}", 
                        entity_token.entity_type, 
                        entity_token.entity.name
                    ));
                }
                
                on_submit.set(command);
            } else {
                // AI mode - send raw input
                on_submit.set(input_value.get());
            }
        }
    };
    
    let remove_command_token = move |_| {
        set_command_token.set(None);
        set_current_command.set(None);
        set_mode.set(InputMode::AI);
        set_entity_tokens.set(vec![]);
        set_input_value.set("".to_string());
    };
    
    let remove_entity_token = move |index: usize| {
        set_entity_tokens.update(|tokens| {
            tokens.remove(index);
        });
    };
    
    let handle_entity_selected = move |entities: Vec<Entity>| {
        let entity_type = current_entity_type.get().unwrap_or_default();
        
        for entity in entities {
            set_entity_tokens.update(|tokens| {
                tokens.push(EntityToken {
                    entity_type: entity_type.clone(),
                    entity: entity.clone(),
                });
            });
        }
        
        set_show_entity_selector.set(false);
        set_input_value.set("".to_string());
    };
    
    view! {
        <div class="dual-mode-input">
            {move || match mode.get() {
                InputMode::Tokenization => {
                    view! {
                        <div class="tokenization-mode">
                            <div class="mode-indicator">
                                <span class="mode-badge tokenization">Tokenization Mode</span>
                            </div>
                            
                            // Command and entity tags
                            <div class="token-display">
                                {move || command_token.get().map(|token| {
                                    view! {
                                        <span class="command-tag">
                                            {token.display}
                                            <button 
                                                class="remove-tag"
                                                on:click=remove_command_token
                                            >
                                                "×"
                                            </button>
                                        </span>
                                    }
                                })}
                                
                                {move || entity_tokens.get().iter().enumerate().map(|(i, token)| {
                                    let idx = i;
                                    view! {
                                        <span class="entity-tag">
                                            {format!("@{}: {}", token.entity_type, token.entity.display_name)}
                                            <button 
                                                class="remove-tag"
                                                on:click=move |_| remove_entity_token(idx)
                                            >
                                                "×"
                                            </button>
                                        </span>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                            
                            // Entity selector (shown above input)
                            {move || if show_entity_selector.get() {
                                let entity_type = current_entity_type.get().unwrap_or_default();
                                let multiple = current_entity_multiple.get();
                                
                                view! {
                                    <div class="entity-selector-container">
                                        {if multiple {
                                            view! {
                                                <MultiSelectEntityCombobox
                                                    entity_type=entity_type.clone()
                                                    prompt=format!("Select {}", entity_type)
                                                    max_selections=None
                                                    initial_selections=vec![]
                                                    on_confirm=signal(handle_entity_selected).1
                                                    on_cancel=signal(move |_: bool| {
                                                        set_show_entity_selector.set(false);
                                                    }).1
                                                />
                                            }.into_any()
                                        } else {
                                            view! {
                                                <EntityCombobox
                                                    entity_type=entity_type.clone()
                                                    prompt=format!("Select {}", entity_type)
                                                    required=true
                                                    on_select=signal(move |entity: Entity| {
                                                        handle_entity_selected(vec![entity]);
                                                    }).1
                                                    on_skip=signal(move |_: ()| {
                                                        set_show_entity_selector.set(false);
                                                    }).1
                                                />
                                            }.into_any()
                                        }}
                                    </div>
                                }.into_view()
                            } else {
                                view! { <div></div> }.into_view()
                            }}
                            
                            // Input field
                            <input
                                type="text"
                                class="tokenized-input"
                                placeholder="Type @ to add entities..."
                                value=move || input_value.get()
                                on:input=move |e| {
                                    let value = event_target_value(&e);
                                    set_input_value.set(value);
                                }
                                on:keydown=handle_keydown
                            />
                            
                            <div class="help-text">
                                "Press Cmd+Enter to submit"
                            </div>
                        </div>
                    }.into_view()
                }
                InputMode::AI => {
                    view! {
                        <div class="ai-mode">
                            <div class="mode-indicator">
                                <span class="mode-badge ai">AI Mode</span>
                            </div>
                            
                            <textarea
                                class="ai-input"
                                placeholder="Ask me anything or start typing a command like 'create invoice'..."
                                prop:value=move || input_value.get()
                                on:input=move |e| {
                                    let value = event_target_value(&e);
                                    set_input_value.set(value);
                                }
                                on:keydown=handle_keydown
                            />
                            
                            <div class="help-text">
                                "Type a command to switch to tokenization mode • Press Cmd+Enter to submit"
                            </div>
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

fn infer_entity_type(description: &str) -> String {
    let desc_lower = description.to_lowercase();
    if desc_lower.contains("customer") || desc_lower.contains("client") {
        "client".to_string()
    } else if desc_lower.contains("supplier") || desc_lower.contains("vendor") {
        "supplier".to_string()
    } else if desc_lower.contains("product") || desc_lower.contains("service") {
        "product".to_string()
    } else if desc_lower.contains("employee") {
        "employee".to_string()
    } else {
        "entity".to_string()
    }
}