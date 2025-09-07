use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use crate::components::autocomplete_simple::Entity;
use crate::config::api_url;
use std::collections::HashSet;

#[derive(Clone, Debug, Deserialize)]
struct SearchResponse {
    results: Vec<Entity>,
}

#[component]
pub fn MultiSelectEntityCombobox(
    entity_type: String,
    prompt: String,
    #[prop(default = None)] max_selections: Option<usize>,
    #[prop(default = vec![])] initial_selections: Vec<Entity>,
    on_confirm: WriteSignal<Vec<Entity>>,
    on_cancel: WriteSignal<bool>,
) -> impl IntoView {
    // State management
    let (selected_entities, set_selected_entities) = signal(initial_selections.clone());
    let (search_query, set_search_query) = signal(String::new());
    let (search_results, set_search_results) = signal::<Vec<Entity>>(vec![]);
    let (filtered_results, set_filtered_results) = signal::<Vec<Entity>>(vec![]);
    let (selected_index, set_selected_index) = signal(0);
    let (is_loading, set_is_loading) = signal(false);
    
    // Track selected entity IDs for quick lookup
    let selected_ids = move || {
        selected_entities.get()
            .iter()
            .map(|e| e.id.clone())
            .collect::<HashSet<String>>()
    };
    
    // Check if we can add more selections
    let can_add_more = move || {
        match max_selections {
            Some(max) => selected_entities.get().len() < max,
            None => true,
        }
    };
    
    // Load initial entities on mount
    {
        let entity_type = entity_type.clone();
        spawn_local(async move {
            set_is_loading.set(true);
            match load_all_entities(entity_type).await {
                Ok(entities) => {
                    set_search_results.set(entities.clone());
                    set_filtered_results.set(entities);
                }
                Err(_) => {
                    set_search_results.set(vec![]);
                    set_filtered_results.set(vec![]);
                }
            }
            set_is_loading.set(false);
        });
    }
    
    // Filter results based on search query and already selected items
    Effect::new(move |_| {
        let query = search_query.get();
        let selected = selected_ids();
        let all_results = search_results.get();
        
        let filtered: Vec<Entity> = all_results
            .into_iter()
            .filter(|entity| {
                // Don't show already selected items
                !selected.contains(&entity.id) &&
                // Filter by search query
                (query.is_empty() || 
                 entity.name.to_lowercase().contains(&query.to_lowercase()) ||
                 entity.description.as_ref()
                    .map(|d| d.to_lowercase().contains(&query.to_lowercase()))
                    .unwrap_or(false))
            })
            .collect();
        
        set_filtered_results.set(filtered);
        set_selected_index.set(0);
    });
    
    // Search effect with debouncing
    let (search_trigger, set_search_trigger) = signal(0u32);
    
    Effect::new({
        let entity_type = entity_type.clone();
        move |_| {
            let query = search_query.get();
            if !query.is_empty() {
                set_search_trigger.update(|v| *v += 1);
            }
        }
    });
    
    Effect::new({
        let entity_type = entity_type.clone();
        move |_| {
            let _ = search_trigger.get();
            let query = search_query.get();
            
            if !query.is_empty() {
                let entity_type = entity_type.clone();
                set_is_loading.set(true);
                
                spawn_local(async move {
                    match search_entities(query, entity_type).await {
                        Ok(entities) => {
                            set_search_results.set(entities);
                        }
                        Err(_) => {
                            set_search_results.set(vec![]);
                        }
                    }
                    set_is_loading.set(false);
                });
            }
        }
    });
    
    // Handle adding an entity to selection
    let add_entity = move |entity: Entity| {
        if can_add_more() {
            set_selected_entities.update(|entities| {
                entities.push(entity);
            });
            set_search_query.set(String::new());
        }
    };
    
    // Handle removing an entity from selection
    let remove_entity = move |entity_id: String| {
        set_selected_entities.update(|entities| {
            entities.retain(|e| e.id != entity_id);
        });
    };
    
    // Handle keyboard navigation
    let handle_keydown = move |ev: leptos::ev::KeyboardEvent| {
        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                let max = filtered_results.get().len().saturating_sub(1);
                set_selected_index.update(|idx| *idx = (*idx + 1).min(max));
            },
            "ArrowUp" => {
                ev.prevent_default();
                set_selected_index.update(|idx| *idx = idx.saturating_sub(1));
            },
            "Enter" => {
                ev.prevent_default();
                if ev.meta_key() || ev.ctrl_key() {
                    // Cmd/Ctrl+Enter confirms selection
                    on_confirm.set(selected_entities.get());
                } else {
                    // Regular Enter adds to selection
                    if let Some(entity) = filtered_results.get().get(selected_index.get()) {
                        add_entity(entity.clone());
                    }
                }
            },
            "Backspace" => {
                if search_query.get().is_empty() && !selected_entities.get().is_empty() {
                    // Remove last selected entity
                    set_selected_entities.update(|entities| {
                        entities.pop();
                    });
                }
            },
            "Escape" => {
                ev.prevent_default();
                on_cancel.set(true);
            },
            _ => {}
        }
    };
    
    // Handle done button click
    let handle_done = move |_| {
        on_confirm.set(selected_entities.get());
    };
    
    view! {
        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
            on:click=move |ev| {
                if ev.target() == ev.current_target() {
                    on_cancel.set(true);
                }
            }
        >
            <div class="bg-white rounded-xl shadow-2xl max-w-2xl w-full mx-4 max-h-[80vh] flex flex-col">
                // Header
                <div class="px-6 py-4 border-b border-gray-200">
                    <h3 class="text-lg font-semibold text-gray-900">
                        {format!("Select {}", capitalize_entity_type(&entity_type))}
                    </h3>
                    <p class="text-sm text-gray-600 mt-1">{prompt}</p>
                </div>
                
                // Selected items display
                <Show when=move || !selected_entities.get().is_empty()>
                    <div class="px-6 py-3 border-b border-gray-200 bg-gray-50">
                        <div class="flex items-center justify-between mb-2">
                            <span class="text-sm font-medium text-gray-700">
                                {move || {
                                    let count = selected_entities.get().len();
                                    match max_selections {
                                        Some(max) => format!("Selected ({}/{})", count, max),
                                        None => format!("Selected ({})", count)
                                    }
                                }}
                            </span>
                            {move || if !can_add_more() {
                                view! {
                                    <span class="text-xs text-orange-600 font-medium">
                                        "Maximum reached"
                                    </span>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                        </div>
                        <div class="flex flex-wrap gap-2">
                            <For
                                each=move || selected_entities.get()
                                key=|entity| entity.id.clone()
                                children=move |entity| {
                                    let entity_id = entity.id.clone();
                                    let entity_name = entity.name.clone();
                                    let entity_type_color = get_entity_color(&entity.entity_type);
                                    
                                    view! {
                                        <div class={format!("inline-flex items-center gap-1 px-3 py-1.5 rounded-full text-sm {} text-white", entity_type_color)}>
                                            <span>{entity_name}</span>
                                            <button
                                                class="ml-1 hover:bg-white hover:bg-opacity-20 rounded-full p-0.5"
                                                on:click=move |ev| {
                                                    ev.prevent_default();
                                                    ev.stop_propagation();
                                                    remove_entity(entity_id.clone());
                                                }
                                            >
                                                <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                                                    <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"/>
                                                </svg>
                                            </button>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    </div>
                </Show>
                
                // Search input
                <div class="p-4 border-b border-gray-200">
                    <input
                        type="text"
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                        placeholder={format!("Search {}...", entity_type)}
                        prop:value=move || search_query.get()
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        on:keydown=handle_keydown
                        autofocus=true
                        disabled=move || !can_add_more()
                    />
                    <div class="mt-2 text-xs text-gray-500">
                        {move || if can_add_more() {
                            "Press Enter to add, Cmd+Enter to finish"
                        } else {
                            "Maximum selections reached. Remove items to add more."
                        }}
                    </div>
                </div>
                
                // Results list
                <div class="flex-1 overflow-y-auto">
                    {move || if is_loading.get() {
                        view! {
                            <div class="px-6 py-8 text-center">
                                <div class="inline-flex items-center space-x-2 text-gray-500">
                                    <div class="w-5 h-5 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
                                    <span>"Loading..."</span>
                                </div>
                            </div>
                        }.into_any()
                    } else if filtered_results.get().is_empty() && can_add_more() {
                        view! {
                            <div class="px-6 py-8 text-center text-gray-500">
                                {if search_query.get().is_empty() {
                                    format!("No more {} to select", entity_type)
                                } else {
                                    format!("No {} matching '{}'", entity_type, search_query.get())
                                }}
                            </div>
                        }.into_any()
                    } else if can_add_more() {
                        view! {
                            <div class="py-2">
                                <For
                                    each=move || filtered_results.get().into_iter().enumerate()
                                    key=|(_, entity)| entity.id.clone()
                                    children=move |(index, entity)| {
                                        let is_selected = move || selected_index.get() == index;
                                        let entity_for_click = entity.clone();
                                        
                                        view! {
                                            <button
                                                class={move || if is_selected() {
                                                    "w-full px-6 py-3 text-left hover:bg-blue-50 bg-blue-100 cursor-pointer flex items-center space-x-3 transition-colors"
                                                } else {
                                                    "w-full px-6 py-3 text-left hover:bg-gray-50 cursor-pointer flex items-center space-x-3 transition-colors"
                                                }}
                                                on:click=move |_| add_entity(entity_for_click.clone())
                                                on:mouseenter=move |_| set_selected_index.set(index)
                                            >
                                                <div class={format!("w-10 h-10 {} text-white rounded-lg flex items-center justify-center text-sm font-bold", 
                                                    get_entity_color(&entity.entity_type))}>
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
                                            </button>
                                        }
                                    }
                                />
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }}
                </div>
                
                // Footer with buttons
                <div class="px-6 py-4 border-t border-gray-200 flex justify-end space-x-3">
                    <button
                        class="px-4 py-2 text-gray-700 hover:text-gray-900 transition-colors"
                        on:click=move |_| on_cancel.set(true)
                    >
                        "Cancel"
                    </button>
                    <button
                        class={move || {
                            let has_selections = !selected_entities.get().is_empty();
                            if has_selections {
                                "px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors font-medium"
                            } else {
                                "px-6 py-2 bg-gray-300 text-gray-500 rounded-lg cursor-not-allowed font-medium"
                            }
                        }}
                        on:click=handle_done
                        disabled=move || selected_entities.get().is_empty()
                    >
                        {move || {
                            let count = selected_entities.get().len();
                            if count == 0 {
                                "Done".to_string()
                            } else {
                                format!("Done ({})", count)
                            }
                        }}
                    </button>
                </div>
            </div>
        </div>
    }
}

async fn load_all_entities(entity_type: String) -> Result<Vec<Entity>, String> {
    let client = reqwest::Client::new();
    let url = api_url(&format!("api/entities/{}", entity_type));
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    
    if response.status().is_success() {
        let entities: Vec<Entity> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(entities)
    } else {
        Err(format!("Server error: {}", response.status()))
    }
}

async fn search_entities(query: String, entity_type: String) -> Result<Vec<Entity>, String> {
    let client = reqwest::Client::new();
    let url = api_url(&format!("api/search?q={}&type={}", query, entity_type));
    
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

fn capitalize_entity_type(entity_type: &str) -> String {
    match entity_type {
        "client" => "Clients",
        "supplier" => "Suppliers", 
        "product" => "Products",
        "employee" => "Employees",
        "invoice" => "Invoices",
        "payment" => "Payments",
        _ => entity_type,
    }.to_string()
}

fn get_entity_color(entity_type: &str) -> &'static str {
    match entity_type {
        "client" => "bg-blue-500",
        "supplier" => "bg-green-500",
        "product" => "bg-purple-500",
        "employee" => "bg-orange-500",
        "invoice" => "bg-indigo-500",
        "payment" => "bg-teal-500",
        _ => "bg-gray-500",
    }
}