use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::Deserialize;
use crate::components::autocomplete_simple::Entity;
use crate::config::api_url;

#[derive(Clone, Debug, Deserialize)]
struct SearchResponse {
    results: Vec<Entity>,
}

#[component]
pub fn EntityCombobox(
    entity_type: String,
    prompt: String,
    #[prop(default = String::new())] initial_query: String,
    on_select: WriteSignal<Option<Entity>>,
    on_cancel: WriteSignal<bool>,
) -> impl IntoView {
    let (search_query, set_search_query) = signal(initial_query.clone());
    let (results, set_results) = signal::<Vec<Entity>>(vec![]);
    let (selected_index, set_selected_index) = signal(0);
    let (is_loading, set_is_loading) = signal(false);
    let (show_dropdown, set_show_dropdown) = signal(true);
    
    // Load initial entities on mount
    {
        let entity_type = entity_type.clone();
        let initial_q = initial_query.clone();
        spawn_local(async move {
            set_is_loading.set(true);
            let result = if !initial_q.is_empty() {
                // If there's an initial query, search with it
                search_entities(initial_q, Some(entity_type)).await
            } else {
                // Otherwise load all entities
                load_entities(entity_type).await
            };
            
            match result {
                Ok(entities) => {
                    set_results.set(entities);
                }
                Err(_) => {
                    set_results.set(vec![]);
                }
            }
            set_is_loading.set(false);
        });
    }
    
    // Search effect - debounced
    let (search_trigger, set_search_trigger) = signal(0u32);
    
    Effect::new({
        let entity_type = entity_type.clone();
        move |_| {
            let query = search_query.get();
            if query.is_empty() {
                return; // Don't search on empty, we already loaded all
            }
            
            // Trigger search
            set_search_trigger.update(|v| *v += 1);
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
                    match search_entities(query, Some(entity_type)).await {
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
            }
        }
    });
    
    let handle_select = move |entity: Entity| {
        on_select.set(Some(entity));
        set_show_dropdown.set(false);
    };
    
    let handle_keydown = move |ev: leptos::ev::KeyboardEvent| {
        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                let current = selected_index.get();
                let max = results.get().len().saturating_sub(1);
                set_selected_index.set((current + 1).min(max));
            },
            "ArrowUp" => {
                ev.prevent_default();
                let current = selected_index.get();
                set_selected_index.set(current.saturating_sub(1));
            },
            "Enter" => {
                ev.prevent_default();
                if let Some(entity) = results.get().get(selected_index.get()) {
                    handle_select(entity.clone());
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
        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
            on:click=move |ev| {
                // Check if click is on backdrop
                if ev.target() == ev.current_target() {
                    on_cancel.set(true);
                }
            }
        >
            <div class="bg-white rounded-xl shadow-2xl max-w-lg w-full mx-4"
                on:click=move |ev| {
                    // Prevent clicks inside modal from closing it
                    ev.stop_propagation();
                }
            >
                <div class="px-6 py-4 border-b border-gray-200">
                    <h3 class="text-lg font-semibold text-gray-900">
                        {format!("Select {}", capitalize_entity_type(&entity_type))}
                    </h3>
                    <p class="text-sm text-gray-600 mt-1">{prompt}</p>
                </div>
                
                <div class="p-4">
                    <input
                        type="text"
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                        placeholder={format!("Search {}...", entity_type)}
                        prop:value=move || search_query.get()
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        on:keydown=handle_keydown
                        autofocus=true
                    />
                </div>
                
                <div class="max-h-64 overflow-y-auto border-t border-gray-200">
                    {move || if is_loading.get() {
                        view! {
                            <div class="px-6 py-8 text-center">
                                <div class="inline-flex items-center space-x-2 text-gray-500">
                                    <div class="w-5 h-5 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
                                    <span>"Loading..."</span>
                                </div>
                            </div>
                        }.into_any()
                    } else if results.get().is_empty() {
                        view! {
                            <div class="px-6 py-8 text-center text-gray-500">
                                {if search_query.get().is_empty() {
                                    format!("No {} found", entity_type)
                                } else {
                                    format!("No {} matching '{}'", entity_type, search_query.get())
                                }}
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="py-2">
                                <For
                                    each=move || results.get().into_iter().enumerate()
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
                                                on:click=move |_| handle_select(entity_for_click.clone())
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
                                                    format!("w-10 h-10 {} text-white rounded-lg flex items-center justify-center text-sm font-bold", color)
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
                                                {move || if is_selected() {
                                                    view! {
                                                        <svg class="w-5 h-5 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                                                            <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
                                                        </svg>
                                                    }.into_any()
                                                } else {
                                                    view! { <div></div> }.into_any()
                                                }}
                                            </button>
                                        }
                                    }
                                />
                            </div>
                        }.into_any()
                    }}
                </div>
                
                <div class="px-6 py-4 border-t border-gray-200 flex justify-end space-x-3">
                    <button
                        class="px-4 py-2 text-gray-700 hover:text-gray-900 transition-colors"
                        on:click=move |_| on_cancel.set(true)
                    >
                        "Cancel"
                    </button>
                </div>
            </div>
        </div>
    }
}

async fn load_entities(entity_type: String) -> Result<Vec<Entity>, String> {
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

fn capitalize_entity_type(entity_type: &str) -> String {
    match entity_type {
        "client" => "Client",
        "supplier" => "Supplier", 
        "product" => "Product",
        "employee" => "Employee",
        "invoice" => "Invoice",
        "payment" => "Payment",
        _ => entity_type,
    }.to_string()
}