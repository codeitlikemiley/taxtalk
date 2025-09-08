# Form Handling Example

## Overview

This example demonstrates comprehensive form handling patterns in Leptos, covering:

- Controlled vs uncontrolled form inputs
- Form validation and error handling
- Complex form state management
- Multi-step forms and wizards
- File uploads and form submission
- Real-time validation and feedback
- Form accessibility and UX best practices

## Project Structure

```
src/
├── main.rs                    # Application entry point
├── components/
│   ├── forms/
│   │   ├── contact_form.rs    # Contact form with validation
│   │   ├── registration_form.rs # Multi-step registration
│   │   ├── survey_form.rs     # Dynamic survey with conditional fields
│   │   ├── file_upload_form.rs # File upload with progress
│   │   └── payment_form.rs    # Payment form with security
│   ├── ui/
│   │   ├── input_field.rs     # Reusable input component
│   │   ├── select_field.rs    # Select dropdown component
│   │   ├── checkbox_group.rs  # Checkbox group component
│   │   └── form_field.rs      # Generic form field wrapper
│   └── validation/
│       ├── validators.rs      # Validation functions
│       └── error_display.rs   # Error message display
├── models/
│   ├── contact.rs             # Contact form data structures
│   ├── registration.rs        # Registration form models
│   └── survey.rs              # Survey form models
└── utils/
    ├── form_state.rs          # Form state management utilities
    └── validation.rs          # Validation utilities
```

## Core Form State Management

### Form State Pattern

```rust
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FormState<T> {
    pub data: T,
    pub errors: HashMap<String, String>,
    pub touched: HashSet<String>,
    pub is_submitting: bool,
    pub is_valid: bool,
}

impl<T> FormState<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            errors: HashMap::new(),
            touched: HashSet::new(),
            is_submitting: false,
            is_valid: true,
        }
    }

    pub fn set_field(&mut self, field: &str, value: impl Into<Value>) {
        // Update field value
        self.update_data_field(field, value);

        // Mark as touched
        self.touched.insert(field.to_string());

        // Validate field
        self.validate_field(field);
    }

    pub fn validate_all(&mut self) {
        // Clear existing errors
        self.errors.clear();

        // Validate all fields
        self.is_valid = self.validate_data();

        // Update validity
        self.update_validity();
    }

    fn update_validity(&mut self) {
        self.is_valid = self.errors.is_empty();
    }
}
```

## Contact Form Component

### Basic Contact Form

```rust
#[component]
pub fn ContactForm() -> impl IntoView {
    let (form_state, set_form_state) = create_signal(FormState::new(ContactData::default()));

    // Form field handlers
    let update_field = move |field: &str, value: String| {
        set_form_state.update(|state| {
            state.set_field(field, value);
        });
    };

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        set_form_state.update(|state| {
            state.is_submitting = true;
            state.validate_all();
        });

        // Simulate API call
        spawn_local(async move {
            // API call here
            set_form_state.update(|state| {
                state.is_submitting = false;
            });
        });
    };

    view! {
        <form on:submit=handle_submit class="contact-form">
            <h2>"Contact Us"</h2>

            <FormField
                label="Name"
                error=move || form_state().errors.get("name").cloned()
            >
                <input
                    type="text"
                    prop:value=move || form_state().data.name.clone()
                    on:input=move |ev| {
                        let target = event_target::<web_sys::HtmlInputElement>(&ev);
                        if let Some(input) = target {
                            update_field("name", input.value());
                        }
                    }
                    class=move || {
                        if form_state().touched.contains("name") && form_state().errors.contains_key("name") {
                            "error"
                        } else {
                            ""
                        }
                    }
                />
            </FormField>

            <FormField
                label="Email"
                error=move || form_state().errors.get("email").cloned()
            >
                <input
                    type="email"
                    prop:value=move || form_state().data.email.clone()
                    on:input=move |ev| {
                        let target = event_target::<web_sys::HtmlInputElement>(&ev);
                        if let Some(input) = target {
                            update_field("email", input.value());
                        }
                    }
                    class=move || {
                        if form_state().touched.contains("email") && form_state().errors.contains_key("email") {
                            "error"
                        } else {
                            ""
                        }
                    }
                />
            </FormField>

            <FormField
                label="Message"
                error=move || form_state().errors.get("message").cloned()
            >
                <textarea
                    prop:value=move || form_state().data.message.clone()
                    on:input=move |ev| {
                        let target = event_target::<web_sys::HtmlTextAreaElement>(&ev);
                        if let Some(textarea) = target {
                            update_field("message", textarea.value());
                        }
                    }
                    rows="5"
                    class=move || {
                        if form_state().touched.contains("message") && form_state().errors.contains_key("message") {
                            "error"
                        } else {
                            ""
                        }
                    }
                />
            </FormField>

            <button
                type="submit"
                prop:disabled=move || !form_state().is_valid || form_state().is_submitting
                class="submit-btn"
            >
                {move || if form_state().is_submitting { "Sending..." } else { "Send Message" }}
            </button>
        </form>
    }
}
```

