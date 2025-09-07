use leptos::prelude::*;
use leptos::html::Input;
use leptos::ev::{SubmitEvent, FocusEvent};
use std::collections::HashMap;
use crate::types::*;
use crate::validation::{FieldValidator, FormValidator, ValidationResult};

/// SimpleForm component with validation
#[component]
pub fn SimpleForm(
    /// Form configuration
    config: FormConfig,
    
    /// Callback when form is submitted with valid data
    on_submit: Callback<FormData, ()>,
    
    /// Optional callback when form is cancelled
    #[prop(optional)] on_cancel: Option<Callback<(), ()>>,
    
    /// Whether to show the form
    #[prop(default = true)] show: bool,
    
    /// Optional CSS class
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    // Form state
    let (form_values, set_form_values) = signal(HashMap::<String, String>::new());
    let (field_errors, set_field_errors) = signal(HashMap::<String, String>::new());
    let (touched_fields, set_touched_fields) = signal(HashMap::<String, bool>::new());
    let (is_submitting, set_is_submitting) = signal(false);
    
    // Initialize form values with defaults
    Effect::new({
        let config = config.clone();
        move |_| {
            let mut initial_values = HashMap::new();
            for field in &config.fields {
                if let Some(default) = &field.default_value {
                    initial_values.insert(field.name.clone(), default.clone());
                } else {
                    initial_values.insert(field.name.clone(), String::new());
                }
            }
            set_form_values.set(initial_values);
        }
    });
    
    // Create form validator
    let validator = {
        let mut form_validator = FormValidator::new();
        for field in &config.fields {
            let mut field_validator = FieldValidator::new();
            for rule in &field.validation_rules {
                field_validator = apply_validation_rule(field_validator, rule.clone());
            }
            form_validator.add_field(&field.name, field_validator);
        }
        StoredValue::new(form_validator)
    };
    
    // Validate field
    let validate_field = move |field_name: String, value: String| {
        validator.with_value(|v| {
            let result = v.validate_field(&field_name, &value);
            set_field_errors.update(|errors| {
                if let ValidationResult::Invalid(msg) = result {
                    errors.insert(field_name, msg);
                } else {
                    errors.remove(&field_name);
                }
            });
        });
    };
    
    // Handle field change
    let handle_field_change = move |field_name: String, value: String| {
        set_form_values.update(|values| {
            values.insert(field_name.clone(), value.clone());
        });
        
        if config.validate_on_change {
            validate_field(field_name, value);
        }
    };
    
    // Handle field blur
    let handle_field_blur = move |field_name: String| {
        set_touched_fields.update(|touched| {
            touched.insert(field_name.clone(), true);
        });
        
        if config.validate_on_blur {
            let value = form_values.get().get(&field_name).cloned().unwrap_or_default();
            validate_field(field_name, value);
        }
    };
    
    // Handle form submission
    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        // Validate all fields
        validator.with_value(|v| {
            let values = form_values.get();
            let validation_results = v.validate_all(&values);
            
            let mut errors = HashMap::new();
            let mut is_valid = true;
            
            for (field_name, result) in validation_results {
                if let ValidationResult::Invalid(msg) = result {
                    errors.insert(field_name, msg);
                    is_valid = false;
                }
            }
            
            set_field_errors.set(errors);
            
            if is_valid {
                set_is_submitting.set(true);
                on_submit.run(FormData { values });
                set_is_submitting.set(false);
            } else {
                // Mark all fields as touched to show errors
                let mut all_touched = HashMap::new();
                for field in &config.fields {
                    all_touched.insert(field.name.clone(), true);
                }
                set_touched_fields.set(all_touched);
            }
        });
    };
    
    let handle_cancel = move |_| {
        if let Some(on_cancel) = on_cancel {
            on_cancel.run(());
        }
    };
    
    let custom_class = class.unwrap_or_default();
    
    view! {
        <Show when=move || show>
            <div class={format!("sf-container {}", custom_class)}>
                <form on:submit=handle_submit class="sf-form">
                    {config.title.as_ref().map(|title| {
                        view! {
                            <h2 class="sf-title">{title.clone()}</h2>
                        }
                    })}
                    
                    {config.description.as_ref().map(|desc| {
                        view! {
                            <p class="sf-description">{desc.clone()}</p>
                        }
                    })}
                    
                    <div class="sf-fields">
                        <For
                            each=move || config.fields.clone()
                            key=|field| field.name.clone()
                            children=move |field| {
                                let field_name = field.name.clone();
                                let field_name_change = field_name.clone();
                                let field_name_blur = field_name.clone();
                                
                                view! {
                                    <div class="sf-field">
                                        <label class="sf-label" for={field.name.clone()}>
                                            {field.label.clone()}
                                            {if field.required && config.show_required_indicator {
                                                view! { <span class="sf-required">"*"</span> }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }}
                                        </label>
                                        
                                        {render_field(
                                            field.clone(),
                                            move || form_values.get().get(&field_name).cloned().unwrap_or_default(),
                                            move |value| handle_field_change(field_name_change.clone(), value),
                                            move || handle_field_blur(field_name_blur.clone()),
                                        )}
                                        
                                        {field.help_text.as_ref().map(|help| {
                                            view! {
                                                <p class="sf-help">{help.clone()}</p>
                                            }
                                        })}
                                        
                                        <Show when=move || {
                                            let is_touched = touched_fields.get().get(&field.name).copied().unwrap_or(false);
                                            let has_error = field_errors.get().contains_key(&field.name);
                                            is_touched && has_error
                                        }>
                                            {move || {
                                                let error = field_errors.get().get(&field.name).cloned().unwrap_or_default();
                                                view! {
                                                    <p class="sf-error">{error}</p>
                                                }
                                            }}
                                        </Show>
                                    </div>
                                }
                            }
                        />
                    </div>
                    
                    <div class="sf-actions">
                        <Show when=move || config.show_cancel>
                            <button
                                type="button"
                                class="sf-btn sf-btn-cancel"
                                on:click=handle_cancel
                            >
                                {config.cancel_text.clone()}
                            </button>
                        </Show>
                        
                        <button
                            type="submit"
                            class="sf-btn sf-btn-submit"
                            disabled=move || is_submitting.get()
                        >
                            {if is_submitting.get() {
                                "Submitting..."
                            } else {
                                &config.submit_text
                            }}
                        </button>
                    </div>
                </form>
            </div>
        </Show>
    }
}

