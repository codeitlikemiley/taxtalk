use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use guided_chat::*;
use std::collections::HashMap;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    // State for demo
    let (selected_workflow, set_selected_workflow) = signal(String::from("invoice"));
    let (completed_data, set_completed_data) = signal::<Option<HashMap<String, serde_json::Value>>>(None);
    let (show_chat, set_show_chat) = signal(true);
    
    // Get workflow based on selection
    let get_workflow = move || {
        match selected_workflow.get().as_str() {
            "invoice" => Workflow::invoice_workflow(),
            "payment" => Workflow::payment_workflow(),
            _ => Workflow::invoice_workflow(),
        }
    };
    
    // Handle workflow completion
    let handle_complete = move |data: HashMap<String, serde_json::Value>| {
        set_completed_data.set(Some(data));
        set_show_chat.set(false);
    };
    
    // Reset chat
    let reset_chat = move |_| {
        set_completed_data.set(None);
        set_show_chat.set(true);
    };
    
    view! {
        <div style="min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); padding: 2rem;">
            <style>{include_str!("styles.css")}</style>
            
            <div style="max-width: 1200px; margin: 0 auto;">
                <div style="text-align: center; color: white; margin-bottom: 3rem;">
                    <h1 style="font-size: 3rem; font-weight: bold; margin-bottom: 1rem;">
                        "Guided Chat Demo"
                    </h1>
                    <p style="font-size: 1.25rem; opacity: 0.9;">
                        "Step-by-step workflows for TaxTalk"
                    </p>
                </div>
                
                <div style="display: grid; grid-template-columns: 300px 1fr; gap: 2rem; align-items: start;">
                    // Workflow selector
                    <div style="background: white; padding: 1.5rem; border-radius: 1rem; box-shadow: 0 20px 40px rgba(0,0,0,0.1);">
                        <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #1f2937;">
                            "Select Workflow"
                        </h2>
                        
                        <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                            <button
                                style=move || format!(
                                    "padding: 0.75rem; border: 2px solid {}; background: {}; border-radius: 0.5rem; text-align: left; cursor: pointer; transition: all 0.2s;",
                                    if selected_workflow.get() == "invoice" { "#3b82f6" } else { "#e5e7eb" },
                                    if selected_workflow.get() == "invoice" { "#eff6ff" } else { "white" }
                                )
                                on:click=move |_| {
                                    set_selected_workflow.set("invoice".to_string());
                                    reset_chat(());
                                }
                            >
                                <div style="font-weight: 600; color: #1f2937;">
                                    "Create Invoice"
                                </div>
                                <div style="font-size: 0.875rem; color: #6b7280; margin-top: 0.25rem;">
                                    "Step-by-step invoice creation"
                                </div>
                            </button>
                            
                            <button
                                style=move || format!(
                                    "padding: 0.75rem; border: 2px solid {}; background: {}; border-radius: 0.5rem; text-align: left; cursor: pointer; transition: all 0.2s;",
                                    if selected_workflow.get() == "payment" { "#3b82f6" } else { "#e5e7eb" },
                                    if selected_workflow.get() == "payment" { "#eff6ff" } else { "white" }
                                )
                                on:click=move |_| {
                                    set_selected_workflow.set("payment".to_string());
                                    reset_chat(());
                                }
                            >
                                <div style="font-weight: 600; color: #1f2937;">
                                    "Record Payment"
                                </div>
                                <div style="font-size: 0.875rem; color: #6b7280; margin-top: 0.25rem;">
                                    "Record payment received"
                                </div>
                            </button>
                        </div>
                        
                        <div style="margin-top: 2rem; padding-top: 2rem; border-top: 1px solid #e5e7eb;">
                            <h3 style="font-size: 1rem; font-weight: 600; margin-bottom: 0.5rem; color: #1f2937;">
                                "Features"
                            </h3>
                            <ul style="font-size: 0.875rem; color: #6b7280; list-style: none; padding: 0;">
                                <li style="margin-bottom: 0.5rem;">
                                    "✓ Step-by-step guidance"
                                </li>
                                <li style="margin-bottom: 0.5rem;">
                                    "✓ Input validation"
                                </li>
                                <li style="margin-bottom: 0.5rem;">
                                    "✓ Quick replies"
                                </li>
                                <li style="margin-bottom: 0.5rem;">
                                    "✓ Progress tracking"
                                </li>
                                <li style="margin-bottom: 0.5rem;">
                                    "✓ Back navigation"
                                </li>
                                <li style="margin-bottom: 0.5rem;">
                                    "✓ Philippine tax rules"
                                </li>
                            </ul>
                        </div>
                    </div>
                    
                    // Chat area
                    <div style="background: white; border-radius: 1rem; box-shadow: 0 20px 40px rgba(0,0,0,0.1); overflow: hidden; height: 600px; display: flex; flex-direction: column;">
                        <Show
                            when=move || show_chat.get()
                            fallback=move || view! {
                                <div style="padding: 2rem; text-align: center;">
                                    <div style="margin-bottom: 2rem;">
                                        <h2 style="font-size: 1.5rem; font-weight: 600; color: #1f2937; margin-bottom: 1rem;">
                                            "Workflow Complete!"
                                        </h2>
                                        
                                        <Show when=move || completed_data.get().is_some()>
                                            <div style="background: #f9fafb; padding: 1.5rem; border-radius: 0.5rem; text-align: left; margin-top: 1rem;">
                                                <h3 style="font-weight: 600; margin-bottom: 1rem; color: #1f2937;">
                                                    "Collected Data:"
                                                </h3>
                                                <pre style="font-family: monospace; font-size: 0.875rem; color: #4b5563; white-space: pre-wrap; word-wrap: break-word;">
                                                    {move || {
                                                        if let Some(data) = completed_data.get() {
                                                            serde_json::to_string_pretty(&data).unwrap_or_default()
                                                        } else {
                                                            String::new()
                                                        }
                                                    }}
                                                </pre>
                                            </div>
                                        </Show>
                                    </div>
                                    
                                    <button
                                        style="padding: 0.75rem 2rem; background: #3b82f6; color: white; border: none; border-radius: 0.5rem; font-weight: 500; cursor: pointer;"
                                        on:click=reset_chat
                                    >
                                        "Start New Workflow"
                                    </button>
                                </div>
                            }
                        >
                            <GuidedChat
                                workflow=get_workflow()
                                config=GuidedChatConfig {
                                    placeholder: "Type your answer...".to_string(),
                                    welcome_message: Some("Welcome! I'll guide you through this process step by step.".to_string()),
                                    completion_message: Some("Perfect! I've collected all the information needed.".to_string()),
                                    show_timestamps: true,
                                    show_step_progress: true,
                                    allow_back_navigation: true,
                                    auto_scroll: true,
                                    enable_markdown: false,
                                    max_messages: None,
                                    typing_indicator_delay: 300,
                                }
                                on_complete=handle_complete
                            />
                        </Show>
                    </div>
                </div>
                
                // Info section
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem; margin-top: 3rem;">
                    <div style="background: white; padding: 1.5rem; border-radius: 0.75rem; box-shadow: 0 10px 20px rgba(0,0,0,0.05);">
                        <h3 style="color: #1e40af; font-weight: 600; margin-bottom: 0.5rem;">
                            "🚀 Guided Workflows"
                        </h3>
                        <p style="color: #6b7280; font-size: 0.875rem;">
                            "Step-by-step guidance through complex processes"
                        </p>
                    </div>
                    
                    <div style="background: white; padding: 1.5rem; border-radius: 0.75rem; box-shadow: 0 10px 20px rgba(0,0,0,0.05);">
                        <h3 style="color: #14532d; font-weight: 600; margin-bottom: 0.5rem;">
                            "✅ Smart Validation"
                        </h3>
                        <p style="color: #6b7280; font-size: 0.875rem;">
                            "Real-time input validation with helpful feedback"
                        </p>
                    </div>
                    
                    <div style="background: white; padding: 1.5rem; border-radius: 0.75rem; box-shadow: 0 10px 20px rgba(0,0,0,0.05);">
                        <h3 style="color: #78350f; font-weight: 600; margin-bottom: 0.5rem;">
                            "🇵🇭 Philippine Rules"
                        </h3>
                        <p style="color: #6b7280; font-size: 0.875rem;">
                            "Built-in VAT, EWT, and BIR compliance"
                        </p>
                    </div>
                    
                    <div style="background: white; padding: 1.5rem; border-radius: 0.75rem; box-shadow: 0 10px 20px rgba(0,0,0,0.05);">
                        <h3 style="color: #831843; font-weight: 600; margin-bottom: 0.5rem;">
                            "⚡ Quick Actions"
                        </h3>
                        <p style="color: #6b7280; font-size: 0.875rem;">
                            "Quick reply buttons for common responses"
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}