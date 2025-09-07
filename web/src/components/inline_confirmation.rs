use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParsedField {
    pub label: String,
    pub value: String,
    pub field_type: String,
    pub editable: bool,
}

#[component]
pub fn InlineConfirmation(
    title: String,
    fields: Vec<ParsedField>,
    message: Option<String>,
    on_confirm: WriteSignal<bool>,
    on_edit: WriteSignal<Option<String>>,
    on_cancel: WriteSignal<bool>,
) -> impl IntoView {
    let (edited_fields, set_edited_fields) = signal(fields.clone());
    let (is_editing, set_is_editing) = signal(false);
    
    let handle_field_change = move |index: usize, new_value: String| {
        set_edited_fields.update(|fields| {
            if let Some(field) = fields.get_mut(index) {
                field.value = new_value;
            }
        });
    };
    
    let handle_confirm = move |_| {
        on_confirm.set(true);
    };
    
    let handle_edit = move |_| {
        set_is_editing.set(true);
    };
    
    let handle_save_edits = move |_| {
        // Convert edited fields to JSON or some format
        let edited_json = serde_json::to_string(&edited_fields.get()).unwrap_or_default();
        on_edit.set(Some(edited_json));
        set_is_editing.set(false);
    };
    
    view! {
        <div class="fixed bottom-4 right-4 max-w-md bg-white rounded-2xl shadow-2xl border border-gray-200 overflow-hidden animate-slide-up z-50">
            <div class="px-4 py-3 bg-gradient-to-r from-green-50 to-blue-50 border-b border-gray-200">
                <h3 class="text-sm font-semibold text-gray-900">{title}</h3>
                {message.map(|msg| {
                    view! {
                        <p class="text-xs text-gray-600 mt-1">{msg}</p>
                    }.into_any()
                }).unwrap_or_else(|| view! { <span></span> }.into_any())}
            </div>
            
            <div class="px-4 py-3 max-h-64 overflow-y-auto">
                {move || if is_editing.get() {
                    view! {
                        <div class="space-y-2">
                            <For
                                each=move || edited_fields.get().into_iter().enumerate()
                                key=|(i, _)| *i
                                children=move |(index, field)| {
                                    if field.editable {
                                        view! {
                                            <div class="flex items-center space-x-2">
                                                <label class="text-xs font-medium text-gray-700 w-24">
                                                    {field.label.clone()}":"
                                                </label>
                                                <input
                                                    type="text"
                                                    class="flex-1 px-2 py-1 text-sm border border-gray-300 rounded focus:outline-none focus:border-blue-500"
                                                    prop:value=move || edited_fields.get()[index].value.clone()
                                                    on:input=move |ev| handle_field_change(index, event_target_value(&ev))
                                                />
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="flex items-center space-x-2">
                                                <span class="text-xs font-medium text-gray-700 w-24">
                                                    {field.label.clone()}":"
                                                </span>
                                                <span class="text-sm text-gray-900">{field.value.clone()}</span>
                                            </div>
                                        }.into_any()
                                    }
                                }
                            />
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="space-y-1">
                            <For
                                each=move || edited_fields.get()
                                key=|field| field.label.clone()
                                children=move |field| {
                                    let value_class = match field.field_type.as_str() {
                                        "amount" => "text-sm font-mono text-green-600",
                                        "entity" => "text-sm font-medium text-blue-600",
                                        "date" => "text-sm text-purple-600",
                                        _ => "text-sm text-gray-900"
                                    };
                                    
                                    view! {
                                        <div class="flex items-center space-x-2 py-1">
                                            <span class="text-xs text-gray-500 w-20">
                                                {field.label.clone()}":"
                                            </span>
                                            <span class=value_class>
                                                {field.value.clone()}
                                            </span>
                                            {if field.field_type == "entity" {
                                                view! {
                                                    <span class="text-xs text-green-500">"✓"</span>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }}
                                        </div>
                                    }
                                }
                            />
                        </div>
                    }.into_any()
                }}
            </div>
            
            <div class="px-4 py-3 bg-gray-50 border-t border-gray-200 flex items-center justify-between">
                <div class="text-xs text-gray-500">
                    "Press Enter to confirm, Esc to cancel"
                </div>
                <div class="flex space-x-2">
                    {move || if is_editing.get() {
                        view! {
                            <>
                                <button
                                    class="px-3 py-1.5 text-xs text-gray-600 hover:text-gray-800 transition-colors"
                                    on:click=move |_| set_is_editing.set(false)
                                >
                                    "Cancel Edit"
                                </button>
                                <button
                                    class="px-3 py-1.5 text-xs bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors"
                                    on:click=handle_save_edits
                                >
                                    "Save Changes"
                                </button>
                            </>
                        }.into_any()
                    } else {
                        view! {
                            <>
                                <button
                                    class="px-3 py-1.5 text-xs text-gray-600 hover:text-gray-800 transition-colors"
                                    on:click=move |_| on_cancel.set(true)
                                >
                                    "Cancel"
                                </button>
                                <button
                                    class="px-3 py-1.5 text-xs text-blue-600 hover:text-blue-700 transition-colors"
                                    on:click=handle_edit
                                >
                                    "Edit"
                                </button>
                                <button
                                    class="px-4 py-1.5 text-xs bg-green-500 hover:bg-green-600 text-white rounded-lg transition-colors font-medium"
                                    on:click=handle_confirm
                                >
                                    "Confirm"
                                </button>
                            </>
                        }.into_any()
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn SmartCommandInput(
    on_submit: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (input_value, set_input_value) = signal(String::new());
    let (is_parsing, set_is_parsing) = signal(false);
    let (parsed_preview, set_parsed_preview) = signal::<Option<String>>(None);
    let (confidence, set_confidence) = signal(0.0f32);
    
    // Parse input as user types (with debouncing in real app)
    Effect::new(move |_| {
        let value = input_value.get();
        if value.len() > 5 {
            // Would call NLP parser here
            set_is_parsing.set(true);
            // Simulate parsing
            spawn_local(async move {
                // In real app, would call /api/nlp/parse
                // Simulate delay with web timeout instead of tokio
                gloo_timers::future::TimeoutFuture::new(300).await;
                
                // Simulated parsed result
                if value.contains("invoice") && value.contains("5000") {
                    set_parsed_preview.set(Some(
                        "Create invoice for client with amount ₱5,000".to_string()
                    ));
                    set_confidence.set(0.85);
                } else {
                    set_parsed_preview.set(None);
                    set_confidence.set(0.0);
                }
                set_is_parsing.set(false);
            });
        } else {
            set_parsed_preview.set(None);
            set_confidence.set(0.0);
        }
    });
    
    let handle_submit = move |_| {
        let value = input_value.get();
        if !value.is_empty() {
            on_submit.set(Some(value));
            set_input_value.set(String::new());
        }
    };
    
    view! {
        <div class="relative">
            {move || if let Some(preview) = parsed_preview.get() {
                let conf_color = if confidence.get() > 0.8 {
                    "text-green-600"
                } else if confidence.get() > 0.6 {
                    "text-yellow-600"
                } else {
                    "text-red-600"
                };
                
                view! {
                    <div class="absolute bottom-full mb-2 left-0 right-0 bg-white rounded-lg shadow-lg border border-gray-200 p-3">
                        <div class="flex items-start justify-between">
                            <div class="flex-1">
                                <p class="text-xs text-gray-500 mb-1">
                                    "I understand you want to:"
                                </p>
                                <p class="text-sm text-gray-900">
                                    {preview}
                                </p>
                            </div>
                            <div class="ml-3">
                                <span class={format!("text-xs font-medium {}", conf_color)}>
                                    {format!("{}% confident", (confidence.get() * 100.0) as u32)}
                                </span>
                            </div>
                        </div>
                        <div class="mt-2 text-xs text-gray-500">
                            "Press Tab to accept, keep typing to change"
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
            
            <div class="flex items-center space-x-2">
                <input
                    type="text"
                    class="flex-1 px-4 py-3 border-2 border-gray-300 rounded-xl focus:outline-none focus:border-blue-500 text-sm"
                    placeholder="Type naturally: 'invoice ABC Corp 5000 pesos due next week'"
                    prop:value=move || input_value.get()
                    on:input=move |ev| set_input_value.set(event_target_value(&ev))
                    on:keydown=move |ev| {
                        if ev.key() == "Enter" {
                            handle_submit(());
                        } else if ev.key() == "Tab" && parsed_preview.get().is_some() {
                            ev.prevent_default();
                            // Accept the parsed version
                            if let Some(preview) = parsed_preview.get() {
                                on_submit.set(Some(preview));
                                set_input_value.set(String::new());
                            }
                        }
                    }
                />
                
                {move || if is_parsing.get() {
                    view! {
                        <div class="w-5 h-5 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
                    }.into_any()
                } else if confidence.get() > 0.6 {
                    view! {
                        <svg class="w-5 h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                        </svg>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>
        </div>
    }
}

use leptos::task::spawn_local;