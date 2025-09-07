use leptos::prelude::*;
use leptos::html::Input;
use crate::types::{Entity, SelectionState};
use crate::virtual_list::VirtualList;

#[component]
pub fn MultiSelectCombobox<F>(
    #[prop(into)] placeholder: String,
    #[prop(into)] _entity_type: String,
    #[prop(default = None)] max_selections: Option<usize>,
    #[prop(default = vec![])] initial_selections: Vec<Entity>,
    #[prop(default = vec![])] available_entities: Vec<Entity>,
    on_change: F,
) -> impl IntoView 
where
    F: Fn(Vec<Entity>) + 'static,
{
    let input_ref = NodeRef::<Input>::new();
    
    // State
    let (selection_state, set_selection_state) = signal(
        SelectionState::new(max_selections).with_initial(initial_selections)
    );
    let (search_query, set_search_query) = signal(String::new());
    let (is_open, set_is_open) = signal(false);
    let (highlighted_index, set_highlighted_index) = signal(0);
    let (all_entities, _set_all_entities) = signal(available_entities);
    let (is_loading, _set_is_loading) = signal(false);
    
    // Filter entities based on search and selection
    let filtered_entities = Memo::new(move |_| {
        let query = search_query.get();
        let state = selection_state.get();
        
        all_entities.get()
            .into_iter()
            .filter(|entity| {
                // Don't show already selected items
                !state.is_selected(&entity.id) &&
                // Filter by search query
                (query.is_empty() || 
                 entity.name.to_lowercase().contains(&query.to_lowercase()) ||
                 entity.description.as_ref()
                    .map(|d| d.to_lowercase().contains(&query.to_lowercase()))
                    .unwrap_or(false))
            })
            .collect::<Vec<_>>()
    });
    
    // Notify parent on selection change
    Effect::new(move |_| {
        let state = selection_state.get();
        on_change(state.selected.clone());
    });
    
    // Add entity to selection
    let add_entity = move |entity: Entity| {
        set_selection_state.update(|state| {
            state.add(entity);
        });
        set_search_query.set(String::new());
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    };
    
    // Remove entity from selection
    let remove_entity = move |entity_id: String| {
        set_selection_state.update(|state| {
            state.remove(&entity_id);
        });
    };
    
    // Clear all selections
    let clear_all = move |_| {
        set_selection_state.update(|state| {
            state.clear();
        });
    };
    
    // Handle keyboard navigation
    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        let key = ev.key();
        match key.as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                if !is_open.get() {
                    set_is_open.set(true);
                } else {
                    let max = filtered_entities.get().len().saturating_sub(1);
                    set_highlighted_index.update(|idx| *idx = (*idx + 1).min(max));
                }
            },
            "ArrowUp" => {
                ev.prevent_default();
                set_highlighted_index.update(|idx| *idx = idx.saturating_sub(1));
            },
            "Enter" => {
                ev.prevent_default();
                if is_open.get() {
                    if let Some(entity) = filtered_entities.get().get(highlighted_index.get()) {
                        add_entity(entity.clone());
                    }
                } else {
                    set_is_open.set(true);
                }
            },
            "Escape" => {
                ev.prevent_default();
                set_is_open.set(false);
                set_search_query.set(String::new());
            },
            "Backspace" => {
                if search_query.get().is_empty() {
                    // Remove last selected item
                    set_selection_state.update(|state| {
                        if let Some(last) = state.selected.last() {
                            let id = last.id.clone();
                            state.remove(&id);
                        }
                    });
                }
            },
            _ => {}
        }
    };
    
    // Render item for virtual list
    let render_item = move |idx: usize, entity: Entity| {
        let is_highlighted = move || highlighted_index.get() == idx;
        let entity_for_click = entity.clone();
        
        view! {
            <div
                class="msc-item"
                class:msc-item-highlighted=is_highlighted
                on:click=move |_| add_entity(entity_for_click.clone())
                on:mouseenter=move |_| set_highlighted_index.set(idx)
            >
                <div class="msc-item-icon">
                    {entity.name.chars().next().unwrap_or('?').to_uppercase().to_string()}
                </div>
                <div class="msc-item-content">
                    <div class="msc-item-name">{entity.name.clone()}</div>
                    {entity.description.map(|desc| {
                        view! {
                            <div class="msc-item-description">{desc}</div>
                        }
                    })}
                </div>
            </div>
        }
    };
    
    view! {
        <div class="msc-container">
            <div class="msc-input-wrapper">
                // Selected items
                <div class="msc-selections">
                    <For
                        each=move || selection_state.get().selected.clone()
                        key=|entity| entity.id.clone()
                        children=move |entity| {
                            let entity_id = entity.id.clone();
                            view! {
                                <div class="msc-tag">
                                    <span class="msc-tag-text">{entity.name}</span>
                                    <button
                                        class="msc-tag-remove"
                                        on:click=move |ev| {
                                            ev.prevent_default();
                                            ev.stop_propagation();
                                            remove_entity(entity_id.clone());
                                        }
                                    >
                                        "×"
                                    </button>
                                </div>
                            }
                        }
                    />
                    
                    // Search input
                    <input
                        node_ref=input_ref
                        type="text"
                        class="msc-input"
                        placeholder=move || {
                            let state = selection_state.get();
                            if state.selected.is_empty() {
                                placeholder.clone()
                            } else if !state.can_add_more() {
                                format!("Maximum {} selections reached", max_selections.unwrap_or(0))
                            } else {
                                "Add more...".to_string()
                            }
                        }
                        prop:value=move || search_query.get()
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        on:keydown=handle_keydown
                        on:focus=move |_| set_is_open.set(true)
                        on:blur=move |_| {
                            // Delay to allow click events to fire
                            set_timeout(move || set_is_open.set(false), std::time::Duration::from_millis(200));
                        }
                        disabled=move || !selection_state.get().can_add_more()
                    />
                </div>
                
                // Clear button
                <Show when=move || !selection_state.get().selected.is_empty()>
                    <button
                        class="msc-clear-btn"
                        on:click=clear_all
                        title="Clear all selections"
                    >
                        "Clear"
                    </button>
                </Show>
            </div>
            
            // Dropdown with virtual scrolling
            <Show when=move || is_open.get() && selection_state.get().can_add_more()>
                <div class="msc-dropdown">
                    {move || if is_loading.get() {
                        view! {
                            <div class="msc-loading">
                                <div class="msc-spinner"></div>
                                <span>"Loading..."</span>
                            </div>
                        }.into_any()
                    } else if filtered_entities.get().is_empty() {
                        view! {
                            <div class="msc-empty">
                                {if search_query.get().is_empty() {
                                    "No more items to select"
                                } else {
                                    "No matching items found"
                                }}
                            </div>
                        }.into_any()
                    } else {
                        let entities = filtered_entities.get();
                        if entities.len() > 50 {
                            // Use virtual scrolling for large lists
                            view! {
                                <VirtualList
                                    items=Signal::derive(move || filtered_entities.get())
                                    item_height=60.0
                                    container_height=300.0
                                    render_item=render_item
                                />
                            }.into_any()
                        } else {
                            // Regular rendering for small lists
                            view! {
                                <div class="msc-dropdown-content">
                                    <For
                                        each=move || filtered_entities.get().into_iter().enumerate()
                                        key=|(_, entity)| entity.id.clone()
                                        children=move |(idx, entity)| {
                                            render_item(idx, entity)
                                        }
                                    />
                                </div>
                            }.into_any()
                        }
                    }}
                </div>
            </Show>
            
            // Selection count
            <div class="msc-status">
                <span class="msc-count">
                    {move || {
                        let state = selection_state.get();
                        let count = state.count();
                        match max_selections {
                            Some(max) => format!("{}/{} selected", count, max),
                            None => format!("{} selected", count)
                        }
                    }}
                </span>
                {move || {
                    let state = selection_state.get();
                    if !state.can_add_more() {
                        view! {
                            <span class="msc-limit-reached">
                                "(limit reached)"
                            </span>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

// Helper to set timeout
fn set_timeout<F>(f: F, duration: std::time::Duration)
where
    F: FnOnce() + 'static,
{
    use gloo_timers::callback::Timeout;
    Timeout::new(duration.as_millis() as u32, f).forget();
}