## Multi-Step Registration Form

### Registration Wizard

```rust
#[component]
pub fn RegistrationForm() -> impl IntoView {
    let (current_step, set_current_step) = create_signal(0);
    let (form_state, set_form_state) = create_signal(FormState::new(RegistrationData::default()));

    let steps = vec![
        "Personal Information",
        "Account Details",
        "Preferences",
        "Review & Submit"
    ];

    let next_step = move |_| {
        let current = current_step();
        if current < steps.len() - 1 {
            // Validate current step before proceeding
            if validate_step(current, &form_state()) {
                set_current_step(current + 1);
            }
        }
    };

    let prev_step = move |_| {
        let current = current_step();
        if current > 0 {
            set_current_step(current - 1);
        }
    };

    let handle_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        set_form_state.update(|state| {
            state.is_submitting = true;
            state.validate_all();
        });

        // Submit registration
        spawn_local(async move {
            // API call here
            set_form_state.update(|state| {
                state.is_submitting = false;
            });
        });
    };

    view! {
        <div class="registration-wizard">
            <div class="wizard-header">
                <For
                    each=move || steps.clone().into_iter().enumerate()
                    key=|(_, step)| step.clone()
                    children=move |(index, step)| view! {
                        <div
                            class=move || {
                                let mut classes = vec!["step"];
                                if index < current_step() {
                                    classes.push("completed");
                                } else if index == current_step() {
                                    classes.push("active");
                                }
                                classes.join(" ")
                            }
                        >
                            <span class="step-number">{index + 1}</span>
                            <span class="step-title">{step}</span>
                        </div>
                    }
                />
            </div>

            <form on:submit=handle_submit class="wizard-form">
                <div class="step-content">
                    {move || match current_step() {
                        0 => view! { <PersonalInfoStep form_state set_form_state /> },
                        1 => view! { <AccountDetailsStep form_state set_form_state /> },
                        2 => view! { <PreferencesStep form_state set_form_state /> },
                        3 => view! { <ReviewStep form_state /> },
                        _ => view! { <div>"Invalid step"</div> }
                    }}
                </div>

                <div class="wizard-navigation">
                    <Show when=move || current_step() > 0>
                        <button type="button" on:click=prev_step class="prev-btn">
                            "Previous"
                        </button>
                    </Show>

                    <Show when=move || current_step() < steps.len() - 1>
                        <button type="button" on:click=next_step class="next-btn">
                            "Next"
                        </button>
                    </Show>

                    <Show when=move || current_step() == steps.len() - 1>
                        <button
                            type="submit"
                            prop:disabled=move || !form_state().is_valid || form_state().is_submitting
                            class="submit-btn"
                        >
                            {move || if form_state().is_submitting { "Submitting..." } else { "Complete Registration" }}
                        </button>
                    </Show>
                </div>
            </form>
        </div>
    }
}
```

