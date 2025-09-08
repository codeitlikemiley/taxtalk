use leptos::prelude::*;
use leptos::ev::{KeyboardEvent, SubmitEvent};
use crate::types::*;
use crate::step_manager::StepManager;
use crate::message_parser::MessageParser;
use gloo_timers::future::TimeoutFuture;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use std::collections::HashMap;
use chrono;

/// Guided chat component for step-by-step workflows
#[component]
pub fn GuidedChat<S, M>(
    /// Workflow definition
    workflow: Workflow,
    /// Configuration
    #[prop(optional)] config: Option<GuidedChatConfig>,
    /// On workflow complete callback
    #[prop(optional)] on_complete: Option<S>,
    /// On message sent callback
    #[prop(optional)] on_message: Option<M>,
    /// CSS class
    #[prop(optional)] class: Option<String>,
) -> impl IntoView
where
    S: Fn(HashMap<String, serde_json::Value>) + Clone + 'static,
    M: Fn(ChatMessage) + Clone + 'static,
{
    let config = config.unwrap_or_default();
    
    // State
    let (messages, set_messages) = signal::<Vec<ChatMessage>>(vec![]);
    let (input_value, set_input_value) = signal(String::new());
    let (is_typing, set_is_typing) = signal(false);
    let (workflow_state, set_workflow_state) = signal(WorkflowState {
        workflow_id: workflow.id.clone(),
        current_step: None,
        completed_steps: vec![],
        collected_data: HashMap::new(),
        is_complete: false,
        error: None,
    });
    
    // Initialize step manager
    let step_manager = StepManager::new(workflow.clone());
    let message_parser = MessageParser::new();
    
    // Add welcome message on mount
    Effect::new({
        let config = config.clone();
        let workflow = workflow.clone();
        move |_| {
            if let Some(welcome) = &config.welcome_message {
                add_assistant_message(
                    welcome.clone(),
                    &set_messages,
                    None,
                );
            }
            
            // Start first step
            if let Some(first_step) = workflow.steps.first() {
                add_assistant_message(
                    first_step.prompt.clone(),
                    &set_messages,
                    Some(first_step.id.clone()),
                );
                
                set_workflow_state.update(|state| {
                    state.current_step = Some(first_step.id.clone());
                });
            }
        }
    });
    
    // Handle user input submission
    let handle_submit = {
        let step_manager = step_manager.clone();
        let config = config.clone();
        let on_message = on_message.clone();
        
        move |ev: SubmitEvent| {
            ev.prevent_default();
            
            let value = input_value.get();
            if value.trim().is_empty() {
                return;
            }
            
            // Add user message
            let user_msg = ChatMessage {
                id: generate_id(),
                message_type: MessageType::User,
                content: value.clone(),
                timestamp: get_timestamp(),
                metadata: None,
                actions: vec![],
                step_id: workflow_state.get().current_step.clone(),
            };
            
            set_messages.update(|msgs| msgs.push(user_msg.clone()));
            set_input_value.set(String::new());
            
            if let Some(ref on_msg) = on_message {
                on_msg(user_msg);
            }
            
            // Show typing indicator
            set_is_typing.set(true);
            
            // Process the input
            let step_manager = step_manager.clone();
            let config = config.clone();
            let on_complete = on_complete.clone();
            spawn_local(async move {
                // Simulate processing delay
                TimeoutFuture::new(config.typing_indicator_delay).await;
                set_is_typing.set(false);
                
                // Get current step
                let state = workflow_state.get();
                if let Some(current_step_id) = &state.current_step {
                    if let Some(current_step) = step_manager.get_step(current_step_id) {
                        // Validate input
                        if let Some(validation) = &current_step.validation {
                            if let Err(error) = validate_input(&value, &current_step.input_type, validation) {
                                add_error_message(
                                    error,
                                    &set_messages,
                                    Some(current_step_id.clone()),
                                );
                                return;
                            }
                        }
                        
                        // Store the collected data
                        set_workflow_state.update(|state| {
                            state.collected_data.insert(
                                current_step_id.clone(),
                                serde_json::json!(value),
                            );
                            state.completed_steps.push(current_step_id.clone());
                        });
                        
                        // Move to next step
                        if let Some(next_step) = step_manager.get_next_step(current_step_id, &workflow_state.get()) {
                            add_assistant_message(
                                next_step.prompt.clone(),
                                &set_messages,
                                Some(next_step.id.clone()),
                            );
                            
                            set_workflow_state.update(|state| {
                                state.current_step = Some(next_step.id.clone());
                            });
                        } else {
                            // Workflow complete
                            set_workflow_state.update(|state| {
                                state.is_complete = true;
                                state.current_step = None;
                            });
                            
                            if let Some(completion_msg) = &config.completion_message {
                                add_success_message(
                                    completion_msg.clone(),
                                    &set_messages,
                                    None,
                                );
                            }
                            
                            // Call completion callback
                            if let Some(ref on_complete_fn) = on_complete {
                                on_complete_fn(workflow_state.get().collected_data.clone());
                            }
                        }
                    }
                }
            });
        }
    };
    
    // Handle quick reply clicks
    let handle_quick_reply = move |value: String| {
        set_input_value.set(value);
        // Auto-submit could be added here if desired
    };
    
    // Handle action button clicks
    let handle_action = move |action: MessageAction| {
        match action.action_type {
            ActionType::Back if config.allow_back_navigation => {
                // Go back to previous step
                set_workflow_state.update(|state| {
                    if let Some(last_completed) = state.completed_steps.pop() {
                        state.current_step = Some(last_completed);
                        // Re-show the prompt for that step
                        if let Some(step) = step_manager.get_step(&state.current_step.as_ref().unwrap()) {
                            add_assistant_message(
                                step.prompt.clone(),
                                &set_messages,
                                Some(step.id.clone()),
                            );
                        }
                    }
                });
            }
            ActionType::Skip => {
                // Skip optional step
                let state = workflow_state.get();
                if let Some(current_step_id) = &state.current_step {
                    if let Some(current_step) = step_manager.get_step(current_step_id) {
                        if !current_step.required {
                            // Move to next step without collecting data
                            if let Some(next_step) = step_manager.get_next_step(current_step_id, &state) {
                                add_assistant_message(
                                    next_step.prompt.clone(),
                                    &set_messages,
                                    Some(next_step.id.clone()),
                                );
                                
                                set_workflow_state.update(|state| {
                                    state.current_step = Some(next_step.id.clone());
                                    state.completed_steps.push(current_step_id.clone());
                                });
                            }
                        }
                    }
                }
            }
            ActionType::QuickReply(value) => {
                handle_quick_reply(value);
            }
            _ => {}
        }
    };
    
    // Get progress percentage
    let progress_percentage = move || {
        let state = workflow_state.get();
        let total_steps = workflow.steps.len();
        let completed = state.completed_steps.len();
        if total_steps > 0 {
            (completed as f32 / total_steps as f32 * 100.0) as u32
        } else {
            0
        }
    };
    
    // Get current step's quick replies
    let current_quick_replies = move || {
        let state = workflow_state.get();
        if let Some(current_step_id) = &state.current_step {
            if let Some(step) = step_manager.get_step(current_step_id) {
                return step.quick_replies.clone();
            }
        }
        vec![]
    };
    
    view! {
        <div class=format!("gc-container {}", class.unwrap_or_default())>
            {if config.show_step_progress {
                Some(view! {
                    <div class="gc-progress">
                        <div class="gc-progress-bar">
                            <div 
                                class="gc-progress-fill"
                                style=move || format!("width: {}%", progress_percentage())
                            ></div>
                        </div>
                        <div class="gc-progress-text">
                            {move || {
                                let state = workflow_state.get();
                                format!("Step {} of {}", 
                                    state.completed_steps.len() + 1,
                                    workflow.steps.len()
                                )
                            }}
                        </div>
                    </div>
                })
            } else {
                None
            }}
            
            <div class="gc-messages" id="gc-messages-container">
                <For
                    each=move || messages.get()
                    key=|msg| msg.id.clone()
                    children=move |msg| {
                        view! {
                            <div class=format!("gc-message gc-message-{}", 
                                match msg.message_type {
                                    MessageType::User => "user",
                                    MessageType::Assistant => "assistant",
                                    MessageType::System => "system",
                                    MessageType::Error => "error",
                                    MessageType::Success => "success",
                                }
                            )>
                                <div class="gc-message-content">
                                    {if config.enable_markdown {
                                        // Could use a markdown parser here
                                        msg.content.clone()
                                    } else {
                                        msg.content.clone()
                                    }}
                                </div>
                                
                                {if !msg.actions.is_empty() {
                                    Some(view! {
                                        <div class="gc-message-actions">
                                            <For
                                                each=move || msg.actions.clone()
                                                key=|action| action.id.clone()
                                                children=move |action| {
                                                    let action_clone = action.clone();
                                                    view! {
                                                        <button
                                                            class=format!("gc-action gc-action-{}", 
                                                                match action.style {
                                                                    ActionStyle::Primary => "primary",
                                                                    ActionStyle::Secondary => "secondary",
                                                                    ActionStyle::Success => "success",
                                                                    ActionStyle::Danger => "danger",
                                                                    ActionStyle::Link => "link",
                                                                }
                                                            )
                                                            on:click=move |_| handle_action(action_clone.clone())
                                                        >
                                                            {action.label}
                                                        </button>
                                                    }
                                                }
                                            />
                                        </div>
                                    })
                                } else {
                                    None
                                }}
                                
                                {if config.show_timestamps {
                                    Some(view! {
                                        <div class="gc-message-timestamp">
                                            {msg.timestamp}
                                        </div>
                                    })
                                } else {
                                    None
                                }}
                            </div>
                        }
                    }
                />
                
                <Show when=move || is_typing.get()>
                    <div class="gc-typing-indicator">
                        <span></span>
                        <span></span>
                        <span></span>
                    </div>
                </Show>
            </div>
            
            {if !workflow_state.get().is_complete {
                Some(view! {
                    <div class="gc-input-area">
                        {if !current_quick_replies().is_empty() {
                            Some(view! {
                                <div class="gc-quick-replies">
                                    <For
                                        each=move || current_quick_replies()
                                        key=|reply| reply.value.clone()
                                        children=move |reply| {
                                            let value = reply.value.clone();
                                            view! {
                                                <button
                                                    class="gc-quick-reply"
                                                    on:click=move |_| handle_quick_reply(value.clone())
                                                >
                                                    {reply.label}
                                                </button>
                                            }
                                        }
                                    />
                                </div>
                            })
                        } else {
                            None
                        }}
                        
                        <form on:submit=handle_submit class="gc-input-form">
                            <input
                                type="text"
                                class="gc-input"
                                placeholder=config.placeholder.clone()
                                prop:value=move || input_value.get()
                                on:input=move |ev| {
                                    let target = ev.target().unwrap();
                                    let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                    set_input_value.set(input.value());
                                }
                                disabled=is_typing.get()
                            />
                            <button
                                type="submit"
                                class="gc-submit"
                                disabled=move || input_value.get().trim().is_empty() || is_typing.get()
                            >
                                "Send"
                            </button>
                        </form>
                        
                        {if config.allow_back_navigation && !workflow_state.get().completed_steps.is_empty() {
                            Some(view! {
                                <button
                                    class="gc-back-button"
                                    on:click=move |_| handle_action(MessageAction {
                                        id: "back".to_string(),
                                        label: "Back".to_string(),
                                        action_type: ActionType::Back,
                                        value: None,
                                        style: ActionStyle::Secondary,
                                    })
                                >
                                    "← Back"
                                </button>
                            })
                        } else {
                            None
                        }}
                    </div>
                })
            } else {
                None
            }}
        </div>
    }
}

