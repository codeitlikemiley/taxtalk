use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{Datelike, Timelike};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Suggestion {
    pub id: String,
    pub text: String,
    pub icon: String,
    pub category: String,
    pub action: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContextualHint {
    pub message: String,
    pub action: Option<String>,
    pub icon: String,
}

#[component]
pub fn SmartSuggestions(
    context: ReadSignal<String>,
    on_select: WriteSignal<Option<String>>,
) -> impl IntoView {
    // Generate suggestions based on context
    let suggestions = move || {
        let ctx = context.get();
        
        if ctx.is_empty() {
            // Default suggestions
            vec![
                Suggestion {
                    id: "quick_invoice".to_string(),
                    text: "Create quick invoice".to_string(),
                    icon: "⚡".to_string(),
                    category: "Quick Action".to_string(),
                    action: "invoice:quick".to_string(),
                },
                Suggestion {
                    id: "recent_clients".to_string(),
                    text: "View recent clients".to_string(),
                    icon: "👥".to_string(),
                    category: "View".to_string(),
                    action: "view:clients".to_string(),
                },
                Suggestion {
                    id: "today_summary".to_string(),
                    text: "Today's summary".to_string(),
                    icon: "📊".to_string(),
                    category: "Report".to_string(),
                    action: "report:today".to_string(),
                },
            ]
        } else if ctx.contains("overdue") {
            vec![
                Suggestion {
                    id: "overdue_invoices".to_string(),
                    text: "Show overdue invoices".to_string(),
                    icon: "⚠️".to_string(),
                    category: "Alert".to_string(),
                    action: "view:overdue_invoices".to_string(),
                },
                Suggestion {
                    id: "send_reminders".to_string(),
                    text: "Send payment reminders".to_string(),
                    icon: "📧".to_string(),
                    category: "Action".to_string(),
                    action: "action:send_reminders".to_string(),
                },
            ]
        } else if ctx.contains("vat") || ctx.contains("tax") {
            vec![
                Suggestion {
                    id: "vat_return".to_string(),
                    text: "Generate VAT return".to_string(),
                    icon: "📋".to_string(),
                    category: "Tax".to_string(),
                    action: "tax:vat_return".to_string(),
                },
                Suggestion {
                    id: "tax_summary".to_string(),
                    text: "View tax summary".to_string(),
                    icon: "📊".to_string(),
                    category: "Report".to_string(),
                    action: "report:tax_summary".to_string(),
                },
            ]
        } else {
            vec![]
        }
    };
    
    // Contextual hints based on time/date
    let contextual_hint = move || {
        let now = chrono::Local::now();
        let day = now.day();
        let hour = now.hour();
        
        if day >= 15 && day <= 20 {
            Some(ContextualHint {
                message: "VAT return deadline approaching (20th)".to_string(),
                action: Some("tax:vat_return".to_string()),
                icon: "⏰".to_string(),
            })
        } else if hour >= 17 {
            Some(ContextualHint {
                message: "End of day - Review today's transactions?".to_string(),
                action: Some("report:today".to_string()),
                icon: "🌆".to_string(),
            })
        } else if hour < 10 {
            Some(ContextualHint {
                message: "Good morning! Check pending tasks".to_string(),
                action: Some("view:tasks".to_string()),
                icon: "☀️".to_string(),
            })
        } else {
            None
        }
    };
    
    view! {
        <div class="space-y-3">
            {move || contextual_hint().map(|hint| {
                view! {
                    <div class="bg-gradient-to-r from-blue-50 to-indigo-50 rounded-lg p-3 border border-blue-200">
                        <div class="flex items-center justify-between">
                            <div class="flex items-center space-x-2">
                                <span class="text-xl">{hint.icon}</span>
                                <span class="text-sm text-gray-700">{hint.message}</span>
                            </div>
                            {hint.action.as_ref().map(|action| {
                                let action_clone = action.clone();
                                view! {
                                    <button
                                        class="text-xs px-3 py-1 bg-blue-500 hover:bg-blue-600 text-white rounded transition-colors"
                                        on:click=move |_| on_select.set(Some(action_clone.clone()))
                                    >
                                        "Take Action"
                                    </button>
                                }.into_any()
                            }).unwrap_or_else(|| view! { <span></span> }.into_any())}
                        </div>
                    </div>
                }.into_any()
            }).unwrap_or_else(|| view! { <div></div> }.into_any())}
            
            {move || if !suggestions().is_empty() {
                view! {
                    <div class="bg-white rounded-lg border border-gray-200">
                        <div class="px-3 py-2 border-b border-gray-200">
                            <h3 class="text-xs font-medium text-gray-600 uppercase tracking-wider">
                                "Suggested Actions"
                            </h3>
                        </div>
                        <div class="py-1">
                            <For
                                each=move || suggestions()
                                key=|s| s.id.clone()
                                children=move |suggestion| {
                                    let action = suggestion.action.clone();
                                    view! {
                                        <button
                                            class="w-full px-3 py-2 flex items-center space-x-3 hover:bg-gray-50 transition-colors"
                                            on:click=move |_| on_select.set(Some(action.clone()))
                                        >
                                            <span class="text-lg">{suggestion.icon}</span>
                                            <div class="flex-1 text-left">
                                                <p class="text-sm text-gray-900">{suggestion.text}</p>
                                                <p class="text-xs text-gray-500">{suggestion.category}</p>
                                            </div>
                                            <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                            </svg>
                                        </button>
                                    }
                                }
                            />
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>
    }
}

#[component] 
pub fn InlineHelp(
    visible: ReadSignal<bool>,
    message: String,
) -> impl IntoView {
    view! {
        <Show when=move || visible.get()>
            <div class="fixed bottom-4 left-4 max-w-sm bg-white rounded-lg shadow-lg border border-gray-200 p-4 animate-slide-up">
                <div class="flex items-start space-x-3">
                    <div class="flex-shrink-0">
                        <span class="text-2xl">{"💡"}</span>
                    </div>
                    <div class="flex-1">
                        <h4 class="text-sm font-medium text-gray-900 mb-1">
                            "Tip"
                        </h4>
                        <p class="text-sm text-gray-600">
                            {message.clone()}
                        </p>
                    </div>
                    <button
                        class="flex-shrink-0 text-gray-400 hover:text-gray-600"
                        on:click=move |_| {
                            // Close help
                        }
                    >
                        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path>
                        </svg>
                    </button>
                </div>
                
                <div class="mt-3 flex space-x-2">
                    <button class="text-xs px-3 py-1 bg-blue-50 text-blue-600 rounded hover:bg-blue-100 transition-colors">
                        "Learn More"
                    </button>
                    <button class="text-xs px-3 py-1 text-gray-600 hover:text-gray-800 transition-colors">
                        "Don't show again"
                    </button>
                </div>
            </div>
        </Show>
    }
}