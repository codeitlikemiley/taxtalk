# Form Handling Example

## Overview

This example demonstrates comprehensive form handling patterns in Leptos, including controlled inputs, validation, submission handling, and form state management.

## Complete Code

```rust
use leptos::*;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

#[derive(Clone, Debug, Validate, Serialize, Deserialize)]
pub struct ContactForm {
    #[validate(length(min = 2, message = "Name must be at least 2 characters"))]
    pub name: String,

    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,

    #[validate(length(min = 10, message = "Message must be at least 10 characters"))]
    pub message: String,

    #[validate(range(min = 1, max = 5, message = "Rating must be between 1 and 5"))]
    pub rating: Option<i32>,

    pub newsletter: bool,
}

impl Default for ContactForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            message: String::new(),
            rating: None,
            newsletter: false,
        }
    }
}

#[component]
fn ContactFormComponent(cx: Scope) -> impl IntoView {
    // Form state
    let (form_data, set_form_data) = create_signal(cx, ContactForm::default());
    let (errors, set_errors) = create_signal(cx, ValidationErrors::new());
    let (is_submitting, set_is_submitting) = create_signal(cx, false);
    let (submit_message, set_submit_message) = create_signal(cx, String::new());

    // Form field updaters
    let update_name = move |ev| {
        let value = event_target_value(&ev);
        set_form_data.update(|form| form.name = value);
        set_errors.update(|errors| {
            errors.errors_mut().remove("name");
        });
    };

    let update_email = move |ev| {
        let value = event_target_value(&ev);
        set_form_data.update(|form| form.email = value);
        set_errors.update(|errors| {
            errors.errors_mut().remove("email");
        });
    };

    let update_message = move |ev| {
        let value = event_target_value(&ev);
        set_form_data.update(|form| form.message = value);
        set_errors.update(|errors| {
            errors.errors_mut().remove("message");
        });
    };

    let update_rating = move |ev| {
        let value = event_target_value(&ev);
        let rating = value.parse::<i32>().ok();
        set_form_data.update(|form| form.rating = rating);
        set_errors.update(|errors| {
            errors.errors_mut().remove("rating");
        });
    };

    let update_newsletter = move |ev| {
        let checked = event_target_checked(&ev);
        set_form_data.update(|form| form.newsletter = checked);
    };

    // Form submission
    let on_submit = move |ev: web_sys::Event| {
        ev.prevent_default();

        let current_form = form_data.get();
        match current_form.validate() {
            Ok(_) => {
                set_is_submitting.set(true);
                set_errors.set(ValidationErrors::new());

                // Simulate API call
                set_timeout(move || {
                    set_submit_message.set("Thank you for your message!".to_string());
                    set_form_data.set(ContactForm::default());
                    set_is_submitting.set(false);

                    // Clear success message after 3 seconds
                    set_timeout(move || {
                        set_submit_message.set(String::new());
                    }, 3000);
                }, 1000);
            }
            Err(validation_errors) => {
                set_errors.set(validation_errors);
            }
        }
    };

    view! { cx,
        div(class="contact-form-container") {
            h2 { "Contact Us" }

            form(on:submit=on_submit, class="contact-form") {
                // Name field
                div(class="form-group") {
                    label(for="name") { "Name *" }
                    input(
                        type="text",
                        id="name",
                        placeholder="Your full name",
                        value=form_data.get().name,
                        on:input=update_name,
                        class=move || {
                            if errors.get().field_errors().contains_key("name") {
                                "form-control error"
                            } else {
                                "form-control"
                            }
                        }
                    )
                    (move || {
                        errors.get().field_errors().get("name")
                            .and_then(|errors| errors.first())
                            .map(|error| view! { cx,
                                span(class="error-message") { (error.message.as_ref().unwrap_or(&"Invalid name".to_string())) }
                            })
                    })
                }

                // Email field
                div(class="form-group") {
                    label(for="email") { "Email *" }
                    input(
                        type="email",
                        id="email",
                        placeholder="your.email@example.com",
                        value=form_data.get().email,
                        on:input=update_email,
                        class=move || {
                            if errors.get().field_errors().contains_key("email") {
                                "form-control error"
                            } else {
                                "form-control"
                            }
                        }
                    )
                    (move || {
                        errors.get().field_errors().get("email")
                            .and_then(|errors| errors.first())
                            .map(|error| view! { cx,
                                span(class="error-message") { (error.message.as_ref().unwrap_or(&"Invalid email".to_string())) }
                            })
                    })
                }

                // Rating field
                div(class="form-group") {
                    label { "How would you rate our service?" }
                    select(
                        value=move || form_data.get().rating.map(|r| r.to_string()).unwrap_or_default(),
                        on:change=update_rating,
                        class="form-control"
                    ) {
                        option(value="") { "Select rating..." }
                        option(value="1") { "1 - Poor" }
                        option(value="2") { "2 - Fair" }
                        option(value="3") { "3 - Good" }
                        option(value="4") { "4 - Very Good" }
                        option(value="5") { "5 - Excellent" }
                    }
                    (move || {
                        errors.get().field_errors().get("rating")
                            .and_then(|errors| errors.first())
                            .map(|error| view! { cx,
                                span(class="error-message") { (error.message.as_ref().unwrap_or(&"Invalid rating".to_string())) }
                            })
                    })
                }

                // Message field
                div(class="form-group") {
                    label(for="message") { "Message *" }
                    textarea(
                        id="message",
                        placeholder="Tell us how we can help...",
                        value=form_data.get().message,
                        on:input=update_message,
                        rows="4",
                        class=move || {
                            if errors.get().field_errors().contains_key("message") {
                                "form-control error"
                            } else {
                                "form-control"
                            }
                        }
                    )
                    (move || {
                        errors.get().field_errors().get("message")
                            .and_then(|errors| errors.first())
                            .map(|error| view! { cx,
                                span(class="error-message") { (error.message.as_ref().unwrap_or(&"Invalid message".to_string())) }
                            })
                    })
                }

                // Newsletter checkbox
                div(class="form-group checkbox-group") {
                    label(class="checkbox-label") {
                        input(
                            type="checkbox",
                            checked=form_data.get().newsletter,
                            on:change=update_newsletter
                        )
                        " Subscribe to our newsletter"
                    }
                }

                // Submit button
                button(
                    type="submit",
                    disabled=is_submitting,
                    class=move || {
                        if is_submitting.get() {
                            "btn btn-primary disabled"
                        } else {
                            "btn btn-primary"
                        }
                    }
                ) {
                    (move || if is_submitting.get() { "Sending..." } else { "Send Message" })
                }
            }

            // Success message
            (if !submit_message.get().is_empty() {
                view! { cx,
                    div(class="success-message") {
                        (submit_message.get())
                    }
                }.into_view(cx)
            } else {
                view! { cx, }.into_view(cx)
            })
        }
    }
}

#[component]
fn LoginForm(cx: Scope) -> impl IntoView {
    // Form state with individual signals
    let (username, set_username) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());
    let (remember_me, set_remember_me) = create_signal(cx, false);
    let (is_loading, set_is_loading) = create_signal(cx, false);
    let (error_message, set_error_message) = create_signal(cx, String::new());

    let on_submit = move |ev: web_sys::Event| {
        ev.prevent_default();

        if username.get().is_empty() || password.get().is_empty() {
            set_error_message.set("Please fill in all fields".to_string());
            return;
        }

        set_is_loading.set(true);
        set_error_message.set(String::new());

        // Simulate login API call
        set_timeout(move || {
            // Mock authentication
            if username.get() == "admin" && password.get() == "password" {
                set_error_message.set("Login successful!".to_string());
            } else {
                set_error_message.set("Invalid username or password".to_string());
            }
            set_is_loading.set(false);
        }, 1500);
    };

    view! { cx,
        div(class="login-form") {
            h3 { "Login" }

            form(on:submit=on_submit) {
                div(class="form-group") {
                    label { "Username" }
                    input(
                        type="text",
                        placeholder="Enter username",
                        value=username,
                        on:input=move |ev| set_username.set(event_target_value(&ev)),
                        disabled=is_loading
                    )
                }

                div(class="form-group") {
                    label { "Password" }
                    input(
                        type="password",
                        placeholder="Enter password",
                        value=password,
                        on:input=move |ev| set_password.set(event_target_value(&ev)),
                        disabled=is_loading
                    )
                }

                div(class="form-group checkbox-group") {
                    label {
                        input(
                            type="checkbox",
                            checked=remember_me,
                            on:change=move |ev| set_remember_me.set(event_target_checked(&ev)),
                            disabled=is_loading
                        )
                        " Remember me"
                    }
                }

                button(
                    type="submit",
                    disabled=is_loading,
                    class="btn btn-primary"
                ) {
                    (move || if is_loading.get() { "Logging in..." } else { "Login" })
                }
            }

            (if !error_message.get().is_empty() {
                view! { cx,
                    div(class=move || {
                        if error_message.get().contains("successful") {
                            "success-message"
                        } else {
                            "error-message"
                        }
                    }) {
                        (error_message.get())
                    }
                }.into_view(cx)
            } else {
                view! { cx, }.into_view(cx)
            })
        }
    }
}

#[component]
fn MultiStepForm(cx: Scope) -> impl IntoView {
    let (current_step, set_current_step) = create_signal(cx, 1);
    let (form_data, set_form_data) = create_signal(cx, MultiStepData::default());

    let next_step = move |_| {
        if validate_current_step(current_step.get(), &form_data.get()) {
            set_current_step.update(|step| *step += 1);
        }
    };

    let prev_step = move |_| {
        set_current_step.update(|step| if *step > 1 { *step -= 1 });
    };

    let on_submit = move |ev: web_sys::Event| {
        ev.prevent_default();
        // Handle final submission
        log!("Form submitted: {:?}", form_data.get());
    };

    view! { cx,
        div(class="multi-step-form") {
            div(class="progress-indicator") {
                (1..=3).map(|step| {
                    view! { cx,
                        span(class=move || {
                            let current = current_step.get();
                            if step < current {
                                "step completed"
                            } else if step == current {
                                "step active"
                            } else {
                                "step"
                            }
                        }) {
                            (step)
                        }
                    }
                }).collect::<Vec<_>>()
            }

            form(on:submit=on_submit) {
                (match current_step.get() {
                    1 => view! { cx,
                        Step1(
                            data=form_data,
                            on_update=move |data| set_form_data.set(data)
                        )
                    }.into_view(cx),
                    2 => view! { cx,
                        Step2(
                            data=form_data,
                            on_update=move |data| set_form_data.set(data)
                        )
                    }.into_view(cx),
                    3 => view! { cx,
                        Step3(data=form_data)
                    }.into_view(cx),
                    _ => view! { cx, }.into_view(cx)
                })

                div(class="form-actions") {
                    (if current_step.get() > 1 {
                        view! { cx,
                            button(
                                type="button",
                                on:click=prev_step,
                                class="btn btn-secondary"
                            ) {
                                "Previous"
                            }
                        }.into_view(cx)
                    } else {
                        view! { cx, }.into_view(cx)
                    })

                    (if current_step.get() < 3 {
                        view! { cx,
                            button(
                                type="button",
                                on:click=next_step,
                                class="btn btn-primary"
                            ) {
                                "Next"
                            }
                        }.into_view(cx)
                    } else {
                        view! { cx,
                            button(type="submit", class="btn btn-primary") {
                                "Submit"
                            }
                        }.into_view(cx)
                    })
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MultiStepData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub preferences: Vec<String>,
}

#[component]
fn Step1(cx: Scope, data: ReadSignal<MultiStepData>, on_update: Callback<MultiStepData>) -> impl IntoView {
    let mut current_data = data.get();

    let update_first_name = move |ev| {
        current_data.first_name = event_target_value(&ev);
        on_update.call(current_data.clone());
    };

    let update_last_name = move |ev| {
        current_data.last_name = event_target_value(&ev);
        on_update.call(current_data.clone());
    };

    view! { cx,
        div(class="step") {
            h3 { "Personal Information" }

            div(class="form-row") {
                div(class="form-group") {
                    label { "First Name" }
                    input(
                        type="text",
                        value=data.get().first_name,
                        on:input=update_first_name
                    )
                }

                div(class="form-group") {
                    label { "Last Name" }
                    input(
                        type="text",
                        value=data.get().last_name,
                        on:input=update_last_name
                    )
                }
            }
        }
    }
}

#[component]
fn Step2(cx: Scope, data: ReadSignal<MultiStepData>, on_update: Callback<MultiStepData>) -> impl IntoView {
    let mut current_data = data.get();

    let update_email = move |ev| {
        current_data.email = event_target_value(&ev);
        on_update.call(current_data.clone());
    };

    let update_phone = move |ev| {
        current_data.phone = event_target_value(&ev);
        on_update.call(current_data.clone());
    };

    view! { cx,
        div(class="step") {
            h3 { "Contact Information" }

            div(class="form-group") {
                label { "Email" }
                input(
                    type="email",
                    value=data.get().email,
                    on:input=update_email
                )
            }

            div(class="form-group") {
                label { "Phone" }
                input(
                    type="tel",
                    value=data.get().phone,
                    on:input=update_phone
                )
            }
        }
    }
}

#[component]
fn Step3(cx: Scope, data: ReadSignal<MultiStepData>) -> impl IntoView {
    view! { cx,
        div(class="step") {
            h3 { "Review Your Information" }

            div(class="review-section") {
                h4 { "Personal Information" }
                p { "Name: " (data.get().first_name) " " (data.get().last_name) }

                h4 { "Contact Information" }
                p { "Email: " (data.get().email) }
                p { "Phone: " (data.get().phone) }
            }
        }
    }
}

fn validate_current_step(step: i32, data: &MultiStepData) -> bool {
    match step {
        1 => !data.first_name.is_empty() && !data.last_name.is_empty(),
        2 => !data.email.is_empty() && !data.phone.is_empty(),
        _ => true,
    }
}

fn main() {
    mount_to_body(|| view! {
        <div class="app">
            <ContactFormComponent/>
            <hr/>
            <LoginForm/>
            <hr/>
            <MultiStepForm/>
        </div>
    })
}
```

