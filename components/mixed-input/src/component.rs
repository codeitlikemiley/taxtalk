use leptos::prelude::*;
use leptos::logging;
use web_sys::KeyboardEvent;
use crate::types::{EntityToken, MixedContent, ParsedSegment};

#[component]
pub fn MixedInput<F, G>(
    #[prop(optional)] initial_value: Option<String>,
    #[prop(optional)] initial_entities: Option<Vec<EntityToken>>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] max_length: Option<usize>,
    #[prop(optional)] on_submit: Option<F>,
    #[prop(optional)] on_at_typed: Option<G>,
    #[prop(optional)] show_helper: bool,
) -> impl IntoView
where
    F: Fn(MixedContent) + 'static + Clone,
    G: Fn(usize, String) + 'static + Clone,
{
    let (value, set_value) = signal(initial_value.unwrap_or_default());
    let (entities, set_entities) = signal(initial_entities.unwrap_or_default());
    let (is_focused, set_is_focused) = signal(false);
    let (char_count, set_char_count) = signal(0);
    
    // Update character count
    create_effect(move |_| {
        let text = value.get();
        set_char_count.set(text.len());
    });
    
    // Build mixed content from current state
    let get_mixed_content = move || {
        let mut content = MixedContent::new();
        let text = value.get();
        let entities_list = entities.get();
        
        // Parse the text and rebuild segments
        let mut remaining = text.as_str();
        let mut segments = Vec::new();
        
        for entity in &entities_list {
            let entity_ref = format!("@entity_{}", entity.id);
            if let Some(pos) = remaining.find(&entity_ref) {
                if pos > 0 {
                    segments.push((pos, ParsedSegment::Text(remaining[..pos].to_string())));
                }
                segments.push((pos, ParsedSegment::Entity(entity.clone())));
                remaining = &remaining[pos + entity_ref.len()..];
            }
        }
        
        if !remaining.is_empty() {
            segments.push((usize::MAX, ParsedSegment::Text(remaining.to_string())));
        }
        
        // Sort segments by position and add to content
        segments.sort_by_key(|s| s.0);
        for (_, segment) in segments {
            content.segments.push(segment);
        }
        
        content.raw_text = text;
        content
    };
    
    // Display value with entity names
    let display_value = create_memo(move |_| {
        let text = value.get();
        let entities_list = entities.get();
        
        let mut result = text.clone();
        for entity in entities_list {
            let from = format!("@entity_{}", entity.id);
            let to = format!("@{}", entity.display_name);
            result = result.replace(&from, &to);
        }
        result
    });
    
    // Handle input changes
    let handle_input = move |ev: web_sys::Event| {
        let new_value = event_target_value(&ev);
        
        // Check max length
        if let Some(max) = max_length {
            if new_value.len() > max {
                return;
            }
        }
        
        // Check for @ mentions
        if let Some(ref on_at) = on_at_typed {
            if let Some(at_pos) = new_value.rfind('@') {
                let after_at = &new_value[at_pos + 1..];
                if !after_at.contains(' ') {
                    on_at(at_pos, after_at.to_string());
                }
            }
        }
        
        set_value.set(new_value);
    };
    
    // Handle keydown events
    let handle_keydown = move |ev: KeyboardEvent| {
        let key = ev.key();
        
        match key.as_str() {
            "Enter" => {
                if !ev.shift_key() && !disabled {
                    ev.prevent_default();
                    if let Some(ref on_submit_fn) = on_submit {
                        let content = get_mixed_content();
                        on_submit_fn(content);
                        // Clear after submit
                        set_value.set(String::new());
                        set_entities.set(Vec::new());
                    }
                }
            },
            "Backspace" => {
                let current_value = value.get();
                if current_value.is_empty() && !entities.get().is_empty() {
                    // Remove last entity if input is empty
                    set_entities.update(|e| { e.pop(); });
                } else {
                    // Check if we're deleting an entity reference
                    for entity in entities.get() {
                        let entity_ref = format!("@entity_{}", entity.id);
                        if current_value.ends_with(&entity_ref[..entity_ref.len()-1]) {
                            // Remove this entity
                            let entity_id = entity.id.clone();
                            set_entities.update(|entities| {
                                entities.retain(|e| e.id != entity_id);
                            });
                            break;
                        }
                    }
                }
            },
            _ => {}
        }
    };
    
    // Add entity to the input
    let add_entity = move |entity: EntityToken| {
        set_entities.update(|entities| {
            entities.push(entity.clone());
        });
        let entity_ref = format!("@entity_{} ", entity.id);
        set_value.update(|v| v.push_str(&entity_ref));
    };
    
    // Remove an entity
    let remove_entity = move |entity_id: String| {
        set_entities.update(|entities| {
            entities.retain(|e| e.id != entity_id);
        });
        // Clean up the text
        let current_text = value.get();
        let entity_ref = format!("@entity_{}", entity_id);
        let new_text = current_text.replace(&entity_ref, "");
        set_value.set(new_text.trim().to_string());
    };
    
    view! {
        <div class="mi-container">
            <style>{include_str!("styles.css")}</style>
            
            // Display selected entities as tags
            <Show when=move || !entities.get().is_empty()>
                <div class="mi-entity-tags">
                    <For
                        each=move || entities.get()
                        key=|entity| entity.id.clone()
                        children=move |entity| {
                            let entity_id = entity.id.clone();
                            let entity_type = entity.entity_type.clone();
                            view! {
                                <div class=move || format!("mi-entity-tag mi-entity-{}", entity_type)>
                                    <span class="mi-entity-type">{entity.entity_type.clone()}</span>
                                    <span class="mi-entity-name">{entity.display_name.clone()}</span>
                                    <button
                                        class="mi-entity-remove"
                                        on:click=move |ev| {
                                            ev.prevent_default();
                                            remove_entity(entity_id.clone());
                                        }
                                        disabled=disabled
                                    >
                                        "×"
                                    </button>
                                </div>
                            }
                        }
                    />
                </div>
            </Show>
            
            // The main textarea input
            <div class="mi-input-wrapper">
                <textarea
                    class=move || {
                        let mut classes = vec!["mi-textarea"];
                        if disabled { classes.push("mi-disabled"); }
                        if is_focused.get() { classes.push("mi-focused"); }
                        classes.join(" ")
                    }
                    prop:value=display_value
                    on:input=handle_input
                    on:keydown=handle_keydown
                    on:focus=move |_| set_is_focused.set(true)
                    on:blur=move |_| set_is_focused.set(false)
                    placeholder=move || placeholder.clone().unwrap_or_else(|| 
                        "Type your message... Use @ to mention entities".to_string()
                    )
                    disabled=disabled
                    rows="3"
                />
                
                // Character count
                <Show when=move || max_length.is_some()>
                    <div class=move || {
                        let count = char_count.get();
                        let max = max_length.unwrap_or(0);
                        let ratio = count as f32 / max as f32;
                        let class = if ratio > 0.9 {
                            "mi-char-count mi-char-warning"
                        } else {
                            "mi-char-count"
                        };
                        class
                    }>
                        {move || format!("{}/{}", char_count.get(), max_length.unwrap_or(0))}
                    </div>
                </Show>
            </div>
            
            // Helper text
            <Show when=move || show_helper && !disabled && is_focused.get()>
                <div class="mi-helper">
                    <span class="mi-helper-item">
                        <kbd>"@"</kbd> " mention entities"
                    </span>
                    <span class="mi-helper-item">
                        <kbd>"Enter"</kbd> " to send"
                    </span>
                    <span class="mi-helper-item">
                        <kbd>"Shift+Enter"</kbd> " new line"
                    </span>
                </div>
            </Show>
        </div>
    }
}

// Export helper function to create entities
pub fn create_entity(entity_type: &str, display_name: &str) -> EntityToken {
    EntityToken::new(entity_type, display_name)
}