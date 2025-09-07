use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use web_sys::KeyboardEvent;

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Command,    // User is typing commands
    AI,         // Free-form AI input
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityInfo {
    pub name: String,
    pub plural: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ParsedInput {
    pub command: Option<String>,
    pub entity: Option<String>,
    pub tokens: Vec<String>,
}

#[component]
pub fn SmartInput() -> impl IntoView {
    let (input_value, set_input_value) = signal("".to_string());
    let (mode, set_mode) = signal(InputMode::AI);
    let (detected_command, set_detected_command) = signal(Option::<String>::None);
    let (detected_entity, set_detected_entity) = signal(Option::<String>::None);
    let (show_entity_selector, set_show_entity_selector) = signal(false);
    let (available_entities, set_available_entities) = signal(Vec::<EntityInfo>::new());
    let (available_tokens, set_available_tokens) = signal(Vec::<String>::new());
    
    // Basic command list (hardcoded for now, should come from server)
    let commands = vec!["create", "read", "update", "delete", "list", "search"];
    let entities = vec![
        EntityInfo { name: "invoice".to_string(), plural: "invoices".to_string(), description: "Sales invoice".to_string() },
        EntityInfo { name: "payment".to_string(), plural: "payments".to_string(), description: "Payment record".to_string() },
        EntityInfo { name: "client".to_string(), plural: "clients".to_string(), description: "Customer/Client".to_string() },
        EntityInfo { name: "product".to_string(), plural: "products".to_string(), description: "Product/Service".to_string() },
    ];
    
    // Parse input to detect commands and entities
    Effect::new(move |_| {
        let value = input_value.get();
        let words: Vec<&str> = value.split_whitespace().collect();
        
        // Reset detections
        let mut found_command = None;
        let mut found_entity = None;
        
        // Check for commands (exact word match)
        for word in &words {
            let word_lower = word.to_lowercase();
            if commands.contains(&word_lower.as_str()) {
                found_command = Some(word_lower);
                break;
            }
        }
        
        // Check for entities
        for word in &words {
            let word_lower = word.to_lowercase();
            for entity in &entities {
                if word_lower == entity.name || word_lower == entity.plural {
                    found_entity = Some(entity.name.clone());
                    break;
                }
            }
        }
        
        // Check if user typed @ to show entity selector
        if value.trim_end().ends_with('@') {
            set_show_entity_selector.set(true);
            
            // Filter entities based on command context
            if let Some(cmd) = &found_command {
                // Show relevant entities for this command
                set_available_entities.set(entities.clone());
            }
        } else {
            set_show_entity_selector.set(false);
        }
        
        // Update mode based on what we found
        if found_command.is_some() {
            set_mode.set(InputMode::Command);
            set_detected_command.set(found_command);
            set_detected_entity.set(found_entity);
        } else {
            set_mode.set(InputMode::AI);
            set_detected_command.set(None);
            set_detected_entity.set(None);
        }
    });
    
    let handle_keydown = move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            e.prevent_default();
            
            let value = input_value.get();
            let mode_val = mode.get();
            let command = detected_command.get();
            let entity = detected_entity.get();
            
            // Process based on mode
            if mode_val == InputMode::Command {
                if let Some(cmd) = command {
                    if let Some(ent) = entity {
                        logging::log!("Execute: {} {}", cmd, ent);
                    } else {
                        logging::log!("Execute: {} (no entity)", cmd);
                    }
                }
            } else {
                logging::log!("AI Query: {}", value);
            }
            
            // Clear input
            set_input_value.set("".to_string());
        }
    };
    
    view! {
        <div class="smart-input">
            // Mode indicator
            <div class="mode-indicator mb-2">
                {move || match mode.get() {
                    InputMode::Command => view! {
                        <span class="text-blue-600 font-semibold">
                            "📝 Command Mode"
                        </span>
                    }.into_view(),
                    InputMode::AI => view! {
                        <span class="text-purple-600 font-semibold">
                            "🤖 AI Mode"
                        </span>
                    }.into_view(),
                }}
            </div>
            
            // Visual tags for detected command and entity
            <div class="detected-tokens mb-2 flex gap-2">
                {move || detected_command.get().map(|cmd| view! {
                    <span class="px-3 py-1 bg-blue-500 text-white rounded-full text-sm">
                        "Command: " {cmd}
                    </span>
                })}
                
                {move || detected_entity.get().map(|ent| view! {
                    <span class="px-3 py-1 bg-green-500 text-white rounded-full text-sm">
                        "Entity: " {ent}
                    </span>
                })}
            </div>
            
            // Entity selector popup (shown when @ is typed)
            {move || if show_entity_selector.get() {
                view! {
                    <div class="entity-selector mb-2 p-3 bg-gray-100 rounded-lg">
                        <div class="text-sm font-semibold mb-2">"Available entities:"</div>
                        <div class="flex flex-wrap gap-2">
                            {available_entities.get().into_iter().map(|entity| {
                                view! {
                                    <button
                                        class="px-3 py-1 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 text-sm"
                                        on:click=move |_| {
                                            let current = input_value.get();
                                            let new_value = current.trim_end_matches('@').to_string() + &entity.name;
                                            set_input_value.set(new_value);
                                            set_show_entity_selector.set(false);
                                        }
                                    >
                                        {entity.name} " (" {entity.description} ")"
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }.into_view()
            } else {
                view! { <div /> }.into_view()
            }}
            
            // Input field
            <input
                type="text"
                class="w-full p-3 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-lg"
                placeholder="Type 'create invoice' or 'update payment @' to see entities..."
                prop:value=move || input_value.get()
                on:input=move |e| {
                    let value = event_target_value(&e);
                    set_input_value.set(value);
                }
                on:keydown=handle_keydown
            />
            
            // Help text
            <div class="help-text text-xs text-gray-500 mt-2">
                "Commands: create, read, update, delete, list, search | Type @ to see available entities | Press Enter to submit"
            </div>
            
            // Debug info
            <div class="mt-4 p-3 bg-gray-50 rounded text-xs text-gray-600">
                <div>"Mode: " {move || format!("{:?}", mode.get())}</div>
                <div>"Command: " {move || format!("{:?}", detected_command.get())}</div>
                <div>"Entity: " {move || format!("{:?}", detected_entity.get())}</div>
            </div>
        </div>
    }
}