## Key Concepts Demonstrated

### 1. Controlled Components
```rust
input(
    value=form_data.get().name,
    on:input=move |ev| {
        let value = event_target_value(&ev);
        set_form_data.update(|form| form.name = value);
    }
)
```
- Form inputs are controlled by component state
- Changes trigger state updates
- Values are derived from signals

### 2. Form Validation
```rust
#[derive(Validate)]
pub struct ContactForm {
    #[validate(length(min = 2, message = "Name must be at least 2 characters"))]
    pub name: String,
    // ...
}

let current_form = form_data.get();
match current_form.validate() {
    Ok(_) => { /* Submit form */ }
    Err(validation_errors) => {
        set_errors.set(validation_errors);
    }
}
```
- Uses `validator` crate for declarative validation
- Validation errors are displayed in UI
- Form submission is blocked on validation errors

### 3. Dynamic Form State
```rust
let (errors, set_errors) = create_signal(cx, ValidationErrors::new());
let (is_submitting, set_is_submitting) = create_signal(cx, false);
let (submit_message, set_submit_message) = create_signal(cx, String::new());
```
- Multiple signals for different aspects of form state
- Loading states, error states, success states
- All reactive and update the UI automatically

### 4. Form Submission Handling
```rust
let on_submit = move |ev: web_sys::Event| {
    ev.prevent_default();

    let current_form = form_data.get();
    match current_form.validate() {
        Ok(_) => {
            set_is_submitting.set(true);
            // Simulate API call
            set_timeout(move || {
                set_submit_message.set("Thank you!".to_string());
                set_is_submitting.set(false);
            }, 1000);
        }
        Err(validation_errors) => {
            set_errors.set(validation_errors);
        }
    }
};
```
- Prevents default form submission
- Validates before submission
- Handles async operations with loading states