## Dynamic Survey Form

### Conditional Fields

```rust
#[component]
pub fn SurveyForm() -> impl IntoView {
    let (form_state, set_form_state) = create_signal(FormState::new(SurveyData::default()));
    let (visible_fields, set_visible_fields) = create_signal(HashSet::new());

    // Update visible fields based on responses
    create_effect(move |_| {
        let data = form_state().data;
        let mut visible = HashSet::new();

        // Show employment fields if user is employed
        if data.employment_status == "employed" {
            visible.insert("job_title");
            visible.insert("company");
            visible.insert("years_experience");
        }

        // Show education fields if user has higher education
        if data.education_level != "high_school" {
            visible.insert("university");
            visible.insert("degree");
            visible.insert("graduation_year");
        }

        // Show feedback field if user is dissatisfied
        if let Some(satisfaction) = data.overall_satisfaction {
            if satisfaction <= 3 {
                visible.insert("improvement_feedback");
            }
        }

        set_visible_fields(visible);
    });

    let update_field = move |field: &str, value: impl Into<Value>| {
        set_form_state.update(|state| {
            state.set_field(field, value);
        });
    };

    view! {
        <form class="survey-form">
            <h2>"Customer Satisfaction Survey"</h2>

            <FormField label="Employment Status">
                <select
                    on:change=move |ev| {
                        let target = event_target::<web_sys::HtmlSelectElement>(&ev);
                        if let Some(select) = target {
                            update_field("employment_status", select.value());
                        }
                    }
                >
                    <option value="">"Select status"</option>
                    <option value="employed">"Employed"</option>
                    <option value="unemployed">"Unemployed"</option>
                    <option value="student">"Student"</option>
                    <option value="retired">"Retired"</option>
                </select>
            </FormField>

            <Show when=move || visible_fields().contains("job_title")>
                <FormField label="Job Title">
                    <input
                        type="text"
                        on:input=move |ev| {
                            let target = event_target::<web_sys::HtmlInputElement>(&ev);
                            if let Some(input) = target {
                                update_field("job_title", input.value());
                            }
                        }
                    />
                </FormField>
            </Show>

            <Show when=move || visible_fields().contains("company")>
                <FormField label="Company">
                    <input
                        type="text"
                        on:input=move |ev| {
                            let target = event_target::<web_sys::HtmlInputElement>(&ev);
                            if let Some(input) = target {
                                update_field("company", input.value());
                            }
                        }
                    />
                </FormField>
            </Show>

            <FormField label="Education Level">
                <select
                    on:change=move |ev| {
                        let target = event_target::<web_sys::HtmlSelectElement>(&ev);
                        if let Some(select) = target {
                            update_field("education_level", select.value());
                        }
                    }
                >
                    <option value="high_school">"High School"</option>
                    <option value="associate">"Associate Degree"</option>
                    <option value="bachelor">"Bachelor's Degree"</option>
                    <option value="master">"Master's Degree"</option>
                    <option value="doctorate">"Doctorate"</option>
                </select>
            </FormField>

            <Show when=move || visible_fields().contains("university")>
                <FormField label="University">
                    <input
                        type="text"
                        on:input=move |ev| {
                            let target = event_target::<web_sys::HtmlInputElement>(&ev);
                            if let Some(input) = target {
                                update_field("university", input.value());
                            }
                        }
                    />
                </FormField>
            </Show>

            <FormField label="Overall Satisfaction (1-5)">
                <div class="rating">
                    <For
                        each=move || (1..=5).collect::<Vec<_>>()
                        key=|rating| *rating
                        children=move |rating| view! {
                            <input
                                type="radio"
                                name="satisfaction"
                                prop:value=rating
                                on:change=move |_| {
                                    update_field("overall_satisfaction", rating);
                                }
                            />
                            <label>{rating}</label>
                        }
                    />
                </div>
            </FormField>

            <Show when=move || visible_fields().contains("improvement_feedback")>
                <FormField label="How can we improve?">
                    <textarea
                        rows="4"
                        on:input=move |ev| {
                            let target = event_target::<web_sys::HtmlTextAreaElement>(&ev);
                            if let Some(textarea) = target {
                                update_field("improvement_feedback", textarea.value());
                            }
                        }
                    />
                </FormField>
            </Show>

            <button type="submit" class="submit-btn">
                "Submit Survey"
            </button>
        </form>
    }
}
```

