use leptos::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

// Reuse ComponentType from smart_command_input
use crate::components::smart_command_input::ComponentType;

#[derive(Debug, Clone)]
pub struct FormField {
    pub name: String,
    pub label: String,
    pub component: ComponentType,
    pub required: bool,
    pub condition: Option<FieldCondition>,
    pub default_value: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct FieldCondition {
    pub when: String,        // Field name to watch
    pub operator: ConditionOperator,
    pub value: Value,
    pub action: ConditionAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    Contains,
    GreaterThan,
    LessThan,
    In,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionAction {
    Show,
    Hide,
    Require,
    Disable,
}

#[component]
pub fn ConditionalForm(
    fields: Signal<Vec<FormField>>,
    on_submit: Callback<HashMap<String, Value>, ()>,
    #[prop(default = false)] inline: bool,
) -> impl IntoView {
    let (form_data, set_form_data) = signal(HashMap::<String, Value>::new());
    let (field_visibility, set_field_visibility) = signal(HashMap::<String, bool>::new());
    let (field_required, set_field_required) = signal(HashMap::<String, bool>::new());
    let (field_disabled, set_field_disabled) = signal(HashMap::<String, bool>::new());
    let (validation_errors, set_validation_errors) = signal(HashMap::<String, String>::new());
    
    // Initialize default values
    Effect::new(move |_| {
        let mut initial_data = HashMap::new();
        let mut initial_visibility = HashMap::new();
        let mut initial_required = HashMap::new();
        
        for field in fields.get().iter() {
            if let Some(default) = &field.default_value {
                initial_data.insert(field.name.clone(), default.clone());
            }
            initial_visibility.insert(field.name.clone(), true);
            initial_required.insert(field.name.clone(), field.required);
        }
        
        set_form_data.set(initial_data);
        set_field_visibility.set(initial_visibility);
        set_field_required.set(initial_required);
    });
    
    // Evaluate conditions
    let evaluate_conditions = move || {
        let current_data = form_data.get();
        let fields_list = fields.get();
        let mut visibility = field_visibility.get();
        let mut required = field_required.get();
        let mut disabled = field_disabled.get();
        
        for field in fields_list.iter() {
            if let Some(condition) = &field.condition {
                let condition_met = evaluate_condition(&condition, &current_data);
                
                match condition.action {
                    ConditionAction::Show => {
                        visibility.insert(field.name.clone(), condition_met);
                    },
                    ConditionAction::Hide => {
                        visibility.insert(field.name.clone(), !condition_met);
                    },
                    ConditionAction::Require => {
                        required.insert(field.name.clone(), condition_met);
                    },
                    ConditionAction::Disable => {
                        disabled.insert(field.name.clone(), condition_met);
                    },
                }
            }
        }
        
        set_field_visibility.set(visibility);
        set_field_required.set(required);
        set_field_disabled.set(disabled);
    };
    
    // Watch for data changes and re-evaluate conditions
    Effect::new(move |_| {
        form_data.get(); // Trigger on form data changes
        evaluate_conditions();
    });
    
    let update_field = move |field_name: String, value: Value| {
        let mut data = form_data.get();
        data.insert(field_name.clone(), value);
        set_form_data.set(data);
        
        // Clear validation error for this field
        let mut errors = validation_errors.get();
        errors.remove(&field_name);
        set_validation_errors.set(errors);
    };
    
    let validate_form = move || -> bool {
        let mut errors = HashMap::new();
        let data = form_data.get();
        let visibility = field_visibility.get();
        let required = field_required.get();
        
        for field in fields.get().iter() {
            // Only validate visible fields
            if !visibility.get(&field.name).unwrap_or(&true) {
                continue;
            }
            
            // Check required fields
            if *required.get(&field.name).unwrap_or(&field.required) {
                if !data.contains_key(&field.name) || 
                   data.get(&field.name).map_or(true, |v| v.is_null() || 
                   (v.is_string() && v.as_str().unwrap_or("").is_empty())) {
                    errors.insert(field.name.clone(), format!("{} is required", field.label));
                }
            }
        }
        
        set_validation_errors.set(errors.clone());
        errors.is_empty()
    };
    
    let handle_submit = move |e: leptos::ev::SubmitEvent| {
        e.prevent_default();
        
        if validate_form() {
            on_submit.run(form_data.get());
        }
    };
    
    let form_class = if inline {
        "conditional-form inline-form flex flex-wrap gap-4"
    } else {
        "conditional-form space-y-4"
    };
    
    view! {
        <form class=form_class on:submit=handle_submit>
            {move || fields.get().iter().map(|field| {
                let field_name = field.name.clone();
                let field_label = field.label.clone();
                let component = field.component.clone();
                let is_visible = field_visibility.get().get(&field.name).copied().unwrap_or(true);
                let is_required = field_required.get().get(&field.name).copied().unwrap_or(field.required);
                let is_disabled = field_disabled.get().get(&field.name).copied().unwrap_or(false);
                let error = validation_errors.get().get(&field.name).cloned();
                let current_value = form_data.get().get(&field.name).cloned();
                
                if !is_visible {
                    view! { <></> }.into_any()
                } else {
                    view! {
                        <div class=if inline { "form-field" } else { "form-field w-full" }>
                            <label class="block text-sm font-medium text-gray-700 mb-1">
                                {field_label.clone()}
                                {if is_required {
                                    view! { <span class="text-red-500 ml-1">"*"</span> }.into_any()
                                } else {
                                    view! { <></> }.into_any()
                                }}
                            </label>
                            <FormInput
                                component=component
                                value=current_value
                                disabled=is_disabled
                                on_change=Box::new(move |value| {
                                    update_field(field_name.clone(), value)
                                })
                            />
                            {if let Some(err) = error {
                                view! {
                                    <p class="text-red-500 text-xs mt-1">{err}</p>
                                }.into_any()
                            } else {
                                view! { <></> }.into_any()
                            }}
                        </div>
                    }.into_any()
                }
            }).collect::<Vec<_>>()}
            
            <div class="form-actions flex gap-2 mt-4">
                <button
                    type="submit"
                    class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
                >
                    "Submit"
                </button>
                <button
                    type="button"
                    class="px-4 py-2 bg-gray-300 text-gray-700 rounded hover:bg-gray-400 transition-colors"
                    on:click=move |_| {
                        set_form_data.set(HashMap::new());
                        set_validation_errors.set(HashMap::new());
                    }
                >
                    "Clear"
                </button>
            </div>
        </form>
    }
}

#[component]
fn FormInput(
    component: ComponentType,
    value: Option<Value>,
    disabled: bool,
    on_change: Box<dyn Fn(Value)>,
) -> impl IntoView {
    let (local_value, set_local_value) = signal(value.clone());
    let component_for_change = component.clone();
    let component_for_display = component.clone();
    let component_for_match = component.clone();
    
    let handle_change = move |e: web_sys::Event| {
        if let Some(input) = e.target() {
            let input: HtmlInputElement = input.dyn_into().unwrap();
            let raw_value = input.value();
            
            let new_value = match component_for_change.clone() {
                ComponentType::NumberInput | ComponentType::AmountInput => {
                    raw_value.parse::<f64>()
                        .map(Value::from)
                        .unwrap_or(Value::Null)
                },
                ComponentType::Boolean => {
                    Value::from(input.checked())
                },
                _ => Value::from(raw_value)
            };
            
            set_local_value.set(Some(new_value.clone()));
            on_change(new_value);
        }
    };
    
    let display_value = move || {
        let comp = component_for_display.clone();
        local_value.get()
            .and_then(|v| match comp {
                ComponentType::AmountInput => v.as_f64().map(|f| format!("{:.2}", f)),
                ComponentType::NumberInput => v.as_f64().map(|f| f.to_string()),
                ComponentType::Boolean => v.as_bool().map(|b| b.to_string()),
                _ => v.as_str().map(|s| s.to_string())
            })
            .unwrap_or_default()
    };
    
    let base_class = if disabled {
        "disabled:bg-gray-100 disabled:cursor-not-allowed"
    } else {
        ""
    };
    
    match component_for_match {
        ComponentType::Boolean => {
            let checked = local_value.get()
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            
            view! {
                <input
                    type="checkbox"
                    class=format!("h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded {}", base_class)
                    checked=checked
                    disabled=disabled
                    on:change=handle_change
                />
            }.into_any()
        },
        ComponentType::Select(options) => {
            view! {
                <select
                    class=format!("block w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500 {}", base_class)
                    disabled=disabled
                    on:change=handle_change
                >
                    <option value="">"-"</option>
                    {options.iter().map(|opt| {
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
                    class=format!("block w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500 {}", base_class)
                    value=display_value
                    disabled=disabled
                    on:change=handle_change
                    step="1"
                />
            }.into_any()
        },
        ComponentType::AmountInput => {
            view! {
                <input
                    type="number"
                    class=format!("block w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500 {}", base_class)
                    value=display_value
                    disabled=disabled
                    on:change=handle_change
                    step="0.01"
                    placeholder="0.00"
                />
            }.into_any()
        },
        ComponentType::DatePicker => {
            view! {
                <input
                    type="date"
                    class=format!("block w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500 {}", base_class)
                    value=display_value
                    disabled=disabled
                    on:change=handle_change
                />
            }.into_any()
        },
        _ => {
            view! {
                <input
                    type="text"
                    class=format!("block w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500 {}", base_class)
                    value=display_value
                    disabled=disabled
                    on:change=handle_change
                />
            }.into_any()
        }
    }
}

fn evaluate_condition(condition: &FieldCondition, form_data: &HashMap<String, Value>) -> bool {
    let field_value = form_data.get(&condition.when);
    
    match (&condition.operator, field_value) {
        (ConditionOperator::Equals, Some(val)) => val == &condition.value,
        (ConditionOperator::NotEquals, Some(val)) => val != &condition.value,
        (ConditionOperator::Contains, Some(val)) => {
            if let (Some(str_val), Some(search)) = (val.as_str(), condition.value.as_str()) {
                str_val.contains(search)
            } else {
                false
            }
        },
        (ConditionOperator::GreaterThan, Some(val)) => {
            if let (Some(num_val), Some(compare)) = (val.as_f64(), condition.value.as_f64()) {
                num_val > compare
            } else {
                false
            }
        },
        (ConditionOperator::LessThan, Some(val)) => {
            if let (Some(num_val), Some(compare)) = (val.as_f64(), condition.value.as_f64()) {
                num_val < compare
            } else {
                false
            }
        },
        (ConditionOperator::In, Some(val)) => {
            if let Some(arr) = condition.value.as_array() {
                arr.contains(val)
            } else {
                false
            }
        },
        _ => false,
    }
}

// Example usage for payment method form
pub fn create_payment_form_fields() -> Vec<FormField> {
    vec![
        FormField {
            name: "payment_method".into(),
            label: "Payment Method".into(),
            component: ComponentType::Select(vec![
                "cash".into(),
                "gcash".into(),
                "maya".into(),
                "bank_transfer".into(),
            ]),
            required: true,
            condition: None,
            default_value: Some(serde_json::json!("cash")),
        },
        FormField {
            name: "reference_number".into(),
            label: "Reference Number".into(),
            component: ComponentType::TextInput,
            required: false,
            condition: Some(FieldCondition {
                when: "payment_method".into(),
                operator: ConditionOperator::In,
                value: serde_json::json!(["gcash", "maya", "bank_transfer"]),
                action: ConditionAction::Show,
            }),
            default_value: None,
        },
        FormField {
            name: "proof".into(),
            label: "Upload Proof".into(),
            component: ComponentType::FileUpload,
            required: false,
            condition: Some(FieldCondition {
                when: "payment_method".into(),
                operator: ConditionOperator::NotEquals,
                value: serde_json::json!("cash"),
                action: ConditionAction::Require,
            }),
            default_value: None,
        },
    ]
}