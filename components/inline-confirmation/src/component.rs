use leptos::prelude::*;
use leptos::ev::{FocusEvent, KeyboardEvent};
use crate::types::*;
use crate::validators::*;
use gloo_timers::future::TimeoutFuture;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;

/// Inline confirmation input with validation
#[component]
pub fn InlineConfirmation<V, C>(
    /// Current value
    value: ReadSignal<String>,
    /// Set value function
    set_value: WriteSignal<String>,
    /// Validation rules
    #[prop(optional)] rules: Option<Vec<ValidationRule>>,
    /// Input type
    #[prop(default = InputType::Text)] input_type: InputType,
    /// Configuration
    #[prop(optional)] config: Option<InlineConfirmConfig>,
    /// Validation callback (for async validation)
    #[prop(optional)] on_validate: Option<V>,
    /// Confirmation callback
    #[prop(optional)] on_confirm: Option<C>,
    /// Placeholder text
    #[prop(default = String::new())] placeholder: String,
    /// Label text
    #[prop(optional)] label: Option<String>,
    /// Help text
    #[prop(optional)] help_text: Option<String>,
    /// CSS class
    #[prop(optional)] class: Option<String>,
    /// Disabled state
    #[prop(default = false)] disabled: bool,
) -> impl IntoView
where
    V: Fn(String) -> ValidationState + Clone + 'static,
    C: Fn(String) -> bool + Clone + 'static,
{
    let config = config.unwrap_or_default();
    let rules = rules.unwrap_or_default();
    
    // State
    let (validation_state, set_validation_state) = signal(ValidationState::Idle);
    let (confirmation_state, set_confirmation_state) = signal(ConfirmationState::Idle);
    let (is_focused, set_is_focused) = signal(false);
    let (has_been_touched, set_has_been_touched) = signal(false);
    let (show_confirmation, set_show_confirmation) = signal(false);
    let (animation_class, set_animation_class) = signal(String::new());
    
    // Perform validation
    let validate = {
        let rules = rules.clone();
        let on_validate = on_validate.clone();
        let config = config.clone();
        
        move |value: String| {
            if value.is_empty() && !rules.iter().any(|r| matches!(r, ValidationRule::Required(_))) {
                set_validation_state.set(ValidationState::Idle);
                return;
            }
            
            // First check local rules
            let local_result = validate_input(&value, &rules);
            
            // If local validation fails, use that result
            if matches!(local_result, ValidationState::Invalid(_)) {
                set_validation_state.set(local_result);
                
                // Shake animation on error
                if config.shake_on_error {
                    set_animation_class.set("ic-shake".to_string());
                    spawn_local(async move {
                        TimeoutFuture::new(500).await;
                        set_animation_class.set(String::new());
                    });
                }
                
                // Auto-focus on error
                if config.auto_focus_on_error {
                    // Focus functionality would be handled here if needed
                }
                
                return;
            }
            
            // If there's an async validator, use it
            if let Some(ref validator) = on_validate {
                set_validation_state.set(ValidationState::Validating);
                let result = validator(value);
                set_validation_state.set(result);
            } else {
                set_validation_state.set(local_result);
            }
        }
    };
    
    // Handle input change
    let handle_input = {
        let config = config.clone();
        let validate = validate.clone();
        
        move |ev: web_sys::Event| {
            let target = ev.target().unwrap();
            let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
            let value = input.value();
            set_value.set(value.clone());
            set_has_been_touched.set(true);
            
            if config.validate_on_change && has_been_touched.get() {
                if config.validation_debounce > 0 {
                    let validate = validate.clone();
                    spawn_local(async move {
                        TimeoutFuture::new(config.validation_debounce).await;
                        validate(value);
                    });
                } else {
                    validate(value);
                }
            }
        }
    };
    
    // Handle blur
    let handle_blur = {
        let config = config.clone();
        let validate = validate.clone();
        
        move |_: FocusEvent| {
            set_is_focused.set(false);
            set_has_been_touched.set(true);
            
            if config.validate_on_blur {
                validate(value.get());
            }
        }
    };
    
    // Handle focus
    let handle_focus = move |_: FocusEvent| {
        set_is_focused.set(true);
    };
    
    // Handle Enter key for confirmation
    let handle_keydown = {
        let config = config.clone();
        let on_confirm = on_confirm.clone();
        
        move |ev: KeyboardEvent| {
            if ev.key() == "Enter" && !disabled {
                if matches!(validation_state.get(), ValidationState::Valid(_)) {
                    if config.require_confirmation {
                        if confirmation_state.get() == ConfirmationState::Pending {
                            // Confirm action
                            if let Some(ref confirm_fn) = on_confirm {
                                if confirm_fn(value.get()) {
                                    set_confirmation_state.set(ConfirmationState::Confirmed);
                                    set_show_confirmation.set(false);
                                    
                                    // Reset after timeout
                                    spawn_local(async move {
                                        TimeoutFuture::new(2000).await;
                                        set_confirmation_state.set(ConfirmationState::Idle);
                                    });
                                }
                            }
                        } else {
                            // Show confirmation
                            set_confirmation_state.set(ConfirmationState::Pending);
                            set_show_confirmation.set(true);
                            
                            // Auto-cancel after timeout
                            if config.confirmation_timeout > 0 {
                                spawn_local(async move {
                                    TimeoutFuture::new(config.confirmation_timeout).await;
                                    if confirmation_state.get() == ConfirmationState::Pending {
                                        set_confirmation_state.set(ConfirmationState::Cancelled);
                                        set_show_confirmation.set(false);
                                    }
                                });
                            }
                        }
                    } else if let Some(ref confirm_fn) = on_confirm {
                        confirm_fn(value.get());
                    }
                }
            } else if ev.key() == "Escape" {
                set_confirmation_state.set(ConfirmationState::Cancelled);
                set_show_confirmation.set(false);
            }
        }
    };
    
    // Format value for specific input types
    let display_value = move || {
        let val = value.get();
        match input_type {
            InputType::Currency => {
                if let Ok(amount) = val.parse::<f64>() {
                    format_philippine_currency(amount)
                } else {
                    val
                }
            }
            InputType::Percentage => {
                if let Ok(percent) = val.parse::<f64>() {
                    format_percentage(percent)
                } else {
                    val
                }
            }
            _ => val,
        }
    };
    
    // Get validation icon - using derived signal instead of Memo to avoid PartialEq requirement
    let validation_icon = move || {
        match validation_state.get() {
            ValidationState::Validating if config.show_loading_spinner => {
                Some(view! { <span class="ic-icon ic-spinner">{"⟳"}</span> })
            }
            ValidationState::Valid(_) if config.show_success_icon => {
                Some(view! { <span class="ic-icon ic-success">{"✓"}</span> })
            }
            ValidationState::Invalid(_) if config.show_error_icon => {
                Some(view! { <span class="ic-icon ic-error">{"✗"}</span> })
            }
            ValidationState::Warning(_) => {
                Some(view! { <span class="ic-icon ic-warning">{"⚠"}</span> })
            }
            _ => None,
        }
    };
    
    // Get validation message - using derived signal
    let validation_message = move || {
        match validation_state.get() {
            ValidationState::Valid(Some(msg)) => Some((msg, "ic-message-success")),
            ValidationState::Invalid(msg) => Some((msg, "ic-message-error")),
            ValidationState::Warning(msg) => Some((msg, "ic-message-warning")),
            _ => None,
        }
    };
    
    // Get input state class - using derived signal
    let input_class = move || {
        let mut classes = vec!["ic-input"];
        
        match validation_state.get() {
            ValidationState::Valid(_) => classes.push("ic-input-valid"),
            ValidationState::Invalid(_) => classes.push("ic-input-invalid"),
            ValidationState::Warning(_) => classes.push("ic-input-warning"),
            ValidationState::Validating => classes.push("ic-input-validating"),
            _ => {}
        }
        
        if is_focused.get() {
            classes.push("ic-input-focused");
        }
        
        if disabled {
            classes.push("ic-input-disabled");
        }
        
        if !animation_class.get().is_empty() {
            classes.push("ic-shake");
        }
        
        classes.join(" ")
    };
    
    view! {
        <div class=format!("ic-container {}", class.unwrap_or_default())>
            {label.as_ref().map(|label_text| view! {
                <label class="ic-label">
                    {label_text.clone()}
                    {
                        let is_required = rules.iter().any(|r| matches!(r, ValidationRule::Required(_)));
                        if is_required {
                            Some(view! { <span class="ic-required">{"*"}</span> })
                        } else {
                            None
                        }
                    }
                </label>
            })}
            
            <div class="ic-input-wrapper">
                <input
                    type=input_type.to_string()
                    class=move || input_class()
                    prop:value=move || display_value()
                    placeholder=placeholder.clone()
                    disabled=disabled
                    on:input=handle_input
                    on:blur=handle_blur
                    on:focus=handle_focus
                    on:keydown=handle_keydown
                />
                
                <Show when=move || validation_icon().is_some()>
                    <div class="ic-icon-wrapper">
                        {move || validation_icon()}
                    </div>
                </Show>
            </div>
            
            {help_text.as_ref().filter(|_| validation_message().is_none()).map(|text| view! {
                <div class="ic-help-text">
                    {text.clone()}
                </div>
            })}
            
            <Show when=move || validation_message().is_some()>
                {move || {
                    validation_message().map(|(msg, class)| {
                        view! {
                            <div class=format!("ic-message {}", class)>
                                {msg}
                            </div>
                        }
                    })
                }}
            </Show>
            
            <Show when=move || show_confirmation.get()>
                <div class="ic-confirmation">
                    <span class="ic-confirmation-text">
                        "Press Enter to confirm or Esc to cancel"
                    </span>
                    <div class="ic-confirmation-timer" 
                        style=move || {
                            if config.confirmation_timeout > 0 {
                                format!("animation-duration: {}ms", config.confirmation_timeout)
                            } else {
                                String::new()
                            }
                        }
                    ></div>
                </div>
            </Show>
        </div>
    }
}