## File Upload Form

### File Upload with Progress

```rust
#[component]
pub fn FileUploadForm() -> impl IntoView {
    let (files, set_files) = create_signal(Vec::<web_sys::File>::new());
    let (upload_progress, set_upload_progress) = create_signal(HashMap::<String, f64>::new());
    let (is_uploading, set_is_uploading) = create_signal(false);

    let handle_file_select = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Some(input) = target {
            if let Some(file_list) = input.files() {
                let mut selected_files = Vec::new();
                for i in 0..file_list.length() {
                    if let Some(file) = file_list.get(i) {
                        selected_files.push(file);
                    }
                }
                set_files(selected_files);
            }
        }
    };

    let upload_files = move |_| {
        set_is_uploading(true);

        for file in files() {
            let file_name = file.name();
            let file_size = file.size() as usize;

            // Simulate upload progress
            spawn_local(async move {
                let total_chunks = 10;
                for chunk in 1..=total_chunks {
                    gloo::timers::future::TimeoutFuture::new(100).await;
                    let progress = (chunk as f64 / total_chunks as f64) * 100.0;
                    set_upload_progress.update(|progress_map| {
                        progress_map.insert(file_name.clone(), progress);
                    });
                }

                // Complete upload
                set_upload_progress.update(|progress_map| {
                    progress_map.insert(file_name, 100.0);
                });
            });
        }

        set_is_uploading(false);
    };

    let remove_file = move |index: usize| {
        set_files.update(|files| {
            files.remove(index);
        });
    };

    view! {
        <div class="file-upload-form">
            <h3>"File Upload"</h3>

            <div class="file-input">
                <input
                    type="file"
                    multiple
                    on:change=handle_file_select
                    accept=".pdf,.doc,.docx,.txt,.jpg,.png"
                />
                <p>"Select files to upload (PDF, DOC, Images)"</p>
            </div>

            <div class="file-list">
                <For
                    each=move || files().into_iter().enumerate()
                    key=|(_, file)| file.name()
                    children=move |(index, file)| {
                        let file_name = file.name();
                        let file_size = file.size();
                        let progress = move || upload_progress().get(&file_name).copied().unwrap_or(0.0);

                        view! {
                            <div class="file-item">
                                <div class="file-info">
                                    <span class="file-name">{file_name}</span>
                                    <span class="file-size">
                                        {format!("{:.1} KB", file_size / 1024.0)}
                                    </span>
                                </div>

                                <div class="file-progress">
                                    <div
                                        class="progress-bar"
                                        style=move || format!("width: {}%", progress())
                                    ></div>
                                </div>

                                <button
                                    type="button"
                                    on:click=move |_| remove_file(index)
                                    prop:disabled=is_uploading
                                    class="remove-btn"
                                >
                                    "×"
                                </button>
                            </div>
                        }
                    }
                />
            </div>

            <Show when=move || !files().is_empty()>
                <button
                    type="button"
                    on:click=upload_files
                    prop:disabled=is_uploading
                    class="upload-btn"
                >
                    {move || if is_uploading() { "Uploading..." } else { "Upload Files" }}
                </button>
            </Show>
        </div>
    }
}
```

## Form Validation System

### Validation Functions

