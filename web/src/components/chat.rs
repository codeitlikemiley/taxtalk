use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::ev;
use serde::{Deserialize, Serialize};
use crate::config::api_url;

#[derive(Clone, Debug)]
enum MessageStatus {
    Sent,
    Delivered,
    Error,
}

#[derive(Clone, Debug)]
struct ChatMessage {
    id: String,
    content: String,
    is_user: bool,
    timestamp: String,
    status: MessageStatus,
}

#[derive(Clone, Debug, Serialize)]
struct ExecuteCommand {
    command: String,
}

#[derive(Clone, Debug, Deserialize)]
struct ExecuteResponse {
    success: bool,
    data: Option<serde_json::Value>,
    error: Option<String>,
}

#[component]
pub fn ModernChat() -> impl IntoView {
    let (messages, set_messages) = signal::<Vec<ChatMessage>>(vec![
        ChatMessage {
            id: "welcome".to_string(),
            content: "Hi! I'm TaxTalk, your AI bookkeeping assistant. I can help you with:\n\n• Creating invoices and recording payments\n• Managing VAT and tax calculations\n• Generating BIR-compliant reports\n• Tracking expenses and income\n\nJust type your command in natural language!".to_string(),
            is_user: false,
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
            status: MessageStatus::Delivered,
        }
    ]);
    
    let (input_value, set_input_value) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    
    let send_message = move |_| {
        let content = input_value.get();
        if content.trim().is_empty() {
            return;
        }
        
        // Add user message
        let user_message = ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            content: content.clone(),
            is_user: true,
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
            status: MessageStatus::Sent,
        };
        
        set_messages.update(|msgs| msgs.push(user_message));
        set_input_value.set(String::new());
        set_is_loading.set(true);
        
        // Send to server
        spawn_local(async move {
            let response = execute_command(content).await;
            
            let bot_message = match response {
                Ok(resp) => {
                    if resp.success {
                        let data_str = resp.data
                            .map(|d| format_json_response(&d))
                            .unwrap_or_else(|| "✅ Command executed successfully".to_string());
                        
                        ChatMessage {
                            id: uuid::Uuid::new_v4().to_string(),
                            content: data_str,
                            is_user: false,
                            timestamp: chrono::Local::now().format("%H:%M").to_string(),
                            status: MessageStatus::Delivered,
                        }
                    } else {
                        ChatMessage {
                            id: uuid::Uuid::new_v4().to_string(),
                            content: format!("❌ {}", resp.error.unwrap_or_else(|| "Unknown error occurred".to_string())),
                            is_user: false,
                            timestamp: chrono::Local::now().format("%H:%M").to_string(),
                            status: MessageStatus::Error,
                        }
                    }
                },
                Err(e) => ChatMessage {
                    id: uuid::Uuid::new_v4().to_string(),
                    content: format!("⚠️ Connection error: {}", e),
                    is_user: false,
                    timestamp: chrono::Local::now().format("%H:%M").to_string(),
                    status: MessageStatus::Error,
                }
            };
            
            set_messages.update(|msgs| msgs.push(bot_message));
            set_is_loading.set(false);
        });
    };
    
    let handle_keypress = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Enter" && !ev.shift_key() {
            ev.prevent_default();
            send_message(ev::MouseEvent::new("click").unwrap());
        }
    };
    
    view! {
        <div class="flex flex-col h-full">
            // Header
            <div class="bg-white border-b border-gray-200 px-6 py-4">
                <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-3">
                        <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                        <h2 class="text-lg font-semibold text-gray-800">"Chat Assistant"</h2>
                    </div>
                    <div class="flex items-center space-x-2">
                        <span class="text-sm text-gray-500">"Powered by Natural Language Processing"</span>
                    </div>
                </div>
            </div>
            
            // Messages area with modern styling
            <div class="flex-1 overflow-y-auto px-6 py-4 space-y-4 bg-gradient-to-b from-gray-50 to-white">
                <For
                    each=move || messages.get()
                    key=|msg| msg.id.clone()
                    children=move |msg| {
                        view! {
                            <div class={if msg.is_user { "flex justify-end" } else { "flex justify-start" }}>
                                <div class={if msg.is_user { "max-w-2xl" } else { "max-w-3xl" }}>
                                    <div class={if msg.is_user { 
                                        "bg-gradient-to-r from-blue-500 to-blue-600 text-white rounded-2xl rounded-br-sm px-5 py-3 shadow-lg" 
                                    } else { 
                                        "bg-white border border-gray-200 rounded-2xl rounded-bl-sm px-5 py-3 shadow-md" 
                                    }}>
                                        <div class="whitespace-pre-wrap text-sm leading-relaxed">{msg.content}</div>
                                    </div>
                                    <div class="flex items-center space-x-2 mt-1 px-2">
                                        <span class="text-xs text-gray-400">{msg.timestamp}</span>
                                        {move || match msg.status {
                                            MessageStatus::Sent => view! {
                                                <svg class="w-3 h-3 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                                                    <path d="M10 12a2 2 0 100-4 2 2 0 000 4z"/>
                                                </svg>
                                            }.into_any(),
                                            MessageStatus::Delivered => view! {
                                                <svg class="w-3 h-3 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                                                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
                                                </svg>
                                            }.into_any(),
                                            MessageStatus::Error => view! {
                                                <svg class="w-3 h-3 text-red-500" fill="currentColor" viewBox="0 0 20 20">
                                                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
                                                </svg>
                                            }.into_any(),
                                        }}
                                    </div>
                                </div>
                            </div>
                        }
                    }
                />
                
                {move || if is_loading.get() {
                    view! {
                        <div class="flex justify-start">
                            <div class="bg-white border border-gray-200 rounded-2xl rounded-bl-sm px-5 py-3 shadow-md">
                                <div class="flex space-x-2">
                                    <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce"></div>
                                    <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 0.1s"></div>
                                    <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>
            
            // Modern input area
            <div class="bg-white border-t border-gray-200 px-6 py-4">
                <div class="flex items-end space-x-3">
                    <div class="flex-1 relative">
                        <textarea
                            class="w-full px-4 py-3 pr-12 border border-gray-300 rounded-xl resize-none focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
                            placeholder="Type your command here..."
                            rows="1"
                            prop:value=move || input_value.get()
                            on:input=move |ev| set_input_value.set(event_target_value(&ev))
                            on:keydown=handle_keypress
                            disabled=move || is_loading.get()
                        />
                        <button
                            class="absolute right-2 bottom-2 p-2 text-gray-400 hover:text-gray-600 transition-colors"
                            title="Add attachment"
                        >
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13"></path>
                            </svg>
                        </button>
                    </div>
                    <button
                        class="px-6 py-3 bg-gradient-to-r from-blue-500 to-blue-600 text-white rounded-xl hover:from-blue-600 hover:to-blue-700 disabled:from-gray-400 disabled:to-gray-400 transition-all shadow-lg hover:shadow-xl transform hover:-translate-y-0.5"
                        on:click=send_message
                        disabled=move || is_loading.get()
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path>
                        </svg>
                    </button>
                </div>
                <div class="mt-2 flex items-center space-x-4 text-xs text-gray-500">
                    <span>"Press Enter to send, Shift+Enter for new line"</span>
                    <span>"•"</span>
                    <span>"Try: @client Juan bought rice 1000"</span>
                </div>
            </div>
        </div>
    }
}

async fn execute_command(command: String) -> Result<ExecuteResponse, String> {
    let client = reqwest::Client::new();
    
    let response = client
        .post(api_url("api/execute"))
        .json(&ExecuteCommand { command })
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    
    if response.status().is_success() {
        response
            .json::<ExecuteResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!("Server error: {}", response.status()))
    }
}

fn format_json_response(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Object(map) => {
            let mut result = String::new();
            for (key, val) in map {
                result.push_str(&format!("📌 {}: {}\n", key, format_json_value(val)));
            }
            result
        },
        _ => serde_json::to_string_pretty(value).unwrap_or_else(|_| "Success".to_string())
    }
}

fn format_json_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        _ => serde_json::to_string(value).unwrap_or_else(|_| "...".to_string())
    }
}