use leptos::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use crate::types::*;
use crate::input::FormInput;

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
        "tui-cf-form tui-cf-inline"
    } else {
        "tui-cf-form"
    };
    
    view! {
        <form class=form_class on:submit=handle_submit>
            {move || fields.get().into_iter().map(|field| {
                let field_name = field.name.clone();
                let field_label = field.label.clone();
                let component = field.component.clone();
                let field_name_vis = field.name.clone();
                let field_name_req = field.name.clone();
                let field_name_dis = field.name.clone();
                let field_name_err = field.name.clone();
                let field_name_val = field.name.clone();
                let field_name_for_update = field_name.clone();
                let field_required_base = field.required;
                
                {
                    view! {
                        <Show
                            when={
                                let field_name_vis_c = field_name_vis.clone();
                                move || field_visibility.get().get(&field_name_vis_c).copied().unwrap_or(true)
                            }
                            fallback=|| ()
                        >
                            <div class=if inline { "tui-cf-field" } else { "tui-cf-field tui-cf-field-full" }>
                                <label class="tui-cf-label">
                                    {field_label.clone()}
                                    <Show
                                        when={
                                            let field_name_req_c = field_name_req.clone();
                                            move || field_required.get().get(&field_name_req_c).copied().unwrap_or(field_required_base)
                                        }
                                        fallback=|| ()
                                    >
                                        <span class="tui-cf-required">"*"</span>
                                    </Show>
                                </label>
                                <FormInput
                                    component=component
                                    value={
                                        let field_name_val_c = field_name_val.clone();
                                        move || form_data.get().get(&field_name_val_c).cloned()
                                    }
                                    disabled={
                                        let field_name_dis_c = field_name_dis.clone();
                                        move || field_disabled.get().get(&field_name_dis_c).copied().unwrap_or(false)
                                    }
                                    on_change=Callback::new(move |value| {
                                        update_field(field_name_for_update.clone(), value)
                                    })
                                />
                                <Show
                                    when={
                                        let field_name_err_c = field_name_err.clone();
                                        move || validation_errors.get().get(&field_name_err_c).is_some()
                                    }
                                    fallback=|| ()
                                >
                                    <p class="tui-cf-error">{
                                        let field_name_err_c2 = field_name_err.clone();
                                        move || validation_errors.get().get(&field_name_err_c2).cloned().unwrap_or_default()
                                    }</p>
                                </Show>
                            </div>
                        </Show>
                    }
                }
            }).collect::<Vec<_>>()}
            
            <div class="tui-cf-actions">
                <button
                    type="submit"
                    class="tui-cf-btn tui-cf-btn-primary"
                >
                    "Submit"
                </button>
                <button
                    type="button"
                    class="tui-cf-btn tui-cf-btn-secondary"
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