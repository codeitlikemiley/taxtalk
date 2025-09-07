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
pub struct Command {
    pub name: String,
    pub description: String,
    pub supports_entities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub plural: String,
    pub description: String,
    pub aliases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandsResponse {
    pub commands: Vec<Command>,
    pub entities: Vec<Entity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    pub command: Option<String>,
    pub entity: Option<String>,
    pub requesting_entity: bool,
    pub at_position: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseRequest {
    pub text: String,
}

#[component]
pub fn CommandEntityInput() -> impl IntoView {
    let (input_value, set_input_value) = signal("".to_string());
    let (mode, set_mode) = signal(InputMode::AI);
    let (commands, set_commands) = signal(Vec::<Command>::new());
    let (entities, set_entities) = signal(Vec::<Entity>::new());
    let (parse_result, set_parse_result) = signal(Option::<ParseResult>::None);
    let (show_entity_selector, set_show_entity_selector) = signal(false);
    let (available_entities, set_available_entities) = signal(Vec::<Entity>::new());
    
    // Load commands and entities on mount
    Effect::new(move |_| {
        spawn_local(async move {
            let api_base = option_env!("API_URL").unwrap_or("http://localhost:3000");
            let url = format!("{}/api/v2/commands", api_base);
            
            match reqwest::get(&url).await {
                Ok(resp) => {
                    if let Ok(data) = resp.json::<CommandsResponse>().await {
                        set_commands.set(data.commands);
                        set_entities.set(data.entities);
                    }
                }
                Err(e) => {
                    leptos::logging::log!("Failed to load commands: {:?}", e);
                }
            }
        });
    });
    
    // Parse input as user types
    Effect::new(move |_| {
        let value = input_value.get();
        if value.is_empty() {
            set_parse_result.set(None);
            set_mode.set(InputMode::AI);
            set_show_entity_selector.set(false);
            return;
        }
        
        let api_base = option_env!("API_URL").unwrap_or("http://localhost:3000");
        spawn_local(async move {
            let url = format!("{}/api/v2/parse", api_base);
            let request = ParseRequest { text: value.clone() };
            
            match reqwest::Client::new()
                .post(&url)
                .json(&request)
                .send()
                .await
            {
                Ok(resp) => {
                    if let Ok(result) = resp.json::<ParseResult>().await {
                        // Update mode based on parse result
                        if result.command.is_some() {
                            set_mode.set(InputMode::Command);
                        } else {
                            set_mode.set(InputMode::AI);
                        }
                        
                        // Show entity selector if @ is typed
                        set_show_entity_selector.set(result.requesting_entity);
                        
                        // Update available entities based on command context
                        if result.requesting_entity {
                            if let Some(cmd_name) = &result.command {
                                // Filter entities that this command supports
                                let commands_list = commands.get();
                                let entities_list = entities.get();
                                
                                if let Some(cmd) = commands_list.iter().find(|c| &c.name == cmd_name) {
                                    if cmd.supports_entities.is_empty() {
                                        // Show all entities if no specific ones defined
                                        set_available_entities.set(entities_list);
                                    } else {
                                        // Show only supported entities
                                        let filtered: Vec<Entity> = entities_list
                                            .into_iter()
                                            .filter(|e| cmd.supports_entities.contains(&e.name))
                                            .collect();
                                        set_available_entities.set(filtered);
                                    }
                                } else {
                                    // No command context, show all entities
                                    set_available_entities.set(entities_list);
                                }
                            } else {
                                // No command typed yet, show all entities
                                set_available_entities.set(entities.get());
                            }
                        }
                        
                        set_parse_result.set(Some(result));
                    }
                }
                Err(e) => {
                    leptos::logging::log!("Failed to parse input: {:?}", e);
                }
            }
        });
    });
    
    let handle_keydown = move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            e.prevent_default();
            
            let value = input_value.get();
            let mode_val = mode.get();
            let result = parse_result.get();
            
            if mode_val == InputMode::Command {
                if let Some(parsed) = result {
                    if let Some(cmd) = parsed.command {
                        if let Some(ent) = parsed.entity {
                            leptos::logging::log!("Execute: {} {}", cmd, ent);
                        } else {
                            leptos::logging::log!("Execute: {} (no entity)", cmd);
                        }
                    }
                }
            } else {
                leptos::logging::log!("AI Query: {}", value);
            }
            
            // Clear input
            set_input_value.set("".to_string());
        }
    };
    
    let select_entity = move |entity_name: String| {
        let current = input_value.get();
        // Replace the @ at the end with the entity name
        let new_value = if current.trim_end().ends_with('@') {
            format!("{}{}", current.trim_end_matches('@'), entity_name)
        } else {
            format!("{} {}", current, entity_name)
        };
        set_input_value.set(new_value);
        set_show_entity_selector.set(false);
    };
    
    view! {
        <div class="command-entity-input">
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
                {move || {
                    parse_result.get().and_then(|r| r.command).map(|cmd| view! {
                        <span class="px-3 py-1 bg-blue-500 text-white rounded-full text-sm">
                            "Command: " {cmd}
                        </span>
                    })
                }}
                
                {move || {
                    parse_result.get().and_then(|r| r.entity).map(|ent| view! {
                        <span class="px-3 py-1 bg-green-500 text-white rounded-full text-sm">
                            "Entity: " {ent}
                        </span>
                    })
                }}
            </div>
            
            // Entity selector popup (shown when @ is typed)
            <div
                class=move || if show_entity_selector.get() {
                    "entity-selector mb-2 p-3 bg-gray-100 rounded-lg"
                } else {
                    "hidden"
                }
            >
                {move || {
                    let entities_to_show = available_entities.get();
                    if entities_to_show.is_empty() && show_entity_selector.get() {
                        view! {
                            <div class="text-sm text-gray-500">
                                "Type a command first (create, update, delete, list) then @ to see entities"
                            </div>
                        }.into_any()
                    } else if !entities_to_show.is_empty() && show_entity_selector.get() {
                        view! {
                            <>
                                <div class="text-sm font-semibold mb-2">"Available entities:"</div>
                                <div class="flex flex-wrap gap-2">
                                    {entities_to_show.into_iter().map(|entity| {
                                        let entity_name = entity.name.clone();
                                        view! {
                                            <button
                                                class="px-3 py-1 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 text-sm"
                                                on:click=move |_| select_entity(entity_name.clone())
                                            >
                                                {entity.name.clone()} " (" {entity.description} ")"
                                            </button>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </>
                        }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }}
            </div>
            
            // Input field
            <input
                type="text"
                class="w-full p-3 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-lg"
                placeholder="Type 'create invoice' or 'update @' to see entities..."
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
                <div>"Parse Result: " {move || format!("{:?}", parse_result.get())}</div>
                <div>"Commands loaded: " {move || commands.get().len()}</div>
                <div>"Entities loaded: " {move || entities.get().len()}</div>
            </div>
        </div>
    }
}