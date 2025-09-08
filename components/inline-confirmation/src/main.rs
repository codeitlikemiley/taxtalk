use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use inline_confirmation::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    // Demo states
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (submission_result, set_submission_result) = signal::<Option<String>>(None);
    
    view! {
        <div style="min-height: 100vh; padding: 2rem; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
            <style>{include_str!("styles.css")}</style>
            
            <div style="max-width: 800px; margin: 0 auto;">
                <div style="text-align: center; color: white; margin-bottom: 3rem;">
                    <h1 style="font-size: 3rem; font-weight: bold; margin-bottom: 1rem;">
                        "Inline Confirmation Demo"
                    </h1>
                    <p style="font-size: 1.25rem; opacity: 0.9;">
                        "Real-time validation with inline feedback"
                    </p>
                </div>
                
                // Simple demo form
                <div style="background: white; padding: 2rem; border-radius: 1rem; box-shadow: 0 20px 40px rgba(0,0,0,0.1); margin-bottom: 2rem;">
                    <h2 style="font-size: 1.5rem; font-weight: 600; margin-bottom: 2rem; color: #1f2937;">
                        "Simple Validation Demo"
                    </h2>
                    
                    <div style="display: grid; gap: 1.5rem;">
                        // Basic email input with validation
                        <div>
                            <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: #374151;">
                                "Email Address"
                            </label>
                            <input
                                type="email"
                                style="width: 100%; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 0.375rem;"
                                placeholder="your@email.com"
                                prop:value=move || email.get()
                                on:input=move |ev| {
                                    let target = ev.target().unwrap();
                                    let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                    set_email.set(input.value());
                                }
                            />
                            {move || {
                                let val = email.get();
                                if val.is_empty() {
                                    None
                                } else if !val.contains('@') {
                                    Some(view! {
                                        <div style="color: #ef4444; font-size: 0.875rem; margin-top: 0.25rem;">
                                            "Please enter a valid email address"
                                        </div>
                                    })
                                } else {
                                    Some(view! {
                                        <div style="color: #10b981; font-size: 0.875rem; margin-top: 0.25rem;">
                                            "✓ Valid email format"
                                        </div>
                                    })
                                }
                            }}
                        </div>
                        
                        // Password with strength indicator
                        <PasswordInput
                            value=password
                            set_value=set_password
                            show_strength=true
                        />
                    </div>
                    
                    <div style="margin-top: 2rem; padding-top: 2rem; border-top: 1px solid #e5e7eb;">
                        <button
                            style="padding: 0.75rem 2rem; background: #3b82f6; color: white; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer;"
                            on:click=move |_| {
                                set_submission_result.set(Some(format!(
                                    "Form submitted with email: {}",
                                    email.get()
                                )));
                            }
                        >
                            "Submit"
                        </button>
                        
                        <Show when=move || submission_result.get().is_some()>
                            <div style="margin-top: 1rem; padding: 1rem; background: #f0f9ff; border-left: 4px solid #3b82f6; border-radius: 0.25rem;">
                                <p style="color: #1e40af; margin: 0;">
                                    {move || submission_result.get().unwrap_or_default()}
                                </p>
                            </div>
                        </Show>
                    </div>
                </div>
                
                // Component features
                <div style="background: white; padding: 2rem; border-radius: 1rem; box-shadow: 0 20px 40px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.5rem; font-weight: 600; margin-bottom: 2rem; color: #1f2937;">
                        "Component Library Features"
                    </h2>
                    
                    <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem;">
                        <div style="padding: 1rem; background: #f0f9ff; border-radius: 0.5rem;">
                            <h3 style="color: #1e40af; font-weight: 600; margin-bottom: 0.5rem;">
                                "✓ Real-time Validation"
                            </h3>
                            <p style="color: #3b82f6; font-size: 0.875rem;">
                                "Instant feedback as you type"
                            </p>
                        </div>
                        
                        <div style="padding: 1rem; background: #f0fdf4; border-radius: 0.5rem;">
                            <h3 style="color: #14532d; font-weight: 600; margin-bottom: 0.5rem;">
                                "🔒 Password Strength"
                            </h3>
                            <p style="color: #16a34a; font-size: 0.875rem;">
                                "Visual strength indicator"
                            </p>
                        </div>
                        
                        <div style="padding: 1rem; background: #fef3c7; border-radius: 0.5rem;">
                            <h3 style="color: #78350f; font-weight: 600; margin-bottom: 0.5rem;">
                                "🇵🇭 Philippine Formats"
                            </h3>
                            <p style="color: #f59e0b; font-size: 0.875rem;">
                                "TIN, phone, currency validation"
                            </p>
                        </div>
                        
                        <div style="padding: 1rem; background: #fce7f3; border-radius: 0.5rem;">
                            <h3 style="color: #831843; font-weight: 600; margin-bottom: 0.5rem;">
                                "✨ Animations"
                            </h3>
                            <p style="color: #ec4899; font-size: 0.875rem;">
                                "Smooth transitions and feedback"
                            </p>
                        </div>
                    </div>
                    
                    <div style="margin-top: 2rem; padding: 1rem; background: #f9fafb; border-radius: 0.5rem;">
                        <p style="color: #6b7280; font-size: 0.875rem; margin: 0;">
                            <strong>"Note:"</strong> " The full InlineConfirmation component with all validation rules is available in the library. This demo shows a simplified version due to Leptos 0.8 view! macro limitations with generic type parameters."
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}