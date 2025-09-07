use leptos::prelude::*;
use leptos::logging;
use web_sys::KeyboardEvent;
use crate::types::*;
use crate::parser::{CommandParser, validate_command};

#[component]
pub fn CommandEntityInput<F>(
    #[prop(optional)] initial_value: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] on_submit: Option<F>,
    #[prop(optional)] show_debug: bool,
) -> impl IntoView
where
    F: Fn(String, ParseResult) + 'static + Clone,
{
    let (input_value, set_input_value) = signal(initial_value.unwrap_or_default());
    let (mode, set_mode) = signal(InputMode::AI);
    let (parse_result, set_parse_result) = signal::<Option<ParseResult>>(None);
    let (show_entity_selector, set_show_entity_selector) = signal(false);
    let (entity_filter, set_entity_filter) = signal(String::new());
    let (selected_entity_index, set_selected_entity_index) = signal(0);
    let (validation_error, set_validation_error) = signal::<Option<String>>(None);
    
    let parser = CommandParser::new();
    let entities = parser.default_entities();
    
    // Parse input as user types
    create_effect(move |_| {
        let value = input_value.get();
        if value.is_empty() {
            set_parse_result.set(None);
            set_mode.set(InputMode::AI);
            set_show_entity_selector.set(false);
            set_validation_error.set(None);
            return;
        }
        
        let result = parser.parse(&value);
        
        // Update mode
        set_mode.set(result.mode.clone());
        
        // Show entity selector
        set_show_entity_selector.set(result.requesting_entity);
        
        // Validate if in command mode
        if result.mode == InputMode::Command {
            match validate_command(&result) {
                Ok(_) => set_validation_error.set(None),
                Err(e) => set_validation_error.set(Some(e)),
            }
        }
        
        set_parse_result.set(Some(result));
    });
    
    // Filter entities based on input after @
    let filtered_entities = create_memo(move |_| {
        let filter = entity_filter.get();
        let parse = parse_result.get();
        
        if let Some(result) = parse {
            if result.requesting_entity {
                // Get text after @
                let input = input_value.get();
                if let Some(at_pos) = result.at_position {
                    let after_at = &input[at_pos + 1..];
                    if !after_at.is_empty() {
                        return entities.iter()
                            .filter(|e| {
                                e.name.to_lowercase().contains(&after_at.to_lowercase()) ||
                                e.plural.to_lowercase().contains(&after_at.to_lowercase()) ||
                                e.aliases.iter().any(|a| a.to_lowercase().contains(&after_at.to_lowercase()))
                            })
                            .cloned()
                            .collect::<Vec<_>>();
                    }
                }
            }
        }
        
        if filter.is_empty() {
            entities.clone()
        } else {
            entities.iter()
                .filter(|e| {
                    e.name.to_lowercase().contains(&filter.to_lowercase()) ||
                    e.plural.to_lowercase().contains(&filter.to_lowercase()) ||
                    e.description.to_lowercase().contains(&filter.to_lowercase())
                })
                .cloned()
                .collect()
        }
    });
    
    let handle_keydown = {
        let on_submit = on_submit.clone();
        move |e: KeyboardEvent| {
            let key = e.key();
            
            if show_entity_selector.get() {
                match key.as_str() {
                    "ArrowDown" => {
                        e.prevent_default();
                        let entities = filtered_entities.get();
                        if !entities.is_empty() {
                            set_selected_entity_index.update(|i| {
                                *i = (*i + 1) % entities.len();
                            });
                        }
                    },
                    "ArrowUp" => {
                        e.prevent_default();
                        let entities = filtered_entities.get();
                        if !entities.is_empty() {
                            set_selected_entity_index.update(|i| {
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
                        let entities = filtered_entities.get();
                        if !entities.is_empty() {
                            let entity = &entities[selected_entity_index.get()];
                            select_entity(entity.name.clone(), &input_value, &set_input_value, &parse_result);
                            set_show_entity_selector.set(false);
                            set_selected_entity_index.set(0);
                        }
                    },
                    "Escape" => {
                        e.prevent_default();
                        set_show_entity_selector.set(false);
                        set_selected_entity_index.set(0);
                    },
                    _ => {}
                }
            } else if key == "Enter" {
                e.prevent_default();
                
                if validation_error.get().is_none() {
                    let value = input_value.get();
                    let result = parse_result.get().unwrap_or_default();
                    
                    logging::log!("Submitting: {} (Mode: {:?})", value, result.mode);
                    
                    if let Some(ref on_submit_fn) = on_submit {
                        on_submit_fn(value.clone(), result);
                    }
                    
                    // Clear input
                    set_input_value.set(String::new());
                    set_parse_result.set(None);
                }
            }
        }
    };
    
    let select_entity_click = move |entity_name: String| {
        select_entity(entity_name, &input_value, &set_input_value, &parse_result);
        set_show_entity_selector.set(false);
        set_selected_entity_index.set(0);
    };
    
    view! {
        <div class="cei-container">
            <style>{include_str!("styles.css")}</style>
            
            // Mode indicator
            <div class="cei-mode-indicator">
                {move || match mode.get() {
                    InputMode::Command => view! {
                        <span class="cei-mode-command">
                            <span class="cei-mode-icon">"⚡"</span>
                            " Command Mode"
                        </span>
                    }.into_view(),
                    InputMode::AI => view! {
                        <span class="cei-mode-ai">
                            <span class="cei-mode-icon">"🤖"</span>
                            " AI Mode"
                        </span>
                    }.into_view(),
                }}
            </div>
            
            // Visual tags for detected command and entity
            <Show when=move || parse_result.get().is_some()>
                <div class="cei-tokens">
                    {move || {
                        parse_result.get().and_then(|r| r.command.clone()).map(|cmd| view! {
                            <span class="cei-token cei-token-command">
                                "Command: " {cmd}
                            </span>
                        })
                    }}
                    
                    {move || {
                        parse_result.get().and_then(|r| r.entity.clone()).map(|ent| view! {
                            <span class="cei-token cei-token-entity">
                                "Entity: " {ent}
                            </span>
                        })
                    }}
                    
                    {move || {
                        let params = parse_result.get()
                            .map(|r| r.parameters.clone())
                            .unwrap_or_default();
                        if !params.is_empty() {
                            Some(view! {
                                <span class="cei-token cei-token-params">
                                    "Params: " {params.join(", ")}
                                </span>
                            })
                        } else {
                            None
                        }
                    }}
                </div>
            </Show>
            
            // Entity selector popup
            <Show when=move || show_entity_selector.get()>
                <div class="cei-entity-selector">
                    <div class="cei-entity-header">
                        "Select an entity:"
                    </div>
                    <div class="cei-entity-list">
                        {move || {
                            let entities = filtered_entities.get();
                            let selected_idx = selected_entity_index.get();
                            entities.into_iter().enumerate().map(|(idx, entity)| {
                                let entity_name = entity.name.clone();
                                let is_selected = idx == selected_idx;
                                view! {
                                    <button
                                        class=move || if is_selected {
                                            "cei-entity-item cei-entity-item-selected"
                                        } else {
                                            "cei-entity-item"
                                        }
                                        on:click=move |_| select_entity_click(entity_name.clone())
                                    >
                                        <span class="cei-entity-name">{entity.name.clone()}</span>
                                        <span class="cei-entity-desc">{entity.description}</span>
                                    </button>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </div>
                    <div class="cei-entity-hint">
                        "Use ↑↓ to navigate, Enter to select, Esc to cancel"
                    </div>
                </div>
            </Show>
            
            // Validation error
            <Show when=move || validation_error.get().is_some()>
                {move || validation_error.get().map(|err| view! {
                    <div class="cei-error">
                        "⚠️ " {err}
                    </div>
                })}
            </Show>
            
            // Input field
            <input
                type="text"
                class="cei-input"
                placeholder=move || placeholder.clone().unwrap_or_else(|| 
                    "Type 'create invoice' or 'update @' to see entities...".to_string()
                )
                prop:value=move || input_value.get()
                on:input=move |e| {
                    let value = event_target_value(&e);
                    set_input_value.set(value);
                }
                on:keydown=handle_keydown
            />
            
            // Help text
            <div class="cei-help">
                "Commands: create, update, delete, list, search, generate | Type @ to see entities | Press Enter to submit"
            </div>
            
            // Debug info
            <Show when=move || show_debug>
                <div class="cei-debug">
                    <div>"Mode: " {move || format!("{:?}", mode.get())}</div>
                    <div>"Parse Result: " {move || format!("{:?}", parse_result.get())}</div>
                    <div>"Validation: " {move || format!("{:?}", validation_error.get())}</div>
                </div>
            </Show>
        </div>
    }
}

fn select_entity(
    entity_name: String,
    input_value: &RwSignal<String>,
    set_input_value: &WriteSignal<String>,
    parse_result: &ReadSignal<Option<ParseResult>>,
) {
    let current = input_value.get();
    let new_value = if let Some(result) = parse_result.get() {
        if let Some(at_pos) = result.at_position {
            // Replace everything after @ with the entity name
            format!("{}{} ", &current[..at_pos + 1], entity_name)
        } else {
            format!("{} @{} ", current, entity_name)
        }
    } else {
        format!("{} @{} ", current, entity_name)
    };
    set_input_value.set(new_value);
}