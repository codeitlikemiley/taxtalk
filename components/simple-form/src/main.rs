use leptos::prelude::*;
use simple_form::{SimpleForm, FormConfig, FormData, FieldConfig, FieldType, SelectOption, ValidationRule};
use simple_form::ph_forms;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (show_basic_form, set_show_basic_form) = signal(false);
    let (show_client_form, set_show_client_form) = signal(false);
    let (show_invoice_form, set_show_invoice_form) = signal(false);
    let (show_payment_form, set_show_payment_form) = signal(false);
    let (submitted_data, set_submitted_data) = signal::<Option<FormData>>(None);
    let (event_log, set_event_log) = signal(Vec::<String>::new());
    
    let log_event = move |msg: String| {
        set_event_log.update(|log| {
            log.push(format!("{}: {}", chrono::Local::now().format("%H:%M:%S"), msg));
            if log.len() > 10 {
                log.remove(0);
            }
        });
    };
    
    // Basic form configuration
    let basic_form_config = FormConfig::new()
        .title("Basic Information")
        .description("Please fill in your basic information")
        .field(
            FieldConfig::new("name", "Full Name", FieldType::Text)
                .required()
                .placeholder("John Doe")
                .validation(ValidationRule::MinLength { min: 3 })
                .validation(ValidationRule::MaxLength { max: 50 })
        )
        .field(
            FieldConfig::new("email", "Email Address", FieldType::Email)
                .required()
                .placeholder("john@example.com")
                .help_text("We'll never share your email")
        )
        .field(
            FieldConfig::new("age", "Age", FieldType::Integer)
                .required()
                .min("18")
                .max("120")
                .validation(ValidationRule::Integer { min: Some(18), max: Some(120) })
        )
        .field(
            FieldConfig::new("country", "Country", FieldType::Select)
                .required()
                .options(vec![
                    SelectOption::new("", "Select a country"),
                    SelectOption::new("PH", "Philippines"),
                    SelectOption::new("US", "United States"),
                    SelectOption::new("UK", "United Kingdom"),
                    SelectOption::new("JP", "Japan"),
                ])
        )
        .field(
            FieldConfig::new("bio", "Bio", FieldType::Textarea)
                .placeholder("Tell us about yourself...")
                .rows(3)
                .validation(ValidationRule::MaxLength { max: 500 })
        )
        .field(
            FieldConfig::new("agree", "I agree to the terms", FieldType::Boolean)
                .required()
                .placeholder("I agree to the terms and conditions")
        );
    
    let handle_form_submit = move |data: FormData| {
        log_event(format!("Form submitted with {} fields", data.values.len()));
        set_submitted_data.set(Some(data));
        set_show_basic_form.set(false);
        set_show_client_form.set(false);
        set_show_invoice_form.set(false);
        set_show_payment_form.set(false);
    };
    
    let handle_form_cancel = move || {
        log_event("Form cancelled".to_string());
        set_show_basic_form.set(false);
        set_show_client_form.set(false);
        set_show_invoice_form.set(false);
        set_show_payment_form.set(false);
    };
    
    view! {
        <div class="demo-container">
            <style>{include_str!("styles.css")}</style>
            <style>{r#"
                .demo-container {
                    max-width: 1200px;
                    margin: 0 auto;
                    padding: 2rem;
                    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
                    min-height: 100vh;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                }
                
                h1 {
                    font-size: 2.5rem;
                    font-weight: 700;
                    margin-bottom: 0.5rem;
                    color: white;
                    text-shadow: 0 2px 4px rgba(0,0,0,0.1);
                }
                
                .subtitle {
                    color: rgba(255, 255, 255, 0.9);
                    margin-bottom: 2rem;
                    font-size: 1.125rem;
                }
                
                .demo-section {
                    margin-bottom: 2rem;
                    background: white;
                    padding: 1.5rem;
                    border-radius: 0.75rem;
                    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
                }
                
                .demo-section h2 {
                    font-size: 1.5rem;
                    font-weight: 600;
                    margin-bottom: 1rem;
                    color: #111827;
                }
                
                .form-buttons {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
                    gap: 1rem;
                }
                
                .demo-btn {
                    padding: 0.75rem 1.5rem;
                    font-size: 0.875rem;
                    font-weight: 500;
                    color: white;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    border: none;
                    border-radius: 0.5rem;
                    cursor: pointer;
                    transition: all 0.2s;
                    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
                }
                
                .demo-btn:hover {
                    transform: translateY(-2px);
                    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.2);
                }
                
                .submitted-data {
                    padding: 1rem;
                    background: #f0f9ff;
                    border: 1px solid #bfdbfe;
                    border-radius: 0.5rem;
                    margin-top: 1rem;
                }
                
                .submitted-data h3 {
                    font-size: 1.125rem;
                    font-weight: 600;
                    color: #1e40af;
                    margin-bottom: 0.5rem;
                }
                
                .data-grid {
                    display: grid;
                    grid-template-columns: auto 1fr;
                    gap: 0.5rem 1rem;
                    font-size: 0.875rem;
                }
                
                .data-key {
                    font-weight: 600;
                    color: #374151;
                }
                
                .data-value {
                    color: #4b5563;
                    font-family: monospace;
                    word-break: break-all;
                }
                
                .event-log {
                    background: #1e1e1e;
                    color: #d4d4d4;
                    padding: 1rem;
                    border-radius: 0.5rem;
                    font-family: monospace;
                    font-size: 0.875rem;
                    max-height: 200px;
                    overflow-y: auto;
                }
                
                .log-entry {
                    padding: 0.125rem 0;
                }
                
                .features {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
                    gap: 1rem;
                    margin-top: 1rem;
                }
                
                .feature {
                    padding: 1rem;
                    background: linear-gradient(135deg, #f0f9ff 0%, #e0e7ff 100%);
                    border-radius: 0.5rem;
                    border: 1px solid #c7d2fe;
                }
                
                .feature-title {
                    font-weight: 600;
                    color: #4338ca;
                    margin-bottom: 0.25rem;
                }
                
                .feature-desc {
                    font-size: 0.875rem;
                    color: #4b5563;
                }
            "#}</style>
            
            <h1>"SimpleForm Component Demo"</h1>
            <p class="subtitle">"Flexible form builder with comprehensive validation framework"</p>
            
            <div class="demo-section">
                <h2>"Try Different Forms"</h2>
                <div class="form-buttons">
                    <button
                        class="demo-btn"
                        on:click=move |_| {
                            set_show_basic_form.set(true);
                            log_event("Opening basic form".to_string());
                        }
                    >
                        "Basic Form"
                    </button>
                    <button
                        class="demo-btn"
                        on:click=move |_| {
                            set_show_client_form.set(true);
                            log_event("Opening client form".to_string());
                        }
                    >
                        "Client Registration"
                    </button>
                    <button
                        class="demo-btn"
                        on:click=move |_| {
                            set_show_invoice_form.set(true);
                            log_event("Opening invoice form".to_string());
                        }
                    >
                        "Create Invoice"
                    </button>
                    <button
                        class="demo-btn"
                        on:click=move |_| {
                            set_show_payment_form.set(true);
                            log_event("Opening payment form".to_string());
                        }
                    >
                        "Record Payment"
                    </button>
                </div>
                
                <Show when=move || submitted_data.get().is_some()>
                    {move || {
                        let data = submitted_data.get().unwrap();
                        view! {
                            <div class="submitted-data">
                                <h3>"Form Submitted Successfully!"</h3>
                                <div class="data-grid">
                                    <For
                                        each=move || data.values.clone().into_iter()
                                        key=|(k, _)| k.clone()
                                        children=move |(key, value)| {
                                            view! {
                                                <>
                                                    <div class="data-key">{key}":"</div>
                                                    <div class="data-value">{value}</div>
                                                </>
                                            }
                                        }
                                    />
                                </div>
                            </div>
                        }
                    }}
                </Show>
            </div>
            
            // Basic Form
            <SimpleForm
                config=basic_form_config
                show=show_basic_form.get()
                on_submit=Callback::new(handle_form_submit)
                on_cancel=Some(Callback::new(handle_form_cancel))
            />
            
            // Client Form
            <SimpleForm
                config=ph_forms::client_form()
                show=show_client_form.get()
                on_submit=Callback::new(handle_form_submit)
                on_cancel=Some(Callback::new(handle_form_cancel))
            />
            
            // Invoice Form
            <SimpleForm
                config=ph_forms::invoice_form()
                show=show_invoice_form.get()
                on_submit=Callback::new(handle_form_submit)
                on_cancel=Some(Callback::new(handle_form_cancel))
            />
            
            // Payment Form
            <SimpleForm
                config=ph_forms::payment_form()
                show=show_payment_form.get()
                on_submit=Callback::new(handle_form_submit)
                on_cancel=Some(Callback::new(handle_form_cancel))
            />
            
            <div class="demo-section">
                <h2>"Features"</h2>
                <div class="features">
                    <div class="feature">
                        <div class="feature-title">"✅ Comprehensive Validation"</div>
                        <div class="feature-desc">"Built-in validators for email, phone, TIN, currency, and more"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"🎨 Multiple Field Types"</div>
                        <div class="feature-desc">"Text, number, date, select, textarea, boolean, and more"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"🇵🇭 Philippine Context"</div>
                        <div class="feature-desc">"Pre-built forms for clients, invoices, payments with local validation"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"⚡ Real-time Validation"</div>
                        <div class="feature-desc">"Validate on blur, change, or submit with customizable behavior"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"🔧 Flexible Configuration"</div>
                        <div class="feature-desc">"Declarative form building with extensive customization options"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"♿ Accessible"</div>
                        <div class="feature-desc">"Proper labels, ARIA attributes, and keyboard navigation"</div>
                    </div>
                </div>
            </div>
            
            <div class="demo-section">
                <h2>"Event Log"</h2>
                <div class="event-log">
                    <Show
                        when=move || !event_log.get().is_empty()
                        fallback=|| view! {
                            <div style="color: #6b7280;">"No events yet..."</div>
                        }
                    >
                        <For
                            each=move || event_log.get()
                            key=|entry| entry.clone()
                            children=move |entry| {
                                view! {
                                    <div class="log-entry">{entry}</div>
                                }
                            }
                        />
                    </Show>
                </div>
            </div>
        </div>
    }
}