use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{Local, Duration};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuickAction {
    pub label: String,
    pub value: String,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenQuickAction {
    pub token_name: String,
    pub token_type: String,
    pub prompt: String,
    pub actions: Vec<QuickAction>,
}

#[component]
pub fn QuickActionBar(
    token: String,
    token_type: String,
    prompt: String,
    on_action: WriteSignal<Option<String>>,
    on_custom: WriteSignal<bool>,
) -> impl IntoView {
    let actions = generate_quick_actions(&token, &token_type);
    
    view! {
        <div class="bg-gradient-to-r from-blue-50 to-indigo-50 rounded-xl p-4 mb-3 border border-blue-200">
            <div class="mb-3">
                <p class="text-sm font-medium text-gray-700">{prompt}</p>
            </div>
            
            {if !actions.is_empty() {
                view! {
                    <div class="flex flex-wrap gap-2">
                        <For
                            each=move || actions.clone()
                            key=|action| action.value.clone()
                            children=move |action| {
                                let action_value = action.value.clone();
                                let color = action.color.unwrap_or_else(|| "blue".to_string());
                                let class = match color.as_str() {
                                    "green" => "px-4 py-2 bg-green-100 hover:bg-green-200 text-green-700 rounded-lg transition-all transform hover:scale-105 flex items-center space-x-2 text-sm font-medium shadow-sm",
                                    "blue" => "px-4 py-2 bg-blue-100 hover:bg-blue-200 text-blue-700 rounded-lg transition-all transform hover:scale-105 flex items-center space-x-2 text-sm font-medium shadow-sm",
                                    "purple" => "px-4 py-2 bg-purple-100 hover:bg-purple-200 text-purple-700 rounded-lg transition-all transform hover:scale-105 flex items-center space-x-2 text-sm font-medium shadow-sm",
                                    "gray" => "px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg transition-all transform hover:scale-105 flex items-center space-x-2 text-sm font-medium shadow-sm",
                                    "red" => "px-4 py-2 bg-red-100 hover:bg-red-200 text-red-700 rounded-lg transition-all transform hover:scale-105 flex items-center space-x-2 text-sm font-medium shadow-sm",
                                    "orange" => "px-4 py-2 bg-orange-100 hover:bg-orange-200 text-orange-700 rounded-lg transition-all transform hover:scale-105 flex items-center space-x-2 text-sm font-medium shadow-sm",
                                    "yellow" => "px-4 py-2 bg-yellow-100 hover:bg-yellow-200 text-yellow-700 rounded-lg transition-all transform hover:scale-105 flex items-center space-x-2 text-sm font-medium shadow-sm",
                                    "indigo" => "px-4 py-2 bg-indigo-100 hover:bg-indigo-200 text-indigo-700 rounded-lg transition-all transform hover:scale-105 flex items-center space-x-2 text-sm font-medium shadow-sm",
                                    "pink" => "px-4 py-2 bg-pink-100 hover:bg-pink-200 text-pink-700 rounded-lg transition-all transform hover:scale-105 flex items-center space-x-2 text-sm font-medium shadow-sm",
                                    _ => "px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg transition-all transform hover:scale-105 flex items-center space-x-2 text-sm font-medium shadow-sm"
                                };
                                
                                view! {
                                    <button
                                        class=class
                                        on:click=move |_| on_action.set(Some(action_value.clone()))
                                    >
                                        {action.icon.as_ref().map(|icon| {
                                            view! {
                                                <span class="text-lg">{icon.clone()}</span>
                                            }.into_any()
                                        }).unwrap_or_else(|| view! { <span></span> }.into_any())}
                                        <span>{action.label.clone()}</span>
                                    </button>
                                }
                            }
                        />
                        
                        <button
                            class="px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg transition-all transform hover:scale-105 flex items-center space-x-2 text-sm font-medium shadow-sm"
                            on:click=move |_| on_custom.set(true)
                        >
                            <span class="text-lg">"✏️"</span>
                            <span>"Custom Input"</span>
                        </button>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}
        </div>
    }
}

fn generate_quick_actions(token: &str, token_type: &str) -> Vec<QuickAction> {
    match token {
        "amount" => vec![
            QuickAction {
                label: "₱1,000".to_string(),
                value: "1000".to_string(),
                icon: Some("💵".to_string()),
                color: Some("green".to_string()),
            },
            QuickAction {
                label: "₱5,000".to_string(),
                value: "5000".to_string(),
                icon: Some("💵".to_string()),
                color: Some("green".to_string()),
            },
            QuickAction {
                label: "₱10,000".to_string(),
                value: "10000".to_string(),
                icon: Some("💵".to_string()),
                color: Some("green".to_string()),
            },
            QuickAction {
                label: "₱25,000".to_string(),
                value: "25000".to_string(),
                icon: Some("💵".to_string()),
                color: Some("green".to_string()),
            },
            QuickAction {
                label: "₱50,000".to_string(),
                value: "50000".to_string(),
                icon: Some("💵".to_string()),
                color: Some("green".to_string()),
            },
        ],
        "method" | "payment_method" => vec![
            QuickAction {
                label: "Cash".to_string(),
                value: "cash".to_string(),
                icon: Some("💰".to_string()),
                color: Some("green".to_string()),
            },
            QuickAction {
                label: "Bank Transfer".to_string(),
                value: "bank_transfer".to_string(),
                icon: Some("🏦".to_string()),
                color: Some("blue".to_string()),
            },
            QuickAction {
                label: "GCash".to_string(),
                value: "gcash".to_string(),
                icon: Some("📱".to_string()),
                color: Some("blue".to_string()),
            },
            QuickAction {
                label: "Maya".to_string(),
                value: "maya".to_string(),
                icon: Some("📱".to_string()),
                color: Some("purple".to_string()),
            },
            QuickAction {
                label: "Check".to_string(),
                value: "check".to_string(),
                icon: Some("📝".to_string()),
                color: Some("gray".to_string()),
            },
            QuickAction {
                label: "Credit Card".to_string(),
                value: "credit_card".to_string(),
                icon: Some("💳".to_string()),
                color: Some("indigo".to_string()),
            },
        ],
        "due_date" => vec![
            QuickAction {
                label: "Today".to_string(),
                value: Local::now().format("%Y-%m-%d").to_string(),
                icon: Some("📅".to_string()),
                color: Some("red".to_string()),
            },
            QuickAction {
                label: "Tomorrow".to_string(),
                value: (Local::now() + Duration::days(1))
                    .format("%Y-%m-%d")
                    .to_string(),
                icon: Some("📅".to_string()),
                color: Some("orange".to_string()),
            },
            QuickAction {
                label: "7 Days".to_string(),
                value: (Local::now() + Duration::days(7))
                    .format("%Y-%m-%d")
                    .to_string(),
                icon: Some("📅".to_string()),
                color: Some("yellow".to_string()),
            },
            QuickAction {
                label: "15 Days".to_string(),
                value: (Local::now() + Duration::days(15))
                    .format("%Y-%m-%d")
                    .to_string(),
                icon: Some("📅".to_string()),
                color: Some("blue".to_string()),
            },
            QuickAction {
                label: "30 Days".to_string(),
                value: (Local::now() + Duration::days(30))
                    .format("%Y-%m-%d")
                    .to_string(),
                icon: Some("📅".to_string()),
                color: Some("green".to_string()),
            },
        ],
        "tax_exempt" => vec![
            QuickAction {
                label: "Yes - Tax Exempt".to_string(),
                value: "true".to_string(),
                icon: Some("✅".to_string()),
                color: Some("green".to_string()),
            },
            QuickAction {
                label: "No - Standard VAT".to_string(),
                value: "false".to_string(),
                icon: Some("❌".to_string()),
                color: Some("red".to_string()),
            },
        ],
        "discount_type" => vec![
            QuickAction {
                label: "Senior Citizen".to_string(),
                value: "senior".to_string(),
                icon: Some("👴".to_string()),
                color: Some("purple".to_string()),
            },
            QuickAction {
                label: "PWD".to_string(),
                value: "pwd".to_string(),
                icon: Some("♿".to_string()),
                color: Some("blue".to_string()),
            },
            QuickAction {
                label: "Volume Discount".to_string(),
                value: "volume".to_string(),
                icon: Some("📦".to_string()),
                color: Some("orange".to_string()),
            },
            QuickAction {
                label: "Promotional".to_string(),
                value: "promo".to_string(),
                icon: Some("🎁".to_string()),
                color: Some("pink".to_string()),
            },
        ],
        _ => {
            // For entity references, show recent selections
            if token_type.contains("EntityReference") {
                vec![
                    QuickAction {
                        label: "Browse All".to_string(),
                        value: "@browse".to_string(),
                        icon: Some("🔍".to_string()),
                        color: Some("blue".to_string()),
                    },
                    QuickAction {
                        label: "Recent".to_string(),
                        value: "@recent".to_string(),
                        icon: Some("🕐".to_string()),
                        color: Some("gray".to_string()),
                    },
                ]
            } else {
                vec![]
            }
        }
    }
}

#[component]
pub fn MiniSessionInput(
    token_name: String,
    token_type: String,
    prompt: String,
    on_submit: WriteSignal<Option<String>>,
    on_cancel: WriteSignal<bool>,
) -> impl IntoView {
    let (input_value, set_input_value) = signal(String::new());
    let (show_quick_actions, set_show_quick_actions) = signal(true);
    
    let handle_submit = move || {
        let value = input_value.get();
        if !value.is_empty() {
            on_submit.set(Some(value));
        }
    };
    
    let handle_quick_action = move |value: Option<String>| {
        if let Some(v) = value {
            if v == "@browse" || v == "@recent" {
                // Handle special actions
                set_show_quick_actions.set(false);
            } else {
                on_submit.set(Some(v));
            }
        }
    };
    
    view! {
        <div class="fixed inset-0 bg-black bg-opacity-30 flex items-center justify-center z-50">
            <div class="bg-white rounded-2xl shadow-2xl max-w-2xl w-full mx-4">
                <div class="px-6 py-4 border-b border-gray-200">
                    <h3 class="text-lg font-semibold text-gray-900">
                        {prompt.clone()}
                    </h3>
                    <p class="text-sm text-gray-500 mt-1">
                        "Select a quick option or enter a custom value"
                    </p>
                </div>
                
                <div class="p-6">
                    {move || if show_quick_actions.get() {
                        // Create signals for QuickActionBar
                        let (quick_action_value, set_quick_action_value) = signal(None::<String>);
                        let (quick_custom, set_quick_custom) = signal(false);
                        
                        // Watch for changes from QuickActionBar
                        Effect::new(move |_| {
                            if let Some(val) = quick_action_value.get() {
                                if val == "@browse" || val == "@recent" {
                                    set_show_quick_actions.set(false);
                                } else {
                                    on_submit.set(Some(val));
                                }
                            }
                        });
                        
                        Effect::new(move |_| {
                            if quick_custom.get() {
                                set_show_quick_actions.set(false);
                            }
                        });
                        
                        view! {
                            <QuickActionBar
                                token=token_name.clone()
                                token_type=token_type.clone()
                                prompt="".to_string()
                                on_action=set_quick_action_value
                                on_custom=set_quick_custom
                            />
                        }.into_any()
                    } else {
                        view! {
                            <div class="space-y-4">
                                <input
                                    type="text"
                                    class="w-full px-4 py-3 border-2 border-gray-300 rounded-xl focus:outline-none focus:border-blue-500 text-lg"
                                    placeholder={format!("Enter {} value...", token_name)}
                                    prop:value=move || input_value.get()
                                    on:input=move |ev| set_input_value.set(event_target_value(&ev))
                                    on:keydown=move |ev| {
                                        if ev.key() == "Enter" {
                                            handle_submit();
                                        } else if ev.key() == "Escape" {
                                            on_cancel.set(true);
                                        }
                                    }
                                    autofocus=true
                                />
                                
                                <div class="flex justify-end space-x-3">
                                    <button
                                        class="px-4 py-2 text-gray-600 hover:text-gray-800 transition-colors"
                                        on:click=move |_| set_show_quick_actions.set(true)
                                    >
                                        "Back to Quick Options"
                                    </button>
                                    <button
                                        class="px-6 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors"
                                        on:click=move |_| handle_submit()
                                    >
                                        "Submit"
                                    </button>
                                </div>
                            </div>
                        }.into_any()
                    }}
                </div>
            </div>
        </div>
    }
}