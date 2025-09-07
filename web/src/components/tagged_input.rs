use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use crate::components::entity_tag::EntityToken;

#[component]
pub fn TaggedInput(
    tokens: Signal<Vec<EntityToken>>,
    set_tokens: WriteSignal<Vec<EntityToken>>,
    input_value: Signal<String>,
    set_input_value: WriteSignal<String>,
    on_at_typed: WriteSignal<bool>,
    placeholder: Signal<String>,
    #[prop(default = false.into())] locked: Signal<bool>,
) -> impl IntoView {
    let input_ref = NodeRef::<leptos::html::Div>::new();
    let (is_focused, set_is_focused) = signal(false);
    
    // Handle clicking on the wrapper to focus the editable area
    let handle_wrapper_click = move |_| {
        if let Some(input) = input_ref.get() {
            let element: &HtmlElement = input.unchecked_ref();
            element.focus().ok();
        }
    };
    
    // Handle input changes
    let handle_input = move |ev: web_sys::Event| {
        if locked.get() {
            ev.prevent_default();
            return;
        }
        
        let target = ev.target().unwrap();
        let element: &HtmlElement = target.unchecked_ref();
        let text = element.inner_text();
        
        // Check if @ was typed
        if text.contains('@') && !input_value.get().contains('@') {
            on_at_typed.set(true);
        }
        
        set_input_value.set(text);
    };
    
    // Handle keydown events
    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        let key = ev.key();
        
        // If locked, only allow navigation keys
        if locked.get() {
            if !matches!(key.as_str(), "ArrowUp" | "ArrowDown" | "Enter" | "Tab" | "Escape") {
                ev.prevent_default();
            }
            return;
        }
        
        // Handle backspace to delete tokens
        if key == "Backspace" && input_value.get().is_empty() && !tokens.get().is_empty() {
            ev.prevent_default();
            set_tokens.update(|t| {
                t.pop();
            });
        }
    };
    
    // Handle token removal
    let handle_remove_token = move |token_id: String| {
        set_tokens.update(|tokens| {
            tokens.retain(|t| t.id != token_id);
        });
    };
    
    view! {
        <div 
            class={move || format!(
                "flex flex-wrap items-center gap-2 p-3 bg-white border rounded-lg cursor-text min-h-[48px] {}",
                if is_focused.get() { "border-blue-500 ring-2 ring-blue-200" } else { "border-gray-300" }
            )}
            on:click=handle_wrapper_click
        >
            // Render tokens as inline tags
            <For
                each=move || tokens.get()
                key=|token| token.id.clone()
                children=move |token| {
                    let token_id = token.id.clone();
                    let display_name = token.display_name.clone();
                    let entity_type = token.entity_type.clone();
                    
                    // Determine color based on entity type
                    let bg_color = match entity_type.as_str() {
                        "client" => "bg-blue-100 text-blue-800 border-blue-200",
                        "supplier" => "bg-green-100 text-green-800 border-green-200",
                        "product" => "bg-purple-100 text-purple-800 border-purple-200",
                        "employee" => "bg-orange-100 text-orange-800 border-orange-200",
                        "invoice" => "bg-indigo-100 text-indigo-800 border-indigo-200",
                        "payment" => "bg-pink-100 text-pink-800 border-pink-200",
                        _ => "bg-gray-100 text-gray-800 border-gray-200",
                    };
                    
                    view! {
                        <span class={format!("inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border {}", bg_color)}>
                            <span class="mr-1">{display_name}</span>
                            <button
                                class="ml-1 hover:opacity-70 focus:outline-none"
                                on:click=move |ev| {
                                    ev.stop_propagation();
                                    handle_remove_token(token_id.clone());
                                }
                            >
                                <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"/>
                                </svg>
                            </button>
                        </span>
                    }
                }
            />
            
            // Editable input area
            <div
                node_ref=input_ref
                contenteditable="true"
                class={move || format!(
                    "flex-1 outline-none min-w-[200px] {}",
                    if tokens.get().is_empty() && input_value.get().is_empty() { "text-gray-400" } else { "text-gray-900" }
                )}
                on:input=handle_input
                on:keydown=handle_keydown
                on:focus=move |_| set_is_focused.set(true)
                on:blur=move |_| set_is_focused.set(false)
                data-placeholder={move || if tokens.get().is_empty() && input_value.get().is_empty() { placeholder.get() } else { String::new() }}
                style="min-height: 1.5rem;"
            >
                {move || if !is_focused.get() && tokens.get().is_empty() && input_value.get().is_empty() {
                    placeholder.get()
                } else {
                    input_value.get()
                }}
            </div>
        </div>
    }
}