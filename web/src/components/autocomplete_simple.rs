use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};
use crate::config::api_url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub entity_type: String,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
struct SearchResponse {
    results: Vec<Entity>,
}

#[derive(Clone, Debug, Serialize)]
struct EntitySuggestionsRequest {
    command: String,
    cursor_position: usize,
    #[serde(default)]
    filled_tokens: Vec<FilledToken>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FilledToken {
    pub token_type: String,
    pub entity_id: String,
    pub entity_type: String,
}

#[derive(Clone, Debug, Deserialize)]
struct EntityTypeSuggestion {
    entity_type: String,
    label: String,
    description: String,
    cardinality: String,
    hint: Option<String>,
}

#[component]
pub fn AutocompletePopup(
    show: Signal<bool>,
    query: Signal<String>,
    entity_type: Signal<Option<String>>,
    position: Signal<(f64, f64)>,
    on_select: WriteSignal<Option<Entity>>,
    on_cancel: WriteSignal<bool>,
    #[prop(default = String::new().into())] command_context: Signal<String>,
    #[prop(default = 0.into())] cursor_pos: Signal<usize>,
    #[prop(default = vec![].into())] filled_tokens: Signal<Vec<FilledToken>>,
) -> impl IntoView {
    let (results, set_results) = signal::<Vec<Entity>>(vec![]);
    let (selected_index, set_selected_index) = signal(0);
    let (is_loading, set_is_loading) = signal(false);
    let (show_entity_types, set_show_entity_types) = signal(false);
    let (entity_suggestions, set_entity_suggestions) = signal::<Vec<EntityTypeSuggestion>>(vec![]);
    let (filtered_suggestions, set_filtered_suggestions) = signal::<Vec<EntityTypeSuggestion>>(vec![]);
    
    // Determine what to show based on entity_type
    Effect::new(move |_| {
        let etype = entity_type.get();
        let q = query.get();
        let cmd = command_context.get();
        let cpos = cursor_pos.get();
        
        if etype == Some("entity_type_hint".to_string()) {
            // Fetch context-aware entity type suggestions from backend
            set_show_entity_types.set(true);
            set_is_loading.set(true);
            let filled = filled_tokens.get();
            let q_clone = q.clone();
            
            spawn_local(async move {
                match fetch_entity_suggestions(cmd, cpos, filled).await {
                    Ok(suggestions) => {
                        // Filter suggestions based on query
                        let filtered: Vec<EntityTypeSuggestion> = if q_clone.is_empty() {
                            suggestions.clone()
                        } else {
                            suggestions.iter()
                                .filter(|s| {
                                    s.label.to_lowercase().contains(&q_clone.to_lowercase()) ||
                                    s.entity_type.to_lowercase().contains(&q_clone.to_lowercase())
                                })
                                .cloned()
                                .collect()
                        };
                        
                        set_entity_suggestions.set(suggestions);
                        set_filtered_suggestions.set(filtered);
                        set_selected_index.set(0);
                    }
                    Err(_) => {
                        set_entity_suggestions.set(vec![]);
                        set_filtered_suggestions.set(vec![]);
                    }
                }
                set_is_loading.set(false);
            });
        } else if etype.is_some() && show.get() {
            // Search for actual entities
            set_show_entity_types.set(false);
            set_is_loading.set(true);
            let entity_type_value = etype.clone();
            
            spawn_local(async move {
                match search_entities(q, entity_type_value).await {
                    Ok(entities) => {
                        set_results.set(entities);
                        set_selected_index.set(0);
                    }
                    Err(_) => {
                        set_results.set(vec![]);
                    }
                }
                set_is_loading.set(false);
            });
        } else {
            set_show_entity_types.set(false);
            set_results.set(vec![]);
            set_entity_suggestions.set(vec![]);
        }
    });
    
    // Handle keyboard navigation directly in the view
    let handle_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if !show.get_untracked() {
            return;
        }
        
        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                let max_index = if show_entity_types.get_untracked() {
                    filtered_suggestions.get_untracked().len().saturating_sub(1)
                } else {
                    results.get_untracked().len().saturating_sub(1)
                };
                set_selected_index.update(|idx| *idx = (*idx + 1).min(max_index));
            },
            "ArrowUp" => {
                ev.prevent_default();
                set_selected_index.update(|idx| *idx = idx.saturating_sub(1));
            },
            "Enter" | "Tab" => {
                ev.prevent_default();
                let show_types = show_entity_types.get_untracked();
                let selected_idx = selected_index.get_untracked();
                
                if show_types {
                    let suggestions = filtered_suggestions.get_untracked();
                    if let Some(suggestion) = suggestions.get(selected_idx) {
                        let etype = suggestion.entity_type.clone();
                        let cardinality = suggestion.cardinality.clone();
                        on_select.set(Some(Entity {
                            id: format!("_type_{}", etype),
                            name: etype,
                            entity_type: "_entity_type".to_string(),
                            description: None,
                            metadata: Some(serde_json::json!({
                                "cardinality": cardinality
                            })),
                        }));
                    }
                } else {
                    let results_list = results.get_untracked();
                    if let Some(entity) = results_list.get(selected_idx) {
                        on_select.set(Some(entity.clone()));
                    }
                }
            },
            "Escape" => {
                ev.prevent_default();
                on_cancel.set(true);
            },
            _ => {}
        }
    };
    
    view! {
        <Show when=move || show.get() && (results.get().len() > 0 || is_loading.get() || show_entity_types.get() || filtered_suggestions.get().len() > 0)>
            <div 
                class="fixed z-50 bg-white rounded-lg shadow-xl border border-gray-200 max-h-64 overflow-y-auto min-w-[250px]"
                style={move || {
                    let (x, y) = position.get();
                    // Position above the input
                    format!("left: {}px; bottom: {}px;", x, 500.0 - y)
                }}
                on:keydown=handle_keydown
                tabindex="0"
            >
                {move || if show_entity_types.get() {
                    // Show entity type suggestions
                    view! {
                        <div class="py-1">
                            <div class="px-3 py-2 text-xs font-semibold text-gray-500 uppercase tracking-wider">
                                "Available Entity Types"
                            </div>
                            <For
                                each=move || filtered_suggestions.get().into_iter().enumerate()
                                key=|(_, suggestion)| suggestion.entity_type.clone()
                                children=move |(index, suggestion)| {
                                    let is_selected = move || selected_index.get() == index;
                                    let etype_for_click = suggestion.entity_type.clone();
                                    let label = suggestion.label.clone();
                                    let desc = suggestion.description.clone();
                                    let cardinality = suggestion.cardinality.clone();
                                    let hint = suggestion.hint.clone();
                                    
                                    view! {
                                        <button
                                            class={move || if is_selected() {
                                                "w-full px-4 py-2 text-left hover:bg-blue-50 bg-blue-100 cursor-pointer"
                                            } else {
                                                "w-full px-4 py-2 text-left hover:bg-gray-50 cursor-pointer"
                                            }}
                                            on:click=move |_| {
                                                // Create a fake entity to signal entity type selection
                                                let click_type = etype_for_click.clone();
                                                on_select.set(Some(Entity {
                                                    id: format!("_type_{}", click_type.clone()),
                                                    name: click_type,
                                                    entity_type: "_entity_type".to_string(),
                                                    description: None,
                                                    metadata: Some(serde_json::json!({
                                                        "cardinality": cardinality.clone()
                                                    })),
                                                }))
                                            }
                                            on:mouseenter=move |_| set_selected_index.set(index)
                                        >
                                            <div class="flex flex-col space-y-1">
                                                <div class="flex items-center justify-between">
                                                    <div class="flex items-center space-x-2">
                                                        <span class="font-mono text-blue-600">{label}</span>
                                                        <span class="text-xs text-gray-500">{desc}</span>
                                                    </div>
                                                    {if cardinality == "multiple" {
                                                        view! {
                                                            <span class="text-xs bg-gray-200 text-gray-600 px-1.5 py-0.5 rounded">
                                                                "Multiple"
                                                            </span>
                                                        }.into_any()
                                                    } else {
                                                        view! { <span></span> }.into_any()
                                                    }}
                                                </div>
                                                {hint.as_ref().map(|h| {
                                                    view! {
                                                        <div class="text-xs text-gray-400 italic pl-2">{h.clone()}</div>
                                                    }.into_any()
                                                }).unwrap_or_else(|| view! { <span></span> }.into_any())}
                                            </div>
                                        </button>
                                    }
                                }
                            />
                        </div>
                    }.into_any()
                } else if is_loading.get() {
                    view! {
                        <div class="px-4 py-3 text-gray-500 text-sm">
                            <div class="flex items-center space-x-2">
                                <div class="w-4 h-4 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
                                <span>"Searching..."</span>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="py-1">
                            <For
                                each=move || results.get().into_iter().enumerate()
                                key=|(_, entity)| entity.id.clone()
                                children=move |(index, entity)| {
                                    let is_selected = move || selected_index.get() == index;
                                    let entity_for_click = entity.clone();
                                    
                                    view! {
                                        <button
                                            class={move || if is_selected() {
                                                "w-full px-4 py-2 text-left hover:bg-blue-50 bg-blue-100 cursor-pointer flex items-center space-x-3"
                                            } else {
                                                "w-full px-4 py-2 text-left hover:bg-gray-50 cursor-pointer flex items-center space-x-3"
                                            }}
                                            on:click=move |_| on_select.set(Some(entity_for_click.clone()))
                                            on:mouseenter=move |_| set_selected_index.set(index)
                                        >
                                            <div class={
                                                let color = match entity.entity_type.as_str() {
                                                    "client" => "bg-blue-500",
                                                    "supplier" => "bg-green-500",
                                                    "product" => "bg-purple-500",
                                                    "employee" => "bg-orange-500",
                                                    _ => "bg-gray-500",
                                                };
                                                format!("w-8 h-8 {} text-white rounded-full flex items-center justify-center text-xs font-bold", color)
                                            }>
                                                {entity.name.chars().next().unwrap_or('?').to_uppercase().to_string()}
                                            </div>
                                            <div class="flex-1">
                                                <div class="text-sm font-medium text-gray-900">{entity.name.clone()}</div>
                                                {entity.description.as_ref().map(|desc| {
                                                    view! {
                                                        <div class="text-xs text-gray-500">{desc.clone()}</div>
                                                    }.into_any()
                                                }).unwrap_or_else(|| {
                                                    view! { <div></div> }.into_any()
                                                })}
                                            </div>
                                            <div class="text-xs text-gray-400 capitalize">{entity.entity_type.clone()}</div>
                                        </button>
                                    }
                                }
                            />
                        </div>
                    }.into_any()
                }}
            </div>
        </Show>
    }
}

