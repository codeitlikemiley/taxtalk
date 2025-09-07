use leptos::prelude::*;
use leptos::ev;
use serde_json::json;

// Simplified form modal for complex tokens
#[component]
pub fn SimpleForm(
    show: Signal<bool>,
    title: String,
    token_name: String,
    token_type: String,
    prompt: String,
    on_submit: WriteSignal<Option<String>>,
    on_cancel: WriteSignal<bool>,
) -> impl IntoView {
    let (value, set_value) = signal(String::new());
    
    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        on_submit.set(Some(value.get()));
        set_value.set(String::new());
    };
    
    let handle_cancel = move |_| {
        on_cancel.set(false);
        set_value.set(String::new());
    };
    
    view! {
        <Show when=move || show.get()>
            <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                <div class="bg-white rounded-2xl shadow-2xl max-w-md w-full">
                    // Header
                    <div class="bg-gradient-to-r from-blue-500 to-purple-600 text-white px-6 py-4 rounded-t-2xl">
                        <div class="flex items-center justify-between">
                            <h2 class="text-xl font-semibold">{title.clone()}</h2>
                            <button
                                class="text-white hover:text-gray-200 transition-colors"
                                on:click=handle_cancel
                            >
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                </svg>
                            </button>
                        </div>
                    </div>
                    
                    // Body
                    <form on:submit=handle_submit>
                        <div class="px-6 py-4">
                            <label class="block text-sm font-medium text-gray-700 mb-2">
                                {prompt.clone()}
                            </label>
                            {render_input(token_type.clone(), value, set_value)}
                        </div>
                        
                        // Footer
                        <div class="bg-gray-50 px-6 py-4 flex justify-end space-x-3 rounded-b-2xl">
                            <button
                                type="button"
                                class="px-5 py-2 text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
                                on:click=handle_cancel
                            >
                                "Cancel"
                            </button>
                            <button
                                type="submit"
                                class="px-5 py-2 bg-gradient-to-r from-blue-500 to-purple-600 text-white rounded-lg hover:from-blue-600 hover:to-purple-700 transition-all shadow-lg hover:shadow-xl"
                            >
                                "Submit"
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </Show>
    }
}

fn render_input(
    token_type: String,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
) -> impl IntoView {
    match token_type.as_str() {
        "Number" => view! {
            <input
                type="number"
                step="0.01"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                prop:value=move || value.get()
                on:input=move |ev| set_value.set(event_target_value(&ev))
                required=true
                autofocus=true
            />
        }.into_any(),
        
        "Date" => view! {
            <input
                type="date"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                prop:value=move || value.get()
                on:input=move |ev| set_value.set(event_target_value(&ev))
                required=true
            />
        }.into_any(),
        
        "Boolean" => view! {
            <div class="flex items-center space-x-4">
                <button
                    type="button"
                    class={move || if value.get() == "true" {
                        "px-4 py-2 bg-blue-500 text-white rounded-lg"
                    } else {
                        "px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300"
                    }}
                    on:click=move |_| set_value.set("true".to_string())
                >
                    "Yes"
                </button>
                <button
                    type="button"
                    class={move || if value.get() == "false" {
                        "px-4 py-2 bg-blue-500 text-white rounded-lg"
                    } else {
                        "px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300"
                    }}
                    on:click=move |_| set_value.set("false".to_string())
                >
                    "No"
                </button>
            </div>
        }.into_any(),
        
        _ => view! {
            <input
                type="text"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                prop:value=move || value.get()
                on:input=move |ev| set_value.set(event_target_value(&ev))
                required=true
                placeholder="Enter value..."
                autofocus=true
            />
        }.into_any(),
    }
}