### 5. Conditional Rendering
```rust
(if !submit_message.get().is_empty() {
    view! { cx,
        div(class="success-message") {
            (submit_message.get())
        }
    }.into_view(cx)
} else {
    view! { cx, }.into_view(cx)
})
```
- Success/error messages appear conditionally
- Form elements show/hide based on state

### 6. Reactive Classes
```rust
class=move || {
    if errors.get().field_errors().contains_key("name") {
        "form-control error"
    } else {
        "form-control"
    }
}
```
- CSS classes change based on validation state
- Visual feedback for form errors

### 7. Multi-Step Forms
```rust
(match current_step.get() {
    1 => view! { cx, <Step1 ... /> }.into_view(cx),
    2 => view! { cx, <Step2 ... /> }.into_view(cx),
    3 => view! { cx, <Step3 ... /> }.into_view(cx),
    _ => view! { cx, }.into_view(cx)
})
```
- Complex forms broken into steps
- Progress indicator
- Step validation before proceeding

## Advanced Patterns

### Form State Management
```rust
// Individual signals approach
let (username, set_username) = create_signal(cx, String::new());
let (password, set_password) = create_signal(cx, String::new());

// Struct-based approach
let (form_data, set_form_data) = create_signal(cx, ContactForm::default());
```
- Choose between individual signals or struct-based state
- Struct approach is better for complex forms
- Individual signals work well for simple forms