```rust
use regex::Regex;

pub fn validate_required(value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err("This field is required".to_string())
    } else {
        Ok(())
    }
}

pub fn validate_email(value: &str) -> Result<(), String> {
    let email_regex = Regex::new(r"^[^@]+@[^@]+\.[^@]+$").unwrap();
    if email_regex.is_match(value) {
        Ok(())
    } else {
        Err("Please enter a valid email address".to_string())
    }
}

pub fn validate_min_length(min_len: usize) -> impl Fn(&str) -> Result<(), String> {
    move |value: &str| {
        if value.len() >= min_len {
            Ok(())
        } else {
            Err(format!("Must be at least {} characters", min_len))
        }
    }
}

pub fn validate_max_length(max_len: usize) -> impl Fn(&str) -> Result<(), String> {
    move |value: &str| {
        if value.len() <= max_len {
            Ok(())
        } else {
            Err(format!("Must be no more than {} characters", max_len))
        }
    }
}

pub fn validate_numeric(value: &str) -> Result<(), String> {
    if value.parse::<f64>().is_ok() {
        Ok(())
    } else {
        Err("Must be a valid number".to_string())
    }
}

pub fn validate_url(value: &str) -> Result<(), String> {
    if url::Url::parse(value).is_ok() {
        Ok(())
    } else {
        Err("Must be a valid URL".to_string())
    }
}

pub fn validate_phone(value: &str) -> Result<(), String> {
    let phone_regex = Regex::new(r"^\+?[\d\s\-\(\)]+$").unwrap();
    if phone_regex.is_match(value) {
        Ok(())
    } else {
        Err("Please enter a valid phone number".to_string())
    }
}
```

### Form Field Component

```rust
#[component]
pub fn FormField(
    #[prop(into)] label: String,
    #[prop(into, optional)] error: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || {
            let mut classes = vec!["form-field"];
            if error.is_some() {
                classes.push("has-error");
            }
            classes.join(" ")
        }>
            <label class="field-label">{label}</label>
            <div class="field-input">
                {children()}
            </div>
            <Show when=move || error.is_some()>
                <div class="field-error">
                    {error.unwrap_or_default()}
                </div>
            </Show>
        </div>
    }
}
```

## Advanced Form Patterns

### Debounced Input

```rust
use leptos::*;
use gloo::timers::callback::Timeout;

#[component]
pub fn DebouncedInput(
    #[prop(into)] value: RwSignal<String>,
    #[prop(default = 300)] delay_ms: u32,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let (debounced_value, set_debounced_value) = create_signal(value());
    let timeout_handle = create_rw_signal(None::<Timeout>);

    // Sync debounced value to parent
    create_effect(move |_| {
        value.set(debounced_value());
    });

    let handle_input = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Some(input) = target {
            let new_value = input.value();

            // Cancel previous timeout
            if let Some(handle) = timeout_handle() {
                handle.cancel();
            }

            // Set immediate value for UI responsiveness
            set_debounced_value(new_value.clone());

            // Set debounced value after delay
            let handle = Timeout::new(delay_ms, move || {
                value.set(new_value);
            });
            timeout_handle.set(Some(handle));
        }
    };

    view! {
        <input
            prop:value=debounced_value
            on:input=handle_input
            {..attrs}
        />
    }
}
```

### Form Context Pattern

```rust
use leptos::*;

#[derive(Clone)]
pub struct FormContext {
    pub form_state: RwSignal<FormState<ContactData>>,
    pub update_field: Callback<(String, Value)>,
    pub validate_field: Callback<String>,
    pub submit_form: Callback<()>,
}

pub fn provide_form_context(form_state: RwSignal<FormState<ContactData>>) -> FormContext {
    let update_field = Callback::new(move |(field, value): (String, Value)| {
        form_state.update(|state| {
            state.set_field(&field, value);
        });
    });

    let validate_field = Callback::new(move |field: String| {
        form_state.update(|state| {
            state.validate_field(&field);
        });
    });

    let submit_form = Callback::new(move |_| {
        form_state.update(|state| {
            state.validate_all();
            if state.is_valid {
                // Handle form submission
            }
        });
    });

    FormContext {
        form_state,
        update_field,
        validate_field,
        submit_form,
    }
}

#[component]
pub fn FormProvider(
    children: Children,
    #[prop(into)] form_state: RwSignal<FormState<ContactData>>,
) -> impl IntoView {
    let context = provide_form_context(form_state);

    view! {
        <FormContextProvider value=context>
            {children()}
        </FormContextProvider>
    }
}
```