// Helper functions
fn generate_id() -> String {
    format!("msg-{}", chrono::Utc::now().timestamp_millis())
}

fn get_timestamp() -> String {
    chrono::Local::now().format("%I:%M %p").to_string()
}

fn add_assistant_message(
    content: String,
    set_messages: &WriteSignal<Vec<ChatMessage>>,
    step_id: Option<String>,
) {
    let msg = ChatMessage {
        id: generate_id(),
        message_type: MessageType::Assistant,
        content,
        timestamp: get_timestamp(),
        metadata: None,
        actions: vec![],
        step_id,
    };
    set_messages.update(|msgs| msgs.push(msg));
}

fn add_error_message(
    content: String,
    set_messages: &WriteSignal<Vec<ChatMessage>>,
    step_id: Option<String>,
) {
    let msg = ChatMessage {
        id: generate_id(),
        message_type: MessageType::Error,
        content,
        timestamp: get_timestamp(),
        metadata: None,
        actions: vec![],
        step_id,
    };
    set_messages.update(|msgs| msgs.push(msg));
}

fn add_success_message(
    content: String,
    set_messages: &WriteSignal<Vec<ChatMessage>>,
    step_id: Option<String>,
) {
    let msg = ChatMessage {
        id: generate_id(),
        message_type: MessageType::Success,
        content,
        timestamp: get_timestamp(),
        metadata: None,
        actions: vec![],
        step_id,
    };
    set_messages.update(|msgs| msgs.push(msg));
}

fn validate_input(
    value: &str,
    input_type: &InputType,
    validation: &ValidationConfig,
) -> Result<(), String> {
    // Basic validation based on input type
    match input_type {
        InputType::Number | InputType::Currency => {
            let num = value.parse::<f64>()
                .map_err(|_| "Please enter a valid number".to_string())?;
            
            if let Some(min) = validation.min_value {
                if num < min {
                    return Err(format!("Value must be at least {}", min));
                }
            }
            
            if let Some(max) = validation.max_value {
                if num > max {
                    return Err(format!("Value must be at most {}", max));
                }
            }
        }
        InputType::Text => {
            if let Some(min) = validation.min_length {
                if value.len() < min {
                    return Err(format!("Must be at least {} characters", min));
                }
            }
            
            if let Some(max) = validation.max_length {
                if value.len() > max {
                    return Err(format!("Must be at most {} characters", max));
                }
            }
        }
        _ => {}
    }
    
    Ok(())
}