### Real-time Validation
```rust
let update_email = move |ev| {
    let value = event_target_value(&ev);
    set_form_data.update(|form| form.email = value);
    // Clear previous errors
    set_errors.update(|errors| {
        errors.errors_mut().remove("email");
    });
};
```
- Clear validation errors as user types
- Provide immediate feedback
- Don't overwhelm with too many error messages

### Async Form Submission
```rust
set_is_submitting.set(true);
// Simulate API call
set_timeout(move || {
    set_submit_message.set("Thank you!".to_string());
    set_form_data.set(ContactForm::default());
    set_is_submitting.set(false);
}, 1000);
```
- Disable form during submission
- Show loading indicators
- Handle success/error states
- Reset form after successful submission

### Form Data Serialization
```rust
#[derive(Serialize, Deserialize)]
pub struct ContactForm {
    // Fields...
}

// Serialize for API calls
let json_data = serde_json::to_string(&form_data.get())?;

// Deserialize from API responses
let response_data: ContactForm = serde_json::from_str(&response)?;
```

## Best Practices

### 1. Form Structure
```rust
// ✅ Good: Use semantic HTML
form(on:submit=on_submit) {
    fieldset {
        legend { "Contact Information" }
        // Form fields...
    }
    button(type="submit") { "Submit" }
}

// ❌ Avoid: Missing form semantics
div {
    // Form fields...
    button(on:click=on_submit) { "Submit" }
}
```

