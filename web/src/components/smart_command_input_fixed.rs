use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use web_sys::KeyboardEvent;
use crate::config;

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
pub struct EntityType {
    pub name: String,
    pub plural: String,
    pub description: String,
    pub aliases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityInstance {
    pub id: String,
    pub name: String,
    pub entity_type: String,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandsResponse {
    pub commands: Vec<Command>,
    pub entities: Vec<EntityType>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub q: String,
    #[serde(rename = "type")]
    pub entity_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SelectedToken {
    pub token_type: String,
    pub instance: EntityInstance,
}

#[component]
pub fn SmartCommandInput() -> impl IntoView {
    let (input_value, set_input_value) = signal("".to_string());
    let (mode, set_mode) = signal(InputMode::AI);
    let (commands, set_commands) = signal(Vec::<Command>::new());
    let (entity_types, set_entity_types) = signal(Vec::<EntityType>::new());
    let (parse_result, set_parse_result) = signal(Option::<ParseResult>::None);
    let (selected_tokens, set_selected_tokens) = signal(Vec::<SelectedToken>::new());
    
    // Load commands on mount - run only once
    {
        let set_commands = set_commands.clone();
        let set_entity_types = set_entity_types.clone();
        spawn_local(async move {
            let url = config::api_url("api/v2/commands");
            match reqwest::get(&url).await {
                Ok(resp) => {
                    if let Ok(data) = resp.json::<CommandsResponse>().await {
                        set_commands.set(data.commands);
                        set_entity_types.set(data.entities);
                    }
                }
                Err(e) => {
                    leptos::logging::log!("Failed to load commands: {:?}", e);
                }
            }
        });
    }
    
    let handle_input = move |e: web_sys::Event| {
        let value = event_target_value(&e);
        set_input_value.set(value.clone());
        
        // Simple parsing logic without async calls for now
        if value.contains("create") || value.contains("update") || value.contains("delete") {
            set_mode.set(InputMode::Command);
        } else {
            set_mode.set(InputMode::AI);
        }
    };
    
    let handle_keydown = move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            e.prevent_default();
            let value = input_value.get();
            let mode_val = mode.get();
            
            if mode_val == InputMode::Command {
                leptos::logging::log!("Execute command: {}", value);
            } else {
                leptos::logging::log!("AI Query: {}", value);
            }
            
            set_input_value.set("".to_string());
            set_selected_tokens.set(vec![]);
        }
    };
    
    let remove_token = move |index: usize| {
        let mut tokens = selected_tokens.get();
        if index < tokens.len() {
            tokens.remove(index);
            set_selected_tokens.set(tokens);
        }
    };
    
    view! {
        <div class="smart-command-input">
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
            
            // Visual tags for detected command
            <div class="detected-tokens mb-2 flex gap-2 flex-wrap">
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
                
                // Selected entity tokens
                {move || {
                    selected_tokens.get().into_iter().enumerate().map(|(i, token)| {
                        view! {
                            <span class="px-3 py-1 bg-purple-500 text-white rounded-full text-sm inline-flex items-center gap-1">
                                {token.token_type.clone()} ": " {token.instance.name.clone()}
                                <button
                                    class="ml-1 hover:bg-purple-600 rounded-full w-4 h-4 flex items-center justify-center"
                                    on:click=move |_| remove_token(i)
                                >
                                    "×"
                                </button>
                            </span>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
            
            // Input field
            <input
                type="text"
                class="w-full p-3 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-lg"
                placeholder="Type 'create invoice' or ask a question..."
                prop:value=move || input_value.get()
                on:input=handle_input
                on:keydown=handle_keydown
            />
            
            // Help text
            <div class="help-text text-xs text-gray-500 mt-2">
                "Commands: create, read, update, delete | Press Enter to submit"
            </div>
            
            // Debug info
            <div class="mt-4 p-3 bg-gray-50 rounded text-xs text-gray-600">
                <div>"Mode: " {move || format!("{:?}", mode.get())}</div>
                <div>"Commands loaded: " {move || commands.get().len()}</div>
                <div>"Entity types loaded: " {move || entity_types.get().len()}</div>
            </div>
        </div>
    }
}