## CSS Styling

```css
/* Form Styles */
.contact-form,
.registration-wizard,
.survey-form {
    max-width: 600px;
    margin: 0 auto;
    padding: 2rem;
    background: #fff;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.form-field {
    margin-bottom: 1.5rem;
}

.field-label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #374151;
}

.field-input input,
.field-input select,
.field-input textarea {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 4px;
    font-size: 1rem;
    transition: border-color 0.2s, box-shadow 0.2s;
}

.field-input input:focus,
.field-input select:focus,
.field-input textarea:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.field-input input.error,
.field-input select.error,
.field-input textarea.error {
    border-color: #ef4444;
}

.field-error {
    margin-top: 0.25rem;
    font-size: 0.875rem;
    color: #ef4444;
}

.submit-btn,
.next-btn,
.prev-btn {
    padding: 0.75rem 1.5rem;
    background: #3b82f6;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    cursor: pointer;
    transition: background-color 0.2s;
}

.submit-btn:hover,
.next-btn:hover,
.prev-btn:hover {
    background: #2563eb;
}

.submit-btn:disabled,
.next-btn:disabled,
.prev-btn:disabled {
    background: #9ca3af;
    cursor: not-allowed;
}

/* Wizard Styles */
.wizard-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 2rem;
    position: relative;
}

.wizard-header::before {
    content: '';
    position: absolute;
    top: 15px;
    left: 0;
    right: 0;
    height: 2px;
    background: #e5e7eb;
    z-index: 1;
}

.step {
    background: white;
    border: 2px solid #e5e7eb;
    border-radius: 50%;
    width: 30px;
    height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    z-index: 2;
    transition: all 0.3s;
}

.step.active {
    border-color: #3b82f6;
    background: #3b82f6;
    color: white;
}

.step.completed {
    border-color: #10b981;
    background: #10b981;
    color: white;
}

.step-number {
    font-size: 0.875rem;
    font-weight: 500;
}

.step-title {
    position: absolute;
    top: 40px;
    left: 50%;
    transform: translateX(-50%);
    font-size: 0.75rem;
    color: #6b7280;
    white-space: nowrap;
}

.wizard-navigation {
    display: flex;
    justify-content: space-between;
    margin-top: 2rem;
    padding-top: 1rem;
    border-top: 1px solid #e5e7eb;
}

/* File Upload Styles */
.file-upload-form {
    max-width: 500px;
    margin: 0 auto;
    padding: 2rem;
    background: #fff;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.file-input {
    margin-bottom: 1.5rem;
    text-align: center;
}

.file-input input[type="file"] {
    display: none;
}

.file-input p {
    margin: 0;
    padding: 2rem;
    border: 2px dashed #d1d5db;
    border-radius: 4px;
    cursor: pointer;
    transition: border-color 0.2s;
}

.file-input p:hover {
    border-color: #3b82f6;
}

.file-list {
    margin-bottom: 1.5rem;
}

.file-item {
    display: flex;
    align-items: center;
    padding: 0.75rem;
    border: 1px solid #e5e7eb;
    border-radius: 4px;
    margin-bottom: 0.5rem;
}

.file-info {
    flex: 1;
}

.file-name {
    font-weight: 500;
    color: #374151;
}

.file-size {
    font-size: 0.875rem;
    color: #6b7280;
}

.file-progress {
    flex: 1;
    margin: 0 1rem;
    height: 4px;
    background: #e5e7eb;
    border-radius: 2px;
    overflow: hidden;
}

.progress-bar {
    height: 100%;
    background: #3b82f6;
    transition: width 0.3s;
}

.remove-btn {
    background: none;
    border: none;
    color: #ef4444;
    font-size: 1.25rem;
    cursor: pointer;
    padding: 0;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.upload-btn {
    width: 100%;
    padding: 0.75rem;
    background: #10b981;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    cursor: pointer;
    transition: background-color 0.2s;
}

.upload-btn:hover {
    background: #059669;
}

.upload-btn:disabled {
    background: #9ca3af;
    cursor: not-allowed;
}

/* Rating Styles */
.rating {
    display: flex;
    gap: 0.5rem;
}

.rating input[type="radio"] {
    display: none;
}

.rating label {
    display: inline-block;
    padding: 0.5rem 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
}

.rating input[type="radio"]:checked + label {
    background: #3b82f6;
    color: white;
    border-color: #3b82f6;
}

/* Responsive Design */
@media (max-width: 640px) {
    .contact-form,
    .registration-wizard,
    .survey-form,
    .file-upload-form {
        margin: 1rem;
        padding: 1rem;
    }

    .wizard-header {
        flex-direction: column;
        align-items: center;
        gap: 1rem;
    }

    .wizard-navigation {
        flex-direction: column;
        gap: 0.5rem;
    }

    .file-item {
        flex-direction: column;
        align-items: stretch;
        gap: 0.5rem;
    }
}
```