/// Password input with strength indicator
#[component]
pub fn PasswordInput(
    /// Current value
    value: ReadSignal<String>,
    /// Set value function
    set_value: WriteSignal<String>,
    /// Show strength indicator
    #[prop(default = true)] show_strength: bool,
    /// CSS class
    #[prop(optional)] class: Option<String>,
) -> impl IntoView
{
    let (show_password, set_show_password) = signal(false);
    let (strength_state, set_strength_state) = signal(ValidationState::Idle);
    
    // Validate password on change
    Effect::new(move |_| {
        let password = value.get();
        if !password.is_empty() {
            let state = validate_password_strength(&password);
            set_strength_state.set(state);
        } else {
            set_strength_state.set(ValidationState::Idle);
        }
    });
    
    // Get strength class and text
    let strength_info = Memo::new(move |_| {
        match strength_state.get() {
            ValidationState::Valid(_) => ("ic-strength-strong", "Strong", 100),
            ValidationState::Warning(_) => ("ic-strength-medium", "Medium", 60),
            ValidationState::Invalid(_) => ("ic-strength-weak", "Weak", 30),
            _ => ("", "", 0),
        }
    });
    
    view! {
        <div class=format!("ic-password-container {}", class.unwrap_or_default())>
            <div class="ic-password-input-wrapper">
                <input
                    type=if show_password.get() { "text" } else { "password" }
                    class="ic-input"
                    prop:value=move || value.get()
                    placeholder="Enter password"
                    on:input=move |ev| {
                        let target = ev.target().unwrap();
                        let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        set_value.set(input.value());
                    }
                />
                
                <button
                    class="ic-password-toggle"
                    on:click=move |_| set_show_password.update(|s| *s = !*s)
                    type="button"
                >
                    {move || if show_password.get() { "👁" } else { "👁‍🗨" }}
                </button>
            </div>
            
            <Show when=move || show_strength && !value.get().is_empty()>
                <div class="ic-strength-indicator">
                    <div class="ic-strength-bar">
                        <div 
                            class=move || format!("ic-strength-fill {}", strength_info.get().0)
                            style=move || format!("width: {}%", strength_info.get().2)
                        ></div>
                    </div>
                    <span class="ic-strength-text">{move || strength_info.get().1}</span>
                </div>
            </Show>
        </div>
    }
}