### 2. Accessibility
```rust
// ✅ Good: Proper labels and ARIA
label(for="email") { "Email Address" }
input(type="email", id="email", aria-describedby="email-help")

// ✅ Good: Error announcements
div(role="alert", aria-live="polite") {
    (error_message)
}
```

### 3. Validation Strategy
```rust
// ✅ Good: Validate on blur and submit
input(
    on:blur=move |_| validate_field("email"),
    on:input=move |_| clear_field_error("email")
)

// ✅ Good: Progressive validation
let validate_step = move || {
    match current_step.get() {
        1 => validate_personal_info(),
        2 => validate_contact_info(),
        _ => true
    }
};
```

### 4. Error Handling
```rust
// ✅ Good: User-friendly error messages
match validation_error.code {
    "email" => "Please enter a valid email address",
    "required" => "This field is required",
    "min_length" => format!("Must be at least {} characters", min_len),
    _ => "Please check this field"
}
```

### 5. Performance Optimization
```rust
// ✅ Good: Debounce validation
let debounced_validate = debounce(cx, move || {
    validate_form()
}, 300);

// ✅ Good: Memoize expensive validation
let validation_result = create_memo(cx, move |_| {
    expensive_validation(form_data.get())
});
```

## Testing Form Components

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_form_validation() {
        let runtime = create_runtime();
        let scope = create_scope(runtime, |cx| {
            let (form_data, _) = create_signal(cx, ContactForm::default());
            let (errors, _) = create_signal(cx, ValidationErrors::new());

            // Test empty form validation
            let empty_form = ContactForm::default();
            assert!(empty_form.validate().is_err());

            // Test valid form
            let valid_form = ContactForm {
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
                message: "This is a test message with enough characters".to_string(),
                rating: Some(5),
                newsletter: true,
            };
            assert!(valid_form.validate().is_ok());
        });
        dispose_runtime(runtime);
    }

    #[test]
    fn test_form_submission() {
        let runtime = create_runtime();
        let scope = create_scope(runtime, |cx| {
            let (form_data, set_form_data) = create_signal(cx, ContactForm {
                name: "Test User".to_string(),
                email: "test@example.com".to_string(),
                message: "Test message".to_string(),
                rating: Some(4),
                newsletter: false,
            });
            let (is_submitting, set_is_submitting) = create_signal(cx, false);

            // Simulate form submission
            set_is_submitting.set(true);
            assert!(is_submitting.get());

            // Simulate successful submission
            set_form_data.set(ContactForm::default());
            set_is_submitting.set(false);

            assert!(!is_submitting.get());
            assert_eq!(form_data.get().name, "");
        });
        dispose_runtime(runtime);
    }
}
```

## Common Patterns

### Controlled Input
```rust
#[component]
fn ControlledInput(
    cx: Scope,
    value: ReadSignal<String>,
    on_change: Callback<String>
) -> impl IntoView {
    view! { cx,
        input(
            value=value,
            on:input=move |ev| on_change.call(event_target_value(&ev))
        )
    }
}
```

### Form Field with Validation
```rust
#[component]
fn ValidatedField<T>(
    cx: Scope,
    label: String,
    value: ReadSignal<String>,
    error: ReadSignal<Option<String>>,
    on_change: Callback<String>,
    input_type: String
) -> impl IntoView {
    view! { cx,
        div(class="form-group") {
            label { (label) }
            input(
                type=input_type,
                value=value,
                on:input=move |ev| on_change.call(event_target_value(&ev)),
                class=move || {
                    if error.get().is_some() {
                        "form-control error"
                    } else {
                        "form-control"
                    }
                }
            )
            (move || {
                error.get().map(|err| view! { cx,
                    span(class="error-message") { (err) }
                })
            })
        }
    }
}
```

### Form Context
```rust
#[derive(Clone)]
pub struct FormContext {
    pub errors: ReadSignal<ValidationErrors>,
    pub is_submitting: ReadSignal<bool>,
    pub submit: Callback<()>,
}

pub static FORM_CONTEXT: Lazy<Context<FormContext>> = Lazy::new(|| {
    create_context::<FormContext>(None)
});
```

## Related Examples

- [Basic Counter Example](basic-counter.md) - Simple reactive state
- [Todo App Example](todo-app.md) - Complex state management
- [Data Fetching Example](data-fetching.md) - Async operations

## Next Steps

1. Add file upload capabilities
2. Implement form auto-save
3. Create dynamic form fields (add/remove)
4. Add form field dependencies
5. Implement form analytics

This form handling example demonstrates how to build robust, user-friendly forms with proper validation, error handling, and state management in Leptos.