async fn search_entities(query: String, entity_type: Option<String>) -> Result<Vec<Entity>, String> {
    let client = reqwest::Client::new();
    
    let mut url = api_url(&format!("api/search?q={}", query));
    if let Some(etype) = entity_type {
        url.push_str(&format!("&type={}", etype));
    }
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    
    if response.status().is_success() {
        let data: SearchResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(data.results)
    } else {
        Err(format!("Server error: {}", response.status()))
    }
}

async fn fetch_entity_suggestions(command: String, cursor_position: usize, filled_tokens: Vec<FilledToken>) -> Result<Vec<EntityTypeSuggestion>, String> {
    let client = reqwest::Client::new();
    
    let request = EntitySuggestionsRequest {
        command,
        cursor_position,
        filled_tokens,
    };
    
    let response = client
        .post(&api_url("api/entity-suggestions"))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    
    if response.status().is_success() {
        let suggestions: Vec<EntityTypeSuggestion> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(suggestions)
    } else {
        Err(format!("Server error: {}", response.status()))
    }
}

// Helper function to detect @ mentions in text and get cursor position
pub fn detect_mention(text: &str, cursor_pos: usize) -> Option<(String, String)> {
    // Find the @ symbol before cursor
    let before_cursor = &text[..cursor_pos];
    
    if let Some(at_pos) = before_cursor.rfind('@') {
        let mention_text = &before_cursor[at_pos + 1..];
        
        // Check if we have a space after @ (which means entity type is complete)
        if let Some(space_pos) = mention_text.find(' ') {
            // Entity type is complete, extract it and the search query
            let entity_type_text = &mention_text[..space_pos];
            let search_query = &mention_text[space_pos + 1..];
            
            // Map the entity type text to actual entity type
            let entity_type = match entity_type_text {
                "client" | "clients" => "client",
                "company" | "companies" => "client", // companies are clients
                "supplier" | "suppliers" => "supplier",
                "product" | "products" => "product",
                "employee" | "employees" => "employee",
                "invoice" | "invoices" => "invoice",
                "payment" | "payments" => "payment",
                _ => return None, // Invalid entity type
            };
            
            return Some((search_query.to_string(), entity_type.to_string()));
        } else {
            // No space yet, user is still typing the entity type
            // Show suggestions for entity types
            return Some((mention_text.to_string(), "entity_type_hint".to_string()));
        }
    }
    
    None
}