// Invoice Items Form
#[component]
pub fn ItemsForm(
    show: Signal<bool>,
    on_submit: WriteSignal<Option<Vec<serde_json::Value>>>,
    on_cancel: WriteSignal<bool>,
) -> impl IntoView {
    let (items, set_items) = signal(vec![
        (RwSignal::new("".to_string()), RwSignal::new("1".to_string()), RwSignal::new("0".to_string()))
    ]);
    
    let add_item = move |_| {
        set_items.update(|items| {
            items.push((
                RwSignal::new("".to_string()),
                RwSignal::new("1".to_string()),
                RwSignal::new("0".to_string())
            ));
        });
    };
    
    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        
        let items_json: Vec<serde_json::Value> = items.get()
            .into_iter()
            .map(|(desc, qty, price)| json!({
                "description": desc.get(),
                "quantity": qty.get().parse::<f64>().unwrap_or(1.0),
                "unit_price": price.get().parse::<f64>().unwrap_or(0.0),
            }))
            .collect();
        
        on_submit.set(Some(items_json));
    };
    
    let handle_cancel = move |_| {
        on_cancel.set(false);
    };
    
    view! {
        <Show when=move || show.get()>
            <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                <div class="bg-white rounded-2xl shadow-2xl max-w-3xl w-full max-h-[80vh] overflow-hidden">
                    // Header
                    <div class="bg-gradient-to-r from-blue-500 to-purple-600 text-white px-6 py-4">
                        <h2 class="text-xl font-semibold">"Add Invoice Items"</h2>
                    </div>
                    
                    // Body
                    <form on:submit=handle_submit>
                        <div class="px-6 py-4 max-h-[50vh] overflow-y-auto">
                            <div class="space-y-3">
                                <For
                                    each=move || items.get().into_iter().enumerate()
                                    key=|(i, _)| *i
                                    children=move |(_index, (desc, qty, price))| {
                                        view! {
                                            <div class="border border-gray-200 rounded-lg p-3">
                                                <div class="grid grid-cols-12 gap-2">
                                                    <div class="col-span-5">
                                                        <input
                                                            type="text"
                                                            class="w-full px-2 py-1 border border-gray-300 rounded text-sm"
                                                            placeholder="Description"
                                                            prop:value=move || desc.get()
                                                            on:input=move |ev| desc.set(event_target_value(&ev))
                                                            required=true
                                                        />
                                                    </div>
                                                    <div class="col-span-2">
                                                        <input
                                                            type="number"
                                                            step="0.01"
                                                            class="w-full px-2 py-1 border border-gray-300 rounded text-sm"
                                                            placeholder="Qty"
                                                            prop:value=move || qty.get()
                                                            on:input=move |ev| qty.set(event_target_value(&ev))
                                                            required=true
                                                        />
                                                    </div>
                                                    <div class="col-span-3">
                                                        <input
                                                            type="number"
                                                            step="0.01"
                                                            class="w-full px-2 py-1 border border-gray-300 rounded text-sm"
                                                            placeholder="Price"
                                                            prop:value=move || price.get()
                                                            on:input=move |ev| price.set(event_target_value(&ev))
                                                            required=true
                                                        />
                                                    </div>
                                                    <div class="col-span-2 text-sm font-medium text-gray-700 py-1">
                                                        "₱" {move || {
                                                            let q = qty.get().parse::<f64>().unwrap_or(0.0);
                                                            let p = price.get().parse::<f64>().unwrap_or(0.0);
                                                            format!("{:.2}", q * p)
                                                        }}
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }
                                />
                                
                                <button
                                    type="button"
                                    class="w-full px-3 py-2 border-2 border-dashed border-gray-300 rounded-lg text-gray-600 hover:border-blue-500 hover:text-blue-500 transition-colors text-sm"
                                    on:click=add_item
                                >
                                    "+ Add Item"
                                </button>
                            </div>
                        </div>
                        
                        // Footer
                        <div class="bg-gray-50 px-6 py-4 flex justify-end space-x-3">
                            <button
                                type="button"
                                class="px-5 py-2 text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50"
                                on:click=handle_cancel
                            >
                                "Cancel"
                            </button>
                            <button
                                type="submit"
                                class="px-5 py-2 bg-gradient-to-r from-blue-500 to-purple-600 text-white rounded-lg hover:from-blue-600 hover:to-purple-700"
                            >
                                "Add Items"
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </Show>
    }
}