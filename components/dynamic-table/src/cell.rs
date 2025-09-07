use leptos::prelude::*;
use serde_json::Value;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use taxtalk_ui_core::ComponentType;

#[component]
pub fn TableCell(
    component: ComponentType,
    value: Option<Value>,
    readonly: bool,
    on_change: Callback<Value, ()>,
) -> impl IntoView {
    let (local_value, set_local_value) = signal(value.clone());
    let component_for_display = component.clone();
    
    let display_value = move || {
        local_value.get()
            .and_then(|v| match component_for_display {
                ComponentType::AmountInput => v.as_f64().map(|f| format!("{:.2}", f)),
                ComponentType::NumberInput => v.as_f64().map(|f| f.to_string()),
                ComponentType::Boolean => v.as_bool().map(|b| if b { "✓" } else { "" }.to_string()),
                _ => v.as_str().map(|s| s.to_string())
            })
            .unwrap_or_default()
    };
    
    if readonly {
        view! {
            <span class="tui-dt-readonly">
                {display_value}
            </span>
        }.into_any()
    } else {
        match component {
            ComponentType::Boolean => {
                let checked = local_value.get()
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                
                view! {
                    <input
                        type="checkbox"
                        class="tui-dt-checkbox"
                        checked=checked
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
            ComponentType::Select(ref options) => {
                let opts = options.clone();
                view! {
                    <select
                        class="tui-dt-select"
                        on:change=move |e: web_sys::Event| {
                            if let Some(select) = e.target() {
                                let select: HtmlSelectElement = select.dyn_into().unwrap();
                                let new_value = Value::from(select.value());
                                set_local_value.set(Some(new_value.clone()));
                                on_change.run(new_value);
                            }
                        }
                    >
                        <option value="">"-"</option>
                        {opts.iter().map(|opt| {
                            view! {
                                <option value=opt.clone()>{opt.clone()}</option>
                            }
                        }).collect::<Vec<_>>()}
                    </select>
                }.into_any()
            },
            ComponentType::NumberInput => {
                view! {
                    <input
                        type="number"
                        class="tui-dt-input tui-dt-number"
                        value=display_value
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
                        class="tui-dt-input tui-dt-amount"
                        value=display_value
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
            _ => {
                view! {
                    <input
                        type="text"
                        class="tui-dt-input"
                        value=display_value
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
}