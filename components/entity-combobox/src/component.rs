use leptos::prelude::*;
use leptos::html::Input;
use leptos::ev::KeyboardEvent;
use leptos::task::spawn_local;
use gloo_timers::future::TimeoutFuture;
use crate::types::*;
use crate::api::{ApiCache, load_entities, search_entities};

/// EntityCombobox component - searchable dropdown with entity selection
#[component]
pub fn EntityCombobox(
    /// Entity type to search for
    entity_type: EntityType,
    
    /// Prompt text to show
    #[prop(optional)] prompt: Option<String>,
    
    /// Initial search query
    #[prop(optional)] initial_query: Option<String>,
    
    /// Callback when an entity is selected
    on_select: Callback<Entity, ()>,
    
    /// Callback when cancelled
    #[prop(optional)] on_cancel: Option<Callback<(), ()>>,
    
    /// Configuration
    #[prop(optional)] config: Option<EntityComboboxConfig>,
    
    /// API base URL (use "mock" for demo data)
    #[prop(default = "mock".to_string())] api_url: String,
    
    /// Optional CSS class
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let config = config.unwrap_or_default();
    
    // State
    let (search_query, set_search_query) = signal(initial_query.clone().unwrap_or_default());
    let (results, set_results) = signal::<Vec<Entity>>(vec![]);
    let (selected_index, set_selected_index) = signal(0);
    let (is_loading, set_is_loading) = signal(false);
    let (show_dropdown, set_show_dropdown) = signal(true);
    let (error_message, set_error_message) = signal::<Option<String>>(None);
    
    let input_ref = NodeRef::<Input>::new();
    
    // API cache
    let cache = if config.cache_results {
        Some(ApiCache::new(config.cache_expiry_secs))
    } else {
        None
    };
    
    // Load initial entities on mount
    {
        let entity_type = entity_type.clone();
        let api_url = api_url.clone();
        let cache = cache.clone();
        let initial_q = initial_query.clone();
        
        spawn_local(async move {
            set_is_loading.set(true);
            set_error_message.set(None);
            
            // Check cache first if enabled
            let cached = if let Some(ref cache) = cache {
                if let Some(q) = &initial_q {
                    if !q.is_empty() {
                        cache.get(entity_type.as_str(), Some(q))
                    } else {
                        cache.get(entity_type.as_str(), None)
                    }
                } else {
                    cache.get(entity_type.as_str(), None)
                }
            } else {
                None
            };
            
            let entities = if let Some(cached_entities) = cached {
                Ok(cached_entities)
            } else {
                let result = if let Some(q) = initial_q {
                    if !q.is_empty() {
                        search_entities(&api_url, q.clone(), Some(entity_type.as_str().to_string())).await
                    } else {
                        load_entities(&api_url, entity_type.as_str().to_string()).await
                    }
                } else {
                    load_entities(&api_url, entity_type.as_str().to_string()).await
                };
                
                // Cache the result if successful
                if let (Ok(ref entities), Some(ref cache)) = (&result, &cache) {
                    if let Some(q) = &initial_q {
                        if !q.is_empty() {
                            cache.set(entity_type.as_str(), Some(q), entities.clone());
                        } else {
                            cache.set(entity_type.as_str(), None, entities.clone());
                        }
                    } else {
                        cache.set(entity_type.as_str(), None, entities.clone());
                    }
                }
                
                result
            };
            
            match entities {
                Ok(entities) => {
                    set_results.set(entities);
                }
                Err(err) => {
                    set_error_message.set(Some(err));
                    set_results.set(vec![]);
                }
            }
            set_is_loading.set(false);
        });
    }
    
    // Debounced search
    let (search_trigger, set_search_trigger) = signal(0u32);
    
    Effect::new({
        let entity_type = entity_type.clone();
        let api_url = api_url.clone();
        let cache = cache.clone();
        let debounce_ms = config.debounce_ms;
        
        move |_| {
            let query = search_query.get();
            
            if !query.is_empty() {
                let entity_type = entity_type.clone();
                let api_url = api_url.clone();
                let cache = cache.clone();
                
                spawn_local(async move {
                    // Debounce
                    TimeoutFuture::new(debounce_ms).await;
                    set_search_trigger.update(|v| *v += 1);
                });
            }
        }
    });
    
    Effect::new({
        let entity_type = entity_type.clone();
        let api_url = api_url.clone();
        let cache = cache.clone();
        
        move |_| {
            let _ = search_trigger.get();
            let query = search_query.get();
            
            if !query.is_empty() {
                set_is_loading.set(true);
                set_error_message.set(None);
                
                let entity_type = entity_type.clone();
                let api_url = api_url.clone();
                let cache = cache.clone();
                
                spawn_local(async move {
                    // Check cache first
                    let cached = if let Some(ref cache) = cache {
                        cache.get(entity_type.as_str(), Some(&query))
                    } else {
                        None
                    };
                    
                    let result = if let Some(cached_entities) = cached {
                        Ok(cached_entities)
                    } else {
                        let result = search_entities(&api_url, query.clone(), Some(entity_type.as_str().to_string())).await;
                        
                        // Cache successful results
                        if let (Ok(ref entities), Some(ref cache)) = (&result, &cache) {
                            cache.set(entity_type.as_str(), Some(&query), entities.clone());
                        }
                        
                        result
                    };
                    
                    match result {
                        Ok(entities) => {
                            set_results.set(entities);
                            set_selected_index.set(0);
                        }
                        Err(err) => {
                            set_error_message.set(Some(err));
                            set_results.set(vec![]);
                        }
                    }
                    set_is_loading.set(false);
                });
            }
        }
    });
    
    // Auto-focus input on mount
    Effect::new(move |_| {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    });
    
    // Auto-scroll selected item into view
    Effect::new(move |_| {
        let idx = selected_index.get();
        if show_dropdown.get() {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.get_element_by_id(&format!("ec-item-{}", idx)) {
                        let scroll_options = web_sys::ScrollIntoViewOptions::new();
                        scroll_options.set_block(web_sys::ScrollLogicalPosition::Nearest);
                        scroll_options.set_behavior(web_sys::ScrollBehavior::Smooth);
                        element.scroll_into_view_with_scroll_into_view_options(&scroll_options);
                    }
                }
            }
        }
    });
    
    let handle_select = move |entity: Entity| {
        on_select.run(entity);
        set_show_dropdown.set(false);
    };
    
    let handle_cancel = move || {
        if let Some(on_cancel) = on_cancel {
            on_cancel.run(());
        }
        set_show_dropdown.set(false);
    };
    
    let handle_keydown = move |ev: KeyboardEvent| {
        if !config.enable_keyboard_nav {
            return;
        }
        
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
                handle_cancel();
            },
            _ => {}
        }
    };
    
    let custom_class = class.unwrap_or_default();
    
    view! {
        <div class={format!("ec-container {}", custom_class)}>
            <Show when=move || show_dropdown.get()>
                <div class="ec-backdrop"
                    on:click=move |_| handle_cancel()
                >
                    <div class="ec-modal"
                        on:click=move |ev| {
                            // Prevent clicks inside modal from closing it
                            ev.stop_propagation();
                        }
                    >
                        <div class="ec-header">
                            <h3 class="ec-title">
                                {format!("Select {}", entity_type.display_name())}
                            </h3>
                            {prompt.as_ref().map(|p| {
                                view! {
                                    <p class="ec-prompt">{p.clone()}</p>
                                }
                            })}
                        </div>
                        
                        <div class="ec-search">
                            <input
                                node_ref=input_ref
                                type="text"
                                class="ec-search-input"
                                placeholder={format!("Search {}...", entity_type.as_str())}
                                value=move || search_query.get()
                                on:input=move |ev| set_search_query.set(event_target_value(&ev))
                                on:keydown=handle_keydown
                            />
                        </div>
                        
                        <div class="ec-results">
                            {move || {
                                if let Some(error) = error_message.get() {
                                    view! {
                                        <div class="ec-error">
                                            <span class="ec-error-icon">"⚠️"</span>
                                            <span>{error}</span>
                                        </div>
                                    }.into_any()
                                } else if is_loading.get() && config.show_loading {
                                    view! {
                                        <div class="ec-loading">
                                            <div class="ec-spinner"></div>
                                            <span>"Loading..."</span>
                                        </div>
                                    }.into_any()
                                } else if results.get().is_empty() {
                                    let et = entity_type.clone();
                                    view! {
                                        <div class="ec-empty">
                                            {move || if search_query.get().is_empty() {
                                                format!("No {} found", et.as_str())
                                            } else {
                                                format!("No {} matching '{}'", et.as_str(), search_query.get())
                                            }}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="ec-list">
                                            <For
                                                each=move || results.get().into_iter().take(config.max_suggestions).enumerate()
                                                key=|(_, entity)| entity.id.clone()
                                                children=move |(index, entity)| {
                                                    let is_selected = move || selected_index.get() == index;
                                                    let entity_for_click = entity.clone();
                                                    let color = EntityType::from(entity.entity_type.clone()).color();
                                                    
                                                    view! {
                                                        <button
                                                            id={format!("ec-item-{}", index)}
                                                            class={move || if is_selected() {
                                                                "ec-item ec-item-selected"
                                                            } else {
                                                                "ec-item"
                                                            }}
                                                            on:click=move |_| handle_select(entity_for_click.clone())
                                                            on:mouseenter=move |_| set_selected_index.set(index)
                                                        >
                                                            <div class={format!("ec-avatar ec-avatar-{}", color)}>
                                                                {entity.name.chars().next().unwrap_or('?').to_uppercase().to_string()}
                                                            </div>
                                                            <div class="ec-item-content">
                                                                <div class="ec-item-name">{entity.name.clone()}</div>
                                                                {entity.description.as_ref().map(|desc| {
                                                                    view! {
                                                                        <div class="ec-item-desc">{desc.clone()}</div>
                                                                    }
                                                                })}
                                                            </div>
                                                            {move || if is_selected() {
                                                                view! {
                                                                    <div class="ec-item-check">"✓"</div>
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
                                }
                            }}
                        </div>
                        
                        <div class="ec-footer">
                            <button
                                class="ec-cancel-btn"
                                on:click=move |_| handle_cancel()
                            >
                                "Cancel"
                            </button>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}