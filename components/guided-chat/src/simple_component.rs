use leptos::prelude::*;
use leptos::ev::SubmitEvent;
use crate::types::*;
use std::collections::HashMap;
use wasm_bindgen::JsCast;

/// Simplified guided chat component that works with Leptos 0.8
#[component]
pub fn SimpleGuidedChat() -> impl IntoView {
    // State
    let (messages, set_messages) = signal::<Vec<ChatMessage>>(vec![]);
    let (input_value, set_input_value) = signal(String::new());
    let (current_step, set_current_step) = signal(0usize);
    let (collected_data, set_collected_data) = signal::<HashMap<String, String>>(HashMap::new());
    let (is_complete, set_is_complete) = signal(false);
    
    // Sample workflow steps
    let steps = &[
        ("client", "Who is the client?"),
        ("amount", "What's the amount?"),
        ("payment_method", "Payment method?"),
    ];
    
    // Add initial message
    Effect::new(move |_| {
        set_messages.update(|msgs| {
            msgs.push(ChatMessage {
                id: "1".to_string(),
                message_type: MessageType::Assistant,
                content: "Welcome! Let's create an invoice.".to_string(),
                timestamp: "".to_string(),
                metadata: None,
                actions: vec![],
                step_id: None,
            });
            msgs.push(ChatMessage {
                id: "2".to_string(),
                message_type: MessageType::Assistant,
                content: steps[0].1.to_string(),
                timestamp: "".to_string(),
                metadata: None,
                actions: vec![],
                step_id: Some(steps[0].0.to_string()),
            });
        });
    });
    
    // Handle form submission
    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        
        let value = input_value.get();
        if value.trim().is_empty() {
            return;
        }
        
        let step_idx = current_step.get();
        
        // Add user message
        set_messages.update(|msgs| {
            msgs.push(ChatMessage {
                id: format!("user-{}", msgs.len()),
                message_type: MessageType::User,
                content: value.clone(),
                timestamp: "".to_string(),
                metadata: None,
                actions: vec![],
                step_id: if step_idx < steps.len() {
                    Some(steps[step_idx].0.to_string())
                } else {
                    None
                },
            });
        });
        
        // Store collected data
        if step_idx < steps.len() {
            set_collected_data.update(|data| {
                data.insert(steps[step_idx].0.to_string(), value.clone());
            });
        }
        
        // Clear input
        set_input_value.set(String::new());
        
        // Move to next step
        if step_idx < steps.len() - 1 {
            let next_idx = step_idx + 1;
            set_current_step.set(next_idx);
            
            // Add assistant message for next step
            set_messages.update(|msgs| {
                msgs.push(ChatMessage {
                    id: format!("assistant-{}", msgs.len()),
                    message_type: MessageType::Assistant,
                    content: steps[next_idx].1.to_string(),
                    timestamp: "".to_string(),
                    metadata: None,
                    actions: vec![],
                    step_id: Some(steps[next_idx].0.to_string()),
                });
            });
        } else {
            // Workflow complete
            set_is_complete.set(true);
            set_messages.update(|msgs| {
                msgs.push(ChatMessage {
                    id: format!("complete-{}", msgs.len()),
                    message_type: MessageType::Success,
                    content: "Workflow complete! Data collected.".to_string(),
                    timestamp: "".to_string(),
                    metadata: None,
                    actions: vec![],
                    step_id: None,
                });
            });
            
            // Log collected data
            web_sys::console::log_1(&format!("Collected data: {:?}", collected_data.get()).into());
        }
    };
    
    // Calculate progress
    let progress = move || {
        let data = collected_data.get();
        let completed_count = data.len();
        if steps.is_empty() {
            100
        } else if completed_count >= steps.len() {
            100
        } else {
            ((completed_count as f32 / steps.len() as f32) * 100.0) as u32
        }
    };
    
    view! {
        <div class="gc-container">
            // Progress bar
            <div class="gc-progress">
                <div class="gc-progress-bar">
                    <div 
                        class="gc-progress-fill"
                        style=move || format!("width: {}%", progress())
                    ></div>
                </div>
                <div class="gc-progress-text">
                    {move || {
                        let data = collected_data.get();
                        let completed = data.len();
                        if completed >= steps.len() {
                            format!("Completed! ({} of {})", steps.len(), steps.len())
                        } else {
                            format!("Step {} of {}", completed + 1, steps.len())
                        }
                    }}
                </div>
            </div>
            
            // Messages
            <div class="gc-messages">
                <For
                    each=move || messages.get()
                    key=|msg| msg.id.clone()
                    children=move |msg| {
                        let class = match msg.message_type {
                            MessageType::User => "gc-message-user",
                            MessageType::Assistant => "gc-message-assistant",
                            MessageType::System => "gc-message-system",
                            MessageType::Error => "gc-message-error",
                            MessageType::Success => "gc-message-success",
                        };
                        
                        view! {
                            <div class=format!("gc-message {}", class)>
                                <div class="gc-message-content">
                                    {msg.content.clone()}
                                </div>
                            </div>
                        }
                    }
                />
            </div>
            
            // Input area - only show if not complete
            <Show
                when=move || !is_complete.get()
                fallback=|| view! { <div></div> }
            >
                <div class="gc-input-area">
                    <form on:submit=handle_submit class="gc-input-form">
                        <input
                            type="text"
                            class="gc-input"
                            placeholder="Type your answer..."
                            prop:value=move || input_value.get()
                            on:input=move |ev| {
                                let target = ev.target().unwrap();
                                let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                set_input_value.set(input.value());
                            }
                        />
                        <button
                            type="submit"
                            class="gc-submit"
                            disabled=move || input_value.get().trim().is_empty()
                        >
                            "Send"
                        </button>
                    </form>
                </div>
            </Show>
        </div>
    }
}