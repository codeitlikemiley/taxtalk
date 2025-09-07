use leptos::prelude::*;
use leptos::ev;
use crate::components::entity_tag::EntityToken;

/// A mixed input component that supports both plain text and entity tags
/// For now, this is a simplified version using a textarea with entity display
#[component]
pub fn MixedInput(
    value: Signal<String>,
    set_value: WriteSignal<String>,
    entities: Signal<Vec<EntityToken>>,
    set_entities: WriteSignal<Vec<EntityToken>>,
    placeholder: String,
    #[prop(default = false.into())] disabled: Signal<bool>,
    #[prop(default = None)] on_submit: Option<WriteSignal<String>>,
    #[prop(default = None)] on_at_typed: Option<WriteSignal<(usize, String)>>,
) -> impl IntoView {
    let (is_focused, set_is_focused) = signal(false);
    
    // Serialize entities and text into a display string
    let display_value = move || {
        let text = value.get();
        let entities_list = entities.get();
        
        // Replace entity mentions with display format
        let mut result = text.clone();
        for entity in entities_list {
            // Simple replacement for display
            let from = format!("@entity_{}", entity.id);
            let to = format!("@{}", entity.display_name);
            result = result.replace(&from, &to);
        }
        result
    };
    
    // Handle input changes
    let handle_input = move |ev: ev::Event| {
        let new_value = event_target_value(&ev);
        
        // Check for @ mentions
        if let Some(on_at) = on_at_typed {
            if let Some(at_pos) = new_value.rfind('@') {
                let after_at = &new_value[at_pos + 1..];
                on_at.set((at_pos, after_at.to_string()));
            }
        }
        
        set_value.set(new_value);
    };
    
    // Handle keydown events
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        let key = ev.key();
        
        match key.as_str() {
            "Enter" => {
                if !ev.shift_key() && !disabled.get() {
                    ev.prevent_default();
                    if let Some(on_submit) = on_submit {
                        on_submit.set(value.get());
                    }
                }
            },
            "Backspace" => {
                // Check if we should remove an entity
                let current_value = value.get();
                if current_value.is_empty() && !entities.get().is_empty() {
                    // Remove last entity if input is empty
                    set_entities.update(|e| { e.pop(); });
                }
            },
            _ => {}
        }
    };
    
    // Remove an entity
    let remove_entity = move |entity_id: String| {
        set_entities.update(|entities| {
            entities.retain(|e| e.id != entity_id);
        });
        // Also clean up the text
        let current_text = value.get();
        let entity_ref = format!("@entity_{}", entity_id);
        let new_text = current_text.replace(&entity_ref, "");
        set_value.set(new_text.trim().to_string());
    };
    
    view! {
        <div class="relative">
            // Display selected entities as tags above the input
            <Show when=move || !entities.get().is_empty()>
                <div class="flex flex-wrap gap-2 mb-2">
                    <For
                        each=move || entities.get()
                        key=|entity| entity.id.clone()
                        children=move |entity| {
                            let entity_id = entity.id.clone();
                            let entity_name = entity.display_name.clone();
                            let entity_type = entity.entity_type.clone();
                            let color = get_entity_color(&entity_type);
                            
                            view! {
                                <div class={format!("inline-flex items-center gap-1 px-2 py-1 rounded-full text-sm {} text-white", color)}>
                                    <span>{entity_name}</span>
                                    <button
                                        class="ml-1 hover:bg-white hover:bg-opacity-20 rounded-full p-0.5"
                                        on:click=move |ev| {
                                            ev.prevent_default();
                                            remove_entity(entity_id.clone());
                                        }
                                        disabled=disabled
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
            </Show>
            
            // The actual input field
            <textarea
                class={move || {
                    let base = "w-full px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none";
                    let state = if disabled.get() {
                        "bg-gray-100 text-gray-500 cursor-not-allowed"
                    } else if is_focused.get() {
                        "bg-white border-blue-500"
                    } else {
                        "bg-white border-gray-300"
                    };
                    format!("{} {}", base, state)
                }}
                prop:value=display_value
                on:input=handle_input
                on:keydown=handle_keydown
                on:focus=move |_| set_is_focused.set(true)
                on:blur=move |_| set_is_focused.set(false)
                placeholder=placeholder
                disabled=disabled
                rows="2"
            />
            
            // Helper text
            <Show when=move || !disabled.get() && is_focused.get()>
                <div class="absolute -bottom-6 left-0 text-xs text-gray-500">
                    "Type @ to mention entities • Enter to send • Shift+Enter for new line"
                </div>
            </Show>
        </div>
    }
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