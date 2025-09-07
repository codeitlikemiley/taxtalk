use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use web_sys::{KeyboardEvent, HtmlInputElement};
use crate::config;
use leptos::html::Input;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

fn request_animation_frame(f: impl FnOnce() + 'static) {
    let closure = Closure::once(Box::new(f) as Box<dyn FnOnce()>);
    web_sys::window()
        .unwrap()
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
}

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
    #[serde(default)]
    pub cardinality: TokenCardinality,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TokenCardinality {
    Single,
    Multiple,
    #[serde(rename = "exact")]
    Exact(usize),
}

impl Default for TokenCardinality {
    fn default() -> Self {
        TokenCardinality::Single
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRequirement {
    pub token_type: String,
    pub description: String,
    pub required: bool,
    #[serde(default)]
    pub cardinality: TokenCardinality,
    pub default_value: Option<serde_json::Value>,
    #[serde(default)]
    pub component: ComponentType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ComponentType {
    #[serde(rename = "entity-selector")]
    EntitySelector,
    #[serde(rename = "text-input")]
    TextInput,
    #[serde(rename = "number-input")]
    NumberInput,
    #[serde(rename = "amount-input")]
    AmountInput,
    #[serde(rename = "date-picker")]
    DatePicker,
    #[serde(rename = "date-range")]
    DateRange,
    #[serde(rename = "select")]
    Select(Vec<String>),
    #[serde(rename = "multi-select")]
    MultiSelect(Vec<String>),
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "file-upload")]
    FileUpload,
}

impl Default for ComponentType {
    fn default() -> Self {
        ComponentType::EntitySelector
    }
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
    #[serde(default)]
    pub requirements: Vec<TokenRequirement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub token_type: String,  // "client", "product", etc.
    pub instance: EntityInstance,
}

#[derive(Debug, Clone, PartialEq)]
enum SelectionMode {
    None,
    TokenType,      // Selecting token type (client, product, etc.)
    EntityInstance, // Selecting actual entity instance
    DirectInput,    // Direct input for amounts, dates, text, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EntitySearchRequest {
    q: String,
    #[serde(rename = "type")]
    entity_type: String,
}

#[component]
pub fn SmartCommandInput() -> impl IntoView {
    let (input_value, set_input_value) = signal("".to_string());
    let (mode, set_mode) = signal(InputMode::AI);
    let (commands, set_commands) = signal(Vec::<Command>::new());
    let (entity_types, set_entity_types) = signal(Vec::<EntityType>::new());
    let (parse_result, set_parse_result) = signal(Option::<ParseResult>::None);
    let (selection_mode, set_selection_mode) = signal(SelectionMode::None);
    let (available_token_types, set_available_token_types) = signal(Vec::<String>::new());
    let (selected_token_type, set_selected_token_type) = signal(Option::<String>::None);
    let (entity_instances, set_entity_instances) = signal(Vec::<EntityInstance>::new());
    let (selected_tokens, set_selected_tokens) = signal(Vec::<SelectedToken>::new());
    let (loading_entities, set_loading_entities) = signal(false);
    let (entity_search_query, set_entity_search_query) = signal(String::new());
    let (filtered_entities, set_filtered_entities) = signal(Vec::<EntityInstance>::new());
    let (token_type_search, set_token_type_search) = signal(String::new());
    let (filtered_token_types, set_filtered_token_types) = signal(Vec::<String>::new());
    let (selected_index, set_selected_index) = signal(0usize);
    let (requirements, set_requirements) = signal(Vec::<TokenRequirement>::new());
    let (current_cardinality, set_current_cardinality) = signal(TokenCardinality::Single);
    let (current_component_type, set_current_component_type) = signal(ComponentType::EntitySelector);
    let (show_create_new, set_show_create_new) = signal(false);
    let entity_search_ref = NodeRef::<Input>::new();
    let token_search_ref = NodeRef::<Input>::new();
    let main_input_ref = NodeRef::<Input>::new();
    
    // Macro to close entity selector to avoid code duplication
    macro_rules! close_entity_selector {
        () => {
            {
                let current = input_value.get();
                
                // Clean up the @ from input and clear it since tokens are resolved
                if let Some(at_pos) = current.rfind('@') {
                    let before_at = &current[..at_pos];
                    // Clear the input after @ since we have the tokens now
                    set_input_value.set(before_at.trim().to_string());
                }
                
                // Keep the command text, just remove the @ part
                // Don't clear completely to preserve context
                
                set_selection_mode.set(SelectionMode::None);
                set_available_token_types.set(vec![]);
                set_filtered_token_types.set(vec![]);
                set_selected_token_type.set(None);
                set_entity_instances.set(vec![]);
                set_entity_search_query.set(String::new());
                set_token_type_search.set(String::new());
                set_selected_index.set(0);
            }
        };
    }
    
    // Load commands and entity types on mount - only run once
    spawn_local(async move {
        let url = config::api_url("api/v2/commands");
        
        match reqwest::get(&url).await {
            Ok(resp) => {
                if let Ok(data) = resp.json::<CommandsResponse>().await {
                    set_commands.set(data.commands);
                    set_entity_types.set(data.entities);
                    set_requirements.set(data.requirements);
                }
            }
            Err(e) => {
                leptos::logging::log!("Failed to load commands: {:?}", e);
            }
        }
    });
    
    // Create a memo for parsing instead of Effect to avoid infinite loops
    let parsed_input = Memo::new(move |_| {
        let value = input_value.get();
        
        // Simple parsing logic
        let mut command = None;
        let mut entity = None;
        let requesting_entity = value.contains('@');
        let at_position = value.rfind('@');
        
        // Check for commands
        if value.contains("create") {
            command = Some("create".to_string());
        } else if value.contains("update") {
            command = Some("update".to_string());
        } else if value.contains("delete") {
            command = Some("delete".to_string());
        } else if value.contains("read") || value.contains("list") {
            command = Some("list".to_string());
        } else if value.contains("search") {
            command = Some("search".to_string());
        }
        
        // Check for entities
        if value.contains("invoice") {
            entity = Some("invoice".to_string());
        } else if value.contains("payment") {
            entity = Some("payment".to_string());
        } else if value.contains("expense") {
            entity = Some("expense".to_string());
        } else if value.contains("client") || value.contains("customer") {
            entity = Some("client".to_string());
        } else if value.contains("product") {
            entity = Some("product".to_string());
        }
        
        ParseResult {
            command,
            entity,
            requesting_entity,
            at_position,
        }
    });
    
    // Watch for @ trigger
    Effect::new(move |_| {
        let value = input_value.get();
        let current_mode = selection_mode.get();
        
        // Only trigger if we're not already in selection mode
        if current_mode == SelectionMode::None && value.ends_with('@') {
            // Parse to get context
            let parsed = parsed_input.get();
            
            // Get available token types based on context
            let types = get_available_token_types(&parsed);
            set_available_token_types.set(types.clone());
            set_filtered_token_types.set(types);
            set_token_type_search.set(String::new());
            set_selected_index.set(0);
            set_selection_mode.set(SelectionMode::TokenType);
        }
    });
    
    // Update parse result and mode based on memo
    Effect::new(move |_| {
        let parsed = parsed_input.get();
        set_parse_result.set(Some(parsed.clone()));
        
        // Determine mode based on content
        if parsed.command.is_some() || parsed.entity.is_some() {
            set_mode.set(InputMode::Command);
        } else {
            set_mode.set(InputMode::AI);
        }
    });
    
    // Handle focus when selection mode changes
    Effect::new(move |_| {
        let mode = selection_mode.get();
        
        request_animation_frame(move || {
            match mode {
                SelectionMode::TokenType => {
                    if let Some(input) = token_search_ref.get() {
                        let _ = input.focus();
                    }
                },
                SelectionMode::EntityInstance => {
                    if let Some(input) = entity_search_ref.get() {
                        let _ = input.focus();
                    }
                },
                SelectionMode::DirectInput => {
                    // Focus stays on main input for direct input
                    if let Some(input) = main_input_ref.get() {
                        let _ = input.focus();
                    }
                },
                SelectionMode::None => {
                    // Return focus to main input
                    if let Some(input) = main_input_ref.get() {
                        let _ = input.focus();
                    }
                }
            }
        });
    });
    
    let handle_keydown = move |e: KeyboardEvent| {
        let is_cmd_or_alt = if cfg!(target_os = "macos") {
            e.meta_key()
        } else {
            e.alt_key()
        };
        
        if e.key() == "Enter" {
            if is_cmd_or_alt {
                e.prevent_default();
                
                if selection_mode.get() != SelectionMode::None {
                    // Close entity selector with Cmd+Enter/Alt+Enter
                    close_entity_selector!();
                } else {
                    // Submit command with Cmd+Enter/Alt+Enter when complete
                    let value = input_value.get();
                    let mode_val = mode.get();
                    let result = parse_result.get();
                    let tokens = selected_tokens.get();
                    
                    if mode_val == InputMode::Command {
                        if let Some(parsed) = result {
                            // Check if all required tokens are provided
                            // TODO: Validate against requirements
                            leptos::logging::log!("Execute command: {:?} with tokens: {:?}", parsed, tokens);
                        }
                    } else {
                        leptos::logging::log!("AI Query: {}", value);
                    }
                    
                    // Clear input and tokens after submission
                    set_input_value.set("".to_string());
                    set_selected_tokens.set(vec![]);
                }
            } else if selection_mode.get() == SelectionMode::None {
                // Regular Enter key without modifier
                e.prevent_default();
                // Don't submit, just prevent newline
            }
        } else if e.key() == "Escape" {
            set_selection_mode.set(SelectionMode::None);
            set_available_token_types.set(vec![]);
            set_filtered_token_types.set(vec![]);
            set_selected_token_type.set(None);
            set_entity_instances.set(vec![]);
            set_entity_search_query.set(String::new());
            set_token_type_search.set(String::new());
            set_selected_index.set(0);
        }
    };
    
    let select_token_type = move |token_type: String| {
        // Check cardinality and component type for this token
        let entities = entity_types.get();
        let entity = entities.iter()
            .find(|e| e.name == token_type);
            
        let cardinality = entity.map(|e| e.cardinality.clone()).unwrap_or_default();
        
        // Determine component type based on token type
        let component_type = match token_type.as_str() {
            "amount" | "money" => ComponentType::AmountInput,
            "date" => ComponentType::DatePicker,
            "date_range" => ComponentType::DateRange,
            "text" | "description" | "note" => ComponentType::TextInput,
            "number" | "quantity" => ComponentType::NumberInput,
            "boolean" | "yes_no" => ComponentType::Boolean,
            _ => ComponentType::EntitySelector,
        };
        
        set_current_cardinality.set(cardinality);
        set_current_component_type.set(component_type.clone());
        set_selected_token_type.set(Some(token_type.clone()));
        
        // Only switch to entity selection if it's an entity selector
        if matches!(component_type, ComponentType::EntitySelector) {
            set_selection_mode.set(SelectionMode::EntityInstance);
            set_entity_search_query.set(String::new());
            set_selected_index.set(0);
            set_loading_entities.set(true);
        } else {
            // For direct input types, switch to appropriate input mode
            set_selection_mode.set(SelectionMode::DirectInput);
        }
        
        spawn_local(async move {
            let search_url = config::api_url("api/search");
            let search_req = SearchRequest {
                q: String::new(),
                entity_type: Some(token_type),
            };
            
            match reqwest::Client::new()
                .get(&search_url)
                .query(&search_req)
                .send()
                .await
            {
                Ok(search_resp) => {
                    if let Ok(search_data) = search_resp.json::<SearchResponse>().await {
                        // Remove duplicates by ID
                        let mut seen = std::collections::HashSet::new();
                        let unique_results: Vec<EntityInstance> = search_data.results
                            .into_iter()
                            .filter(|e| seen.insert(e.id.clone()))
                            .collect();
                        
                        set_entity_instances.set(unique_results.clone());
                        set_filtered_entities.set(unique_results);
                    }
                }
                Err(e) => {
                    leptos::logging::log!("Failed to search entities: {:?}", e);
                }
            }
            set_loading_entities.set(false);
        });
    };
    
    let select_entity = move |instance: EntityInstance| {
        let token_type = selected_token_type.get().unwrap_or_else(|| instance.entity_type.clone());
        let cardinality = current_cardinality.get();
        
        // Add to selected tokens
        let mut tokens = selected_tokens.get();
        tokens.push(SelectedToken {
            token_type: token_type.clone(),
            instance: instance.clone(),
        });
        set_selected_tokens.set(tokens.clone());
        
        // Clear the search
        set_entity_search_query.set(String::new());
        
        // Check if we should auto-close based on cardinality
        let selected_count = tokens.iter()
            .filter(|t| t.token_type == token_type)
            .count();
        
        match cardinality {
            TokenCardinality::Single => {
                // Auto-close after selecting one
                close_entity_selector!();
            },
            TokenCardinality::Exact(n) if selected_count >= n => {
                // Auto-close when exact number reached
                close_entity_selector!();
            },
            _ => {
                // Keep open for multiple selection
            }
        }
        
        // Re-filter to show remaining entities (excluding already selected ones)
        let all_entities = entity_instances.get();
        let selected_ids: std::collections::HashSet<String> = tokens
            .iter()
            .filter(|t| t.token_type == token_type)
            .map(|t| t.instance.id.clone())
            .collect();
        
        let filtered: Vec<EntityInstance> = all_entities.into_iter()
            .filter(|entity| !selected_ids.contains(&entity.id))
            .collect();
        set_filtered_entities.set(filtered);
        set_selected_index.set(0);
    };
    
    
    let remove_token = move |index: usize| {
        let mut tokens = selected_tokens.get();
        if index < tokens.len() {
            tokens.remove(index);
            set_selected_tokens.set(tokens);
        }
    };
    
    // Calculate progress for required tokens
    let progress_memo = Memo::new(move |_| {
        let reqs = requirements.get();
        let tokens = selected_tokens.get();
        let parsed = parse_result.get();
        
        // Filter for required tokens based on current command/entity
        let required: Vec<_> = reqs.iter()
            .filter(|r| r.required && r.default_value.is_none())
            .collect();
        
        let total_required = required.len();
        let provided = tokens.len(); // Simplified - should match against required types
        
        (provided, total_required)
    });
    
    view! {
        <div class="smart-command-input">
            // Mode and Progress indicator
            <div class="flex items-center justify-between mb-2">
                <div class="mode-indicator">
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
                
                // Progress indicator
                {move || {
                    let (provided, required) = progress_memo.get();
                    if mode.get() == InputMode::Command && required > 0 {
                        let percentage = (provided as f64 / required as f64 * 100.0) as u32;
                        let is_complete = provided >= required;
                        
                        view! {
                            <div class="flex items-center gap-2">
                                <div class="relative w-10 h-10">
                                    <svg class="w-10 h-10 transform -rotate-90">
                                        <circle
                                            cx="20"
                                            cy="20"
                                            r="16"
                                            stroke="currentColor"
                                            stroke-width="3"
                                            fill="none"
                                            class="text-gray-300"
                                        />
                                        <circle
                                            cx="20"
                                            cy="20"
                                            r="16"
                                            stroke="currentColor"
                                            stroke-width="3"
                                            fill="none"
                                            stroke-dasharray={format!("{} {}", percentage, 100 - percentage)}
                                            stroke-dashoffset="25"
                                            class={if is_complete { "text-green-500" } else { "text-blue-500" }}
                                        />
                                    </svg>
                                    <div class="absolute inset-0 flex items-center justify-center">
                                        <span class="text-xs font-semibold">
                                            {provided} "/" {required}
                                        </span>
                                    </div>
                                </div>
                                <div class="text-sm text-gray-600">
                                    {if is_complete {
                                        "Ready to submit (Cmd+Enter)"
                                    } else {
                                        "Add required tokens"
                                    }}
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! { <></> }.into_any()
                    }
                }}
            </div>
            
            // Visual tags for detected command and entity type
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
                                {format_token_type(&token.token_type)} ": " {token.instance.name.clone()}
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
            
            // Token type and entity instance selector popup
            <div
                class=move || match selection_mode.get() {
                    SelectionMode::None => "hidden",
                    _ => "entity-selector mb-2 p-3 bg-gray-100 rounded-lg"
                }
            >
                {move || match selection_mode.get() {
                    SelectionMode::TokenType => {
                        view! {
                            <>
                                <div class="text-sm font-semibold mb-2">"Select token type:"</div>
                                
                                // Search input for token types
                                <input
                                    type="text"
                                    class="w-full px-3 py-2 mb-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    placeholder="Type to filter token types..."
                                    prop:value=move || token_type_search.get()
                                    node_ref=token_search_ref
                                    on:input=move |e| {
                                        let value = event_target_value(&e);
                                        set_token_type_search.set(value.clone());
                                        
                                        // Filter token types
                                        let all_types = available_token_types.get();
                                        let filtered: Vec<String> = all_types.into_iter()
                                            .filter(|t| t.to_lowercase().contains(&value.to_lowercase()))
                                            .collect();
                                        set_filtered_token_types.set(filtered);
                                        set_selected_index.set(0);
                                    }
                                    on:keydown=move |e| {
                                        let key = e.key();
                                        let types = filtered_token_types.get();
                                        
                                        match key.as_str() {
                                            "ArrowDown" | "Tab" => {
                                                e.prevent_default();
                                                if !types.is_empty() {
                                                    set_selected_index.update(|i| *i = (*i + 1) % types.len());
                                                }
                                            },
                                            "ArrowUp" => {
                                                e.prevent_default();
                                                if !types.is_empty() {
                                                    set_selected_index.update(|i| {
                                                        if *i == 0 {
                                                            *i = types.len() - 1;
                                                        } else {
                                                            *i -= 1;
                                                        }
                                                    });
                                                }
                                            },
                                            "Enter" => {
                                                e.prevent_default();
                                                if let Some(token_type) = types.get(selected_index.get()) {
                                                    select_token_type(token_type.clone());
                                                }
                                            },
                                            "Escape" => {
                                                e.prevent_default();
                                                set_selection_mode.set(SelectionMode::None);
                                            },
                                            _ => {}
                                        }
                                        
                                        // Handle Ctrl+N and Ctrl+P
                                        if e.ctrl_key() {
                                            match key.as_str() {
                                                "n" => {
                                                    e.prevent_default();
                                                    if !types.is_empty() {
                                                        set_selected_index.update(|i| *i = (*i + 1) % types.len());
                                                    }
                                                },
                                                "p" => {
                                                    e.prevent_default();
                                                    if !types.is_empty() {
                                                        set_selected_index.update(|i| {
                                                            if *i == 0 {
                                                                *i = types.len() - 1;
                                                            } else {
                                                                *i -= 1;
                                                            }
                                                        });
                                                    }
                                                },
                                                _ => {}
                                            }
                                        }
                                    }
                                />
                                
                                // Token type list
                                {move || {
                                    let types = filtered_token_types.get();
                                    let current_index = selected_index.get();
                                    
                                    if types.is_empty() {
                                        view! {
                                            <div class="text-sm text-gray-500 text-center py-4">
                                                "No matching token types found."
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <>
                                                <div class="flex flex-col gap-1 max-h-60 overflow-y-auto">
                                                    {types.into_iter().enumerate().map(|(idx, token_type)| {
                                                        let tt = token_type.clone();
                                                        let is_selected = idx == current_index;
                                                        
                                                        view! {
                                                            <button
                                                                class=move || {
                                                                    if is_selected {
                                                                        "px-3 py-2 bg-blue-100 border border-blue-300 rounded-lg text-left"
                                                                    } else {
                                                                        "px-3 py-2 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 text-left"
                                                                    }
                                                                }
                                                                on:click=move |_| select_token_type(tt.clone())
                                                                on:mouseenter=move |_| set_selected_index.set(idx)
                                                            >
                                                                <div class="font-medium">{format_token_type(&token_type)}</div>
                                                            </button>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                                <div class="text-xs text-gray-500 mt-2">
                                                    "↑↓ or Ctrl+P/N to navigate • Enter to select • Esc to close"
                                                </div>
                                            </>
                                        }.into_any()
                                    }
                                }}
                            </>
                        }.into_any()
                    },
                    SelectionMode::EntityInstance => {
                        view! {
                            <>
                                <div class="text-sm font-semibold mb-2">
                                    {move || {
                                        if let Some(token_type) = selected_token_type.get() {
                                            format!("Select {}:", format_token_type(&token_type))
                                        } else {
                                            "Select entity:".to_string()
                                        }
                                    }}
                                </div>
                                
                                // Search input for entities
                                <input
                                    type="text"
                                    class="w-full px-3 py-2 mb-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    placeholder={move || {
                                        if let Some(token_type) = selected_token_type.get() {
                                            format!("Search {}...", format_token_type(&token_type).to_lowercase())
                                        } else {
                                            "Search...".to_string()
                                        }
                                    }}
                                    prop:value=move || entity_search_query.get()
                                    node_ref=entity_search_ref
                                    on:input=move |e| {
                                        let value = event_target_value(&e);
                                        set_entity_search_query.set(value.clone());
                                        
                                        // Filter entities locally and remove duplicates
                                        let all_entities = entity_instances.get();
                                        let mut seen = std::collections::HashSet::new();
                                        let filtered: Vec<EntityInstance> = all_entities.into_iter()
                                            .filter(|entity| {
                                                // First check if we've seen this ID
                                                if !seen.insert(entity.id.clone()) {
                                                    return false;
                                                }
                                                // Then apply search filter
                                                value.is_empty() || 
                                                entity.name.to_lowercase().contains(&value.to_lowercase()) ||
                                                entity.description.as_ref()
                                                    .map(|d| d.to_lowercase().contains(&value.to_lowercase()))
                                                    .unwrap_or(false)
                                            })
                                            .collect();
                                        set_filtered_entities.set(filtered);
                                        set_selected_index.set(0);
                                    }
                                    on:keydown=move |e| {
                                        let key = e.key();
                                        let entities = filtered_entities.get();
                                        let is_cmd_or_alt = if cfg!(target_os = "macos") {
                                            e.meta_key()
                                        } else {
                                            e.alt_key()
                                        };
                                        
                                        match key.as_str() {
                                            "ArrowDown" | "Tab" => {
                                                e.prevent_default();
                                                if !entities.is_empty() {
                                                    set_selected_index.update(|i| *i = (*i + 1) % entities.len());
                                                }
                                            },
                                            "ArrowUp" => {
                                                e.prevent_default();
                                                if !entities.is_empty() {
                                                    set_selected_index.update(|i| {
                                                        if *i == 0 {
                                                            *i = entities.len() - 1;
                                                        } else {
                                                            *i -= 1;
                                                        }
                                                    });
                                                }
                                            },
                                            "Enter" => {
                                                e.prevent_default();
                                                if is_cmd_or_alt {
                                                    // Cmd+Enter/Alt+Enter closes the selector
                                                    close_entity_selector!();
                                                } else if let Some(entity) = entities.get(selected_index.get()) {
                                                    select_entity(entity.clone());
                                                }
                                            },
                                            "Escape" => {
                                                e.prevent_default();
                                                close_entity_selector!();
                                            },
                                            _ => {}
                                        }
                                        
                                        // Handle Ctrl+N and Ctrl+P
                                        if e.ctrl_key() {
                                            match key.as_str() {
                                                "n" => {
                                                    e.prevent_default();
                                                    if !entities.is_empty() {
                                                        set_selected_index.update(|i| *i = (*i + 1) % entities.len());
                                                    }
                                                },
                                                "p" => {
                                                    e.prevent_default();
                                                    if !entities.is_empty() {
                                                        set_selected_index.update(|i| {
                                                            if *i == 0 {
                                                                *i = entities.len() - 1;
                                                            } else {
                                                                *i -= 1;
                                                            }
                                                        });
                                                    }
                                                },
                                                _ => {}
                                            }
                                        }
                                    }
                                />
                                
                                // Entity list
                                {move || {
                                    let instances = filtered_entities.get();
                                    if loading_entities.get() {
                                        view! {
                                            <div class="text-sm text-gray-500 text-center py-4">
                                                "Loading..."
                                            </div>
                                        }.into_any()
                                    } else if instances.is_empty() && entity_search_query.get().is_empty() {
                                        view! {
                                            <div class="text-sm text-gray-500 text-center py-4">
                                                {move || {
                                                    let token_type = selected_token_type.get().unwrap_or_default();
                                                    let selected_count = selected_tokens.get()
                                                        .iter()
                                                        .filter(|t| t.token_type == token_type)
                                                        .count();
                                                    
                                                    if selected_count > 0 {
                                                        format!("All available {} have been selected", format_token_type(&token_type).to_lowercase())
                                                    } else {
                                                        format!("No {} found.", format_token_type(&token_type).to_lowercase())
                                                    }
                                                }}
                                            </div>
                                        }.into_any()
                                    } else if instances.is_empty() {
                                        view! {
                                            <div class="text-sm text-gray-500 text-center py-4">
                                                {move || format!("No results for '{}'", entity_search_query.get())}
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <>
                                                <div class="flex flex-col gap-1 max-h-60 overflow-y-auto">
                                                    // Add "Create new" option at the top
                                                    <button
                                                        class=move || {
                                                            if selected_index.get() == 0 && show_create_new.get() {
                                                                "px-3 py-2 bg-green-100 border border-green-300 rounded-lg text-left"
                                                            } else {
                                                                "px-3 py-2 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 text-left"
                                                            }
                                                        }
                                                        on:click=move |_| {
                                                            let query = entity_search_query.get();
                                                            if !query.is_empty() {
                                                                // Create new entity with the search query as name
                                                                let new_entity = EntityInstance {
                                                                    id: format!("new_{}", uuid::Uuid::new_v4()),
                                                                    name: query.clone(),
                                                                    entity_type: selected_token_type.get().unwrap_or_default(),
                                                                    description: Some(format!("New {}", format_token_type(&selected_token_type.get().unwrap_or_default()))),
                                                                    metadata: None,
                                                                };
                                                                select_entity(new_entity);
                                                            }
                                                        }
                                                        on:mouseenter=move |_| {
                                                            set_selected_index.set(0);
                                                            set_show_create_new.set(true);
                                                        }
                                                    >
                                                        <div class="font-medium text-green-600">
                                                            "+ Create new: " {move || entity_search_query.get()}
                                                        </div>
                                                    </button>
                                                    
                                                    {instances.into_iter().enumerate().map(|(idx, instance)| {
                                                        let inst = instance.clone();
                                                        let is_selected = idx + 1 == selected_index.get(); // Adjust for "Create new" at index 0
                                                        
                                                        view! {
                                                            <button
                                                                class=move || {
                                                                    if is_selected {
                                                                        "px-3 py-2 bg-blue-100 border border-blue-300 rounded-lg text-left"
                                                                    } else {
                                                                        "px-3 py-2 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 text-left"
                                                                    }
                                                                }
                                                                on:click=move |_| select_entity(inst.clone())
                                                                on:mouseenter=move |_| set_selected_index.set(idx)
                                                            >
                                                                <div class="font-medium">{instance.name.clone()}</div>
                                                                {instance.description.map(|desc| view! {
                                                                    <div class="text-xs text-gray-500">{desc}</div>
                                                                })}
                                                            </button>
                                                        }
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                                <div class="flex items-center justify-between mt-2">
                                                    <div class="text-xs text-gray-500">
                                                        "↑↓ to navigate • Enter to add • Esc to cancel"
                                                    </div>
                                                    <button
                                                        class="px-3 py-1 bg-green-600 text-white rounded-lg hover:bg-green-700 text-sm font-medium"
                                                        on:click=move |_| close_entity_selector!()
                                                    >
                                                        {move || {
                                                            let count = selected_tokens.get()
                                                                .iter()
                                                                .filter(|t| {
                                                                    if let Some(token_type) = selected_token_type.get() {
                                                                        t.token_type == token_type
                                                                    } else {
                                                                        false
                                                                    }
                                                                })
                                                                .count();
                                                            if count > 0 {
                                                                format!("Done ({} selected)", count)
                                                            } else {
                                                                "Cancel".to_string()
                                                            }
                                                        }}
                                                    </button>
                                                </div>
                                            </>
                                        }.into_any()
                                    }
                                }}
                            </>
                        }.into_any()
                    },
                    SelectionMode::DirectInput => {
                        view! {
                            <>
                                <div class="text-sm font-semibold mb-2">
                                    {move || {
                                        if let Some(token_type) = selected_token_type.get() {
                                            format!("Enter {}: ", format_token_type(&token_type))
                                        } else {
                                            "Enter value:".to_string()
                                        }
                                    }}
                                </div>
                                
                                {move || {
                                    let component = current_component_type.get();
                                    match component {
                                        ComponentType::AmountInput => view! {
                                            <div class="flex gap-2">
                                                <input
                                                    type="number"
                                                    step="0.01"
                                                    class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                                    placeholder="Enter amount (e.g., 1000.00)"
                                                    on:keydown=move |e| {
                                                        if e.key() == "Enter" {
                                                            e.prevent_default();
                                                            let input_el = e.target().unwrap();
                                                            let input: HtmlInputElement = input_el.dyn_into().unwrap();
                                                            let value = input.value();
                                                            
                                                            if !value.is_empty() {
                                                                // Create a pseudo entity for the amount
                                                                let amount_entity = EntityInstance {
                                                                    id: format!("amount_{}", value),
                                                                    name: format!("₱{}", value),
                                                                    entity_type: "amount".to_string(),
                                                                    description: None,
                                                                    metadata: Some(serde_json::json!({ "value": value })),
                                                                };
                                                                select_entity(amount_entity);
                                                                close_entity_selector!();
                                                            }
                                                        } else if e.key() == "Escape" {
                                                            close_entity_selector!();
                                                        }
                                                    }
                                                    autofocus=true
                                                />
                                                <select class="px-3 py-2 border border-gray-300 rounded-lg">
                                                    <option value="PHP" selected="selected">"PHP"</option>
                                                    <option value="USD">"USD"</option>
                                                    <option value="EUR">"EUR"</option>
                                                </select>
                                            </div>
                                        }.into_any(),
                                        
                                        ComponentType::DatePicker => view! {
                                            <input
                                                type="date"
                                                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                                prop:value={
                                                    // Default to today
                                                    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                                                    today
                                                }
                                                on:change=move |e| {
                                                    let value = event_target_value(&e);
                                                    if !value.is_empty() {
                                                        let date_entity = EntityInstance {
                                                            id: format!("date_{}", value),
                                                            name: value.clone(),
                                                            entity_type: "date".to_string(),
                                                            description: None,
                                                            metadata: Some(serde_json::json!({ "date": value })),
                                                        };
                                                        select_entity(date_entity);
                                                        close_entity_selector!();
                                                    }
                                                }
                                                on:keydown=move |e| {
                                                    if e.key() == "Escape" {
                                                        close_entity_selector!();
                                                    }
                                                }
                                                autofocus=true
                                            />
                                        }.into_any(),
                                        
                                        ComponentType::TextInput => view! {
                                            <input
                                                type="text"
                                                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                                placeholder="Enter text..."
                                                on:keydown=move |e| {
                                                    if e.key() == "Enter" {
                                                        e.prevent_default();
                                                        let input_el = e.target().unwrap();
                                                        let input: HtmlInputElement = input_el.dyn_into().unwrap();
                                                        let value = input.value();
                                                        if !value.is_empty() {
                                                            let text_entity = EntityInstance {
                                                                id: format!("text_{}", uuid::Uuid::new_v4()),
                                                                name: value.clone(),
                                                                entity_type: selected_token_type.get().unwrap_or("text".to_string()),
                                                                description: None,
                                                                metadata: Some(serde_json::json!({ "text": value })),
                                                            };
                                                            select_entity(text_entity);
                                                            close_entity_selector!();
                                                        }
                                                    } else if e.key() == "Escape" {
                                                        close_entity_selector!();
                                                    }
                                                }
                                                autofocus=true
                                            />
                                        }.into_any(),
                                        
                                        _ => view! {
                                            <div class="text-sm text-gray-500">
                                                "Component type not yet implemented"
                                            </div>
                                        }.into_any()
                                    }
                                }}
                                
                                <div class="text-xs text-gray-500 mt-2">
                                    "Enter to confirm • Esc to cancel"
                                </div>
                            </>
                        }.into_any()
                    },
                    SelectionMode::None => view! { <></> }.into_any()
                }}
            </div>
            
            // Input field
            <input
                type="text"
                class="w-full p-3 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-lg"
                placeholder="Type 'create invoice @' to select a client..."
                prop:value=move || input_value.get()
                node_ref=main_input_ref
                on:input=move |e| {
                    let value = event_target_value(&e);
                    set_input_value.set(value);
                }
                on:keydown=handle_keydown
            />
            
            // Help text
            <div class="help-text text-xs text-gray-500 mt-2">
                "Commands: create, read, update, delete, list, search | Type @ to search and select entities | Press Enter to submit"
            </div>
            
            // Debug info
            <div class="mt-4 p-3 bg-gray-50 rounded text-xs text-gray-600">
                <div>"Mode: " {move || format!("{:?}", mode.get())}</div>
                <div>"Parse Result: " {move || format!("{:?}", parse_result.get())}</div>
                <div>"Selected Tokens: " {move || selected_tokens.get().len()}</div>
                <div>"Commands loaded: " {move || commands.get().len()}</div>
                <div>"Entity types loaded: " {move || entity_types.get().len()}</div>
            </div>
        </div>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchResponse {
    results: Vec<EntityInstance>,
}

// Helper function to get available token types based on command context
fn get_available_token_types(parse_result: &ParseResult) -> Vec<String> {
    // Based on the command and entity context, determine what token types are available
    match (parse_result.command.as_deref(), parse_result.entity.as_deref()) {
        (Some("create"), Some("invoice")) => vec![
            "client".to_string(),
            "product".to_string(),
            "date".to_string(),
            "amount".to_string(),
        ],
        (Some("create"), Some("payment")) => vec![
            "client".to_string(),
            "invoice".to_string(),
            "amount".to_string(),
            "date".to_string(),
        ],
        (Some("update"), Some("invoice")) => vec![
            "invoice".to_string(),
            "status".to_string(),
            "amount".to_string(),
        ],
        (Some("delete"), Some("invoice")) => vec!["invoice".to_string()],
        (Some("create"), Some("expense")) => vec![
            "supplier".to_string(),
            "category".to_string(),
            "amount".to_string(),
            "date".to_string(),
        ],
        (Some("list"), _) => vec![
            "client".to_string(),
            "invoice".to_string(),
            "product".to_string(),
            "payment".to_string(),
        ],
        _ => vec![
            "client".to_string(),
            "product".to_string(),
            "invoice".to_string(),
            "payment".to_string(),
        ],
    }
}

// Helper function to format token type for display
fn format_token_type(token_type: &str) -> String {
    match token_type {
        "client" => "Client",
        "product" => "Product",
        "invoice" => "Invoice",
        "payment" => "Payment",
        "supplier" => "Supplier",
        "date" => "Date",
        "amount" => "Amount",
        "status" => "Status",
        "category" => "Category",
        _ => token_type,
    }.to_string()
}