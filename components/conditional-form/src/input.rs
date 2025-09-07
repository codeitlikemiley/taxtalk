use leptos::prelude::*;
use serde_json::Value;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use taxtalk_ui_core::ComponentType;

#[component]
pub fn FormInput<V, D>(
    component: ComponentType,
    value: V,
    disabled: D,
    on_change: Callback<Value, ()>,
) -> impl IntoView 
where
    V: Fn() -> Option<Value> + 'static,
    D: Fn() -> bool + 'static,
{
    let (local_value, set_local_value) = signal(value());
    
    let display_value = {
        let component = component.clone();
        move || {
            local_value.get()
                .and_then(|v| match component {
                    ComponentType::AmountInput => v.as_f64().map(|f| format!("{:.2}", f)),
                    ComponentType::NumberInput => v.as_f64().map(|f| f.to_string()),
                    ComponentType::Boolean => v.as_bool().map(|b| b.to_string()),
                    _ => v.as_str().map(|s| s.to_string())
                })
                .unwrap_or_default()
        }
    };
    
    let is_disabled = disabled();
    let base_class = if is_disabled {
        "tui-cf-input-disabled"
    } else {
        ""
    };
    
    match component {
        ComponentType::Boolean => {
            let checked = local_value.get()
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            
            view! {
                <input
                    type="checkbox"
                    class=format!("tui-cf-checkbox {}", base_class)
                    checked=checked
                    disabled=is_disabled
                    on:change=move |e: web_sys::Event| {
                        if let Some(input) = e.target() {
                            let input: HtmlInputElement = input.dyn_into().unwrap();
                            let new_value = Value::from(input.checked());
                            set_local_value.set(Some(new_value.clone()));
                            on_change.run(new_value);
                        }
                    }
                />
            }.into_any()
        },
        ComponentType::Select(options) => {
            let current_value = local_value.get()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_default();
                
            view! {
                <select
                    class=format!("tui-cf-select {}", base_class)
                    disabled=is_disabled
                    on:change=move |e: web_sys::Event| {
                        if let Some(select) = e.target() {
                            let select: web_sys::HtmlSelectElement = select.dyn_into().unwrap();
                            let new_value = Value::from(select.value());
                            set_local_value.set(Some(new_value.clone()));
                            on_change.run(new_value);
                        }
                    }
                >
                    <option value="" selected=current_value.is_empty()>"-"</option>
                    {options.iter().map(|opt| {
                        let selected = current_value == *opt;
                        view! {
                            <option value=opt.clone() selected=selected>{opt.clone()}</option>
                        }
                    }).collect::<Vec<_>>()}
                </select>
            }.into_any()
        },
        ComponentType::NumberInput => {
            view! {
                <input
                    type="number"
                    class=format!("tui-cf-input tui-cf-number {}", base_class)
                    value=display_value
                    disabled=is_disabled
                    on:change=move |e: web_sys::Event| {
                        if let Some(input) = e.target() {
                            let input: HtmlInputElement = input.dyn_into().unwrap();
                            let raw_value = input.value();
                            let new_value = raw_value.parse::<f64>()
                                .map(Value::from)
                                .unwrap_or(Value::Null);
                            set_local_value.set(Some(new_value.clone()));
                            on_change.run(new_value);
                        }
                    }
                    step="1"
                />
            }.into_any()
        },
        ComponentType::AmountInput => {
            view! {
                <input
                    type="number"
                    class=format!("tui-cf-input tui-cf-amount {}", base_class)
                    value=display_value
                    disabled=is_disabled
                    on:change=move |e: web_sys::Event| {
                        if let Some(input) = e.target() {
                            let input: HtmlInputElement = input.dyn_into().unwrap();
                            let raw_value = input.value();
                            let new_value = raw_value.parse::<f64>()
                                .map(Value::from)
                                .unwrap_or(Value::Null);
                            set_local_value.set(Some(new_value.clone()));
                            on_change.run(new_value);
                        }
                    }
                    step="0.01"
                    placeholder="0.00"
                />
            }.into_any()
        },
        ComponentType::DatePicker => {
            view! {
                <input
                    type="date"
                    class=format!("tui-cf-input tui-cf-date {}", base_class)
                    value=display_value
                    disabled=is_disabled
                    on:change=move |e: web_sys::Event| {
                        if let Some(input) = e.target() {
                            let input: HtmlInputElement = input.dyn_into().unwrap();
                            let new_value = Value::from(input.value());
                            set_local_value.set(Some(new_value.clone()));
                            on_change.run(new_value);
                        }
                    }
                />
            }.into_any()
        },
        _ => {
            view! {
                <input
                    type="text"
                    class=format!("tui-cf-input {}", base_class)
                    value=display_value
                    disabled=is_disabled
                    on:change=move |e: web_sys::Event| {
                        if let Some(input) = e.target() {
                            let input: HtmlInputElement = input.dyn_into().unwrap();
                            let new_value = Value::from(input.value());
                            set_local_value.set(Some(new_value.clone()));
                            on_change.run(new_value);
                        }
                    }
                />
            }.into_any()
        }
    }
}