/// Render a form field based on its type
fn render_field(
    field: FieldConfig,
    value: impl Fn() -> String + 'static + Clone,
    on_change: impl Fn(String) + 'static + Clone,
    on_blur: impl Fn() + 'static + Clone,
) -> impl IntoView {
    match field.field_type {
        FieldType::Text | FieldType::Email | FieldType::Password | FieldType::Phone | FieldType::Url => {
            let input_type = match field.field_type {
                FieldType::Email => "email",
                FieldType::Password => "password",
                FieldType::Phone => "tel",
                FieldType::Url => "url",
                _ => "text",
            };
            
            view! {
                <input
                    type=input_type
                    id={field.name.clone()}
                    name={field.name.clone()}
                    class="sf-input"
                    placeholder={field.placeholder.unwrap_or_default()}
                    value=value
                    on:input=move |ev| on_change(event_target_value(&ev))
                    on:blur=move |_| on_blur()
                    disabled={field.disabled}
                    readonly={field.readonly}
                    autofocus={field.autofocus}
                />
            }.into_any()
        },
        
        FieldType::Number | FieldType::Integer | FieldType::Currency => {
            let step = field.step.unwrap_or_else(|| {
                match field.field_type {
                    FieldType::Integer => "1".to_string(),
                    FieldType::Currency => "0.01".to_string(),
                    _ => "any".to_string(),
                }
            });
            
            view! {
                <input
                    type="number"
                    id={field.name.clone()}
                    name={field.name.clone()}
                    class="sf-input"
                    placeholder={field.placeholder.unwrap_or_default()}
                    value=value
                    on:input=move |ev| on_change(event_target_value(&ev))
                    on:blur=move |_| on_blur()
                    min={field.min}
                    max={field.max}
                    step={step}
                    disabled={field.disabled}
                    readonly={field.readonly}
                    autofocus={field.autofocus}
                />
            }.into_any()
        },
        
        FieldType::Date => {
            view! {
                <input
                    type="date"
                    id={field.name.clone()}
                    name={field.name.clone()}
                    class="sf-input"
                    value=value
                    on:input=move |ev| on_change(event_target_value(&ev))
                    on:blur=move |_| on_blur()
                    min={field.min}
                    max={field.max}
                    disabled={field.disabled}
                    readonly={field.readonly}
                    autofocus={field.autofocus}
                />
            }.into_any()
        },
        
        FieldType::DateTime => {
            view! {
                <input
                    type="datetime-local"
                    id={field.name.clone()}
                    name={field.name.clone()}
                    class="sf-input"
                    value=value
                    on:input=move |ev| on_change(event_target_value(&ev))
                    on:blur=move |_| on_blur()
                    min={field.min}
                    max={field.max}
                    disabled={field.disabled}
                    readonly={field.readonly}
                    autofocus={field.autofocus}
                />
            }.into_any()
        },
        
        FieldType::Time => {
            view! {
                <input
                    type="time"
                    id={field.name.clone()}
                    name={field.name.clone()}
                    class="sf-input"
                    value=value
                    on:input=move |ev| on_change(event_target_value(&ev))
                    on:blur=move |_| on_blur()
                    min={field.min}
                    max={field.max}
                    disabled={field.disabled}
                    readonly={field.readonly}
                    autofocus={field.autofocus}
                />
            }.into_any()
        },
        
        FieldType::Boolean => {
            let is_checked = move || value() == "true";
            view! {
                <div class="sf-checkbox-wrapper">
                    <input
                        type="checkbox"
                        id={field.name.clone()}
                        name={field.name.clone()}
                        class="sf-checkbox"
                        checked=is_checked
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            on_change(checked.to_string())
                        }
                        disabled={field.disabled}
                        autofocus={field.autofocus}
                    />
                    <label for={field.name.clone()} class="sf-checkbox-label">
                        {field.placeholder.unwrap_or_default()}
                    </label>
                </div>
            }.into_any()
        },
        
        FieldType::Select => {
            let options = field.options.unwrap_or_default();
            view! {
                <select
                    id={field.name.clone()}
                    name={field.name.clone()}
                    class="sf-select"
                    on:change=move |ev| on_change(event_target_value(&ev))
                    on:blur=move |_| on_blur()
                    disabled={field.disabled}
                    autofocus={field.autofocus}
                >
                    <For
                        each=move || options.clone()
                        key=|opt| opt.value.clone()
                        children=move |option| {
                            let is_selected = {
                                let value = value.clone();
                                move || value() == option.value
                            };
                            view! {
                                <option
                                    value={option.value.clone()}
                                    selected=is_selected
                                    disabled={option.disabled}
                                >
                                    {option.label}
                                </option>
                            }
                        }
                    />
                </select>
            }.into_any()
        },
        
        FieldType::Textarea => {
            let rows = field.rows.unwrap_or(3);
            view! {
                <textarea
                    id={field.name.clone()}
                    name={field.name.clone()}
                    class="sf-textarea"
                    placeholder={field.placeholder.unwrap_or_default()}
                    value=value
                    on:input=move |ev| on_change(event_target_value(&ev))
                    on:blur=move |_| on_blur()
                    rows={rows}
                    disabled={field.disabled}
                    readonly={field.readonly}
                    autofocus={field.autofocus}
                />
            }.into_any()
        },
        
        _ => {
            // Fallback for unimplemented field types
            view! {
                <input
                    type="text"
                    id={field.name.clone()}
                    name={field.name.clone()}
                    class="sf-input"
                    placeholder={field.placeholder.unwrap_or_default()}
                    value=value
                    on:input=move |ev| on_change(event_target_value(&ev))
                    on:blur=move |_| on_blur()
                    disabled={field.disabled}
                    readonly={field.readonly}
                    autofocus={field.autofocus}
                />
            }.into_any()
        }
    }
}

/// Apply validation rule to field validator
fn apply_validation_rule(validator: FieldValidator, rule: crate::validation::ValidationRule) -> FieldValidator {
    use crate::validation::ValidationRule;
    
    match rule {
        ValidationRule::Required => validator.required(),
        ValidationRule::MinLength { min } => validator.min_length(min),
        ValidationRule::MaxLength { max } => validator.max_length(max),
        ValidationRule::Pattern { regex, message } => validator.pattern(&regex, &message),
        ValidationRule::Email => validator.email(),
        ValidationRule::Url => validator.url(),
        ValidationRule::Number { min, max } => validator.number(min, max),
        ValidationRule::Integer { min, max } => validator.integer(min, max),
        ValidationRule::Date { min, max } => validator.date(min, max),
        ValidationRule::PhoneNumber => validator.phone_number(),
        ValidationRule::TaxId => validator.tax_id(),
        ValidationRule::Currency { min, max } => validator.currency(min, max),
        ValidationRule::Custom { .. } => validator, // Custom validators need special handling
    }
}