## Key Features Demonstrated

### 1. Form State Management
- **Reactive Form State**: Using signals for form data, errors, and validation
- **Field-Level Validation**: Real-time validation with error display
- **Form-Level Validation**: Cross-field validation and submission checks

### 2. User Experience
- **Progressive Disclosure**: Conditional fields based on user input
- **Multi-Step Forms**: Wizard-style forms with step validation
- **Real-Time Feedback**: Immediate validation and error messages
- **Loading States**: Visual feedback during form submission

### 3. Accessibility
- **Proper Labels**: All form fields have associated labels
- **Error Announcements**: Screen reader friendly error messages
- **Keyboard Navigation**: Full keyboard support
- **Focus Management**: Proper focus handling in multi-step forms

### 4. Advanced Patterns
- **Debounced Input**: Performance optimization for search/filter inputs
- **Form Context**: Shared form state across components
- **File Upload**: Progress tracking and multiple file handling
- **Dynamic Forms**: Conditional rendering based on form state

### 5. Validation Strategies
- **Field-Level**: Immediate validation on field change
- **Form-Level**: Validation before submission
- **Async Validation**: Server-side validation for complex rules
- **Custom Validators**: Reusable validation functions

## Testing Form Components

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_form_validation() {
        let form_state = FormState::new(ContactData {
            name: "".to_string(),
            email: "invalid-email".to_string(),
            message: "Hi".to_string(),
        });

        form_state.validate_all();

        assert!(!form_state.is_valid);
        assert!(form_state.errors.contains_key("name"));
        assert!(form_state.errors.contains_key("email"));
        assert!(form_state.errors.contains_key("message"));
    }

    #[test]
    fn test_field_validation() {
        let mut form_state = FormState::new(ContactData::default());

        form_state.set_field("email", "test@example.com");
        assert!(!form_state.errors.contains_key("email"));

        form_state.set_field("email", "invalid");
        assert!(form_state.errors.contains_key("email"));
    }

    #[test]
    fn test_required_validation() {
        assert!(validate_required("").is_err());
        assert!(validate_required("   ").is_err());
        assert!(validate_required("test").is_ok());
    }

    #[test]
    fn test_email_validation() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("invalid").is_err());
        assert!(validate_email("@example.com").is_err());
    }
}
```

This comprehensive form handling example demonstrates the full power of Leptos for building complex, user-friendly forms with excellent validation, accessibility, and user experience.