use leptos::prelude::*;
use leptos::ev::KeyboardEvent;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::components::inline_confirmation::ParsedField;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandAction {
    pub id: String,
    pub label: String,
    pub description: String,
    pub icon: String,
    pub shortcut: Option<String>,
    pub category: String,
    pub plugin: String,
    pub action: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParsedNLPCommand {
    pub action: String,
    pub plugin: String,
    pub tokens: Vec<ParsedField>,
    pub confidence: f32,
}

#[component]
pub fn CommandPalette(
    show: ReadSignal<bool>,
    on_close: WriteSignal<bool>,
    on_execute: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    let (selected_index, set_selected_index) = signal(0);
    let (is_nlp_mode, set_is_nlp_mode) = signal(false);
    let (parsed_command, set_parsed_command) = signal::<Option<ParsedNLPCommand>>(None);
    let (_show_confirmation, set_show_confirmation) = signal(false);
    let (is_parsing, set_is_parsing) = signal(false);
    
    // Available commands (using Arc to allow sharing between closures)
    let commands = Arc::new(vec![
        CommandAction {
            id: "create_invoice".to_string(),
            label: "Create Invoice".to_string(),
            description: "Generate a new invoice for a client".to_string(),
            icon: "📄".to_string(),
            shortcut: Some("Ctrl+I".to_string()),
            category: "Sales".to_string(),
            plugin: "invoice".to_string(),
            action: "create".to_string(),
        },
        CommandAction {
            id: "record_payment".to_string(),
            label: "Record Payment".to_string(),
            description: "Record a payment from a client".to_string(),
            icon: "💰".to_string(),
            shortcut: Some("Ctrl+P".to_string()),
            category: "Finance".to_string(),
            plugin: "payment".to_string(),
            action: "create".to_string(),
        },
        CommandAction {
            id: "add_expense".to_string(),
            label: "Add Expense".to_string(),
            description: "Record a business expense".to_string(),
            icon: "💸".to_string(),
            shortcut: Some("Ctrl+E".to_string()),
            category: "Finance".to_string(),
            plugin: "expense".to_string(),
            action: "create".to_string(),
        },
        CommandAction {
            id: "new_client".to_string(),
            label: "New Client".to_string(),
            description: "Add a new client to the system".to_string(),
            icon: "👤".to_string(),
            shortcut: None,
            category: "Contacts".to_string(),
            plugin: "client".to_string(),
            action: "create".to_string(),
        },
        CommandAction {
            id: "vat_return".to_string(),
            label: "Generate VAT Return".to_string(),
            description: "Create monthly VAT return (BIR Form 2550M)".to_string(),
            icon: "📊".to_string(),
            shortcut: Some("Ctrl+V".to_string()),
            category: "Tax".to_string(),
            plugin: "tax".to_string(),
            action: "vat_return".to_string(),
        },
    ]);
    
    // Clone commands for closures - no need to clone Arc, it's already cheap to clone
    let commands_keydown = commands.clone();
    let _commands_mode = commands.clone();
    let commands_for_view = commands.clone();
    
    // Parse NLP command
    Effect::new(move |_| {
        let query = search_query.get();
        if is_nlp_mode.get() && query.len() > 5 {
            set_is_parsing.set(true);
            
            // Call NLP API
            spawn_local(async move {
                // Simulate API call
                gloo_timers::future::TimeoutFuture::new(500).await;
                
                // Mock parsed result
                if query.contains("invoice") {
                    set_parsed_command.set(Some(ParsedNLPCommand {
                        action: "create_invoice".to_string(),
                        plugin: "invoice".to_string(),
                        tokens: vec![
                            ParsedField {
                                label: "Client".to_string(),
                                value: "ABC Corporation".to_string(),
                                field_type: "entity".to_string(),
                                editable: true,
                            },
                            ParsedField {
                                label: "Amount".to_string(),
                                value: "₱5,000".to_string(),
                                field_type: "amount".to_string(),
                                editable: true,
                            },
                        ],
                        confidence: 0.85,
                    }));
                } else {
                    set_parsed_command.set(None);
                }
                
                set_is_parsing.set(false);
            });
        }
    });
    
    
    view! {
        <Show when=move || show.get()>
            <div class="fixed inset-0 bg-black bg-opacity-50 flex items-start justify-center pt-20 z-50">
                <div class="bg-white rounded-2xl shadow-2xl w-full max-w-2xl animate-slide-down">
                    <div class="p-4 border-b border-gray-200">
                        <input
                            type="text"
                            class="w-full px-3 py-2 text-lg focus:outline-none"
                            placeholder="Type a command or describe what you want to do..."
                            prop:value=move || search_query.get()
                            on:input=move |ev| set_search_query.set(event_target_value(&ev))
                            on:keydown={
                                let commands_for_keydown = commands_keydown.clone();
                                move |ev: KeyboardEvent| {
                                    let commands_local = commands_for_keydown.clone();
                                    match ev.key().as_str() {
                                        "ArrowDown" => {
                                            ev.prevent_default();
                                            let query = search_query.get().to_lowercase();
                                            let cmds = if query.is_empty() {
                                                (*commands_local).clone()
                                            } else if query.len() > 3 && !query.starts_with('/') {
                                                vec![]
                                            } else {
                                                commands_local
                                                    .iter()
                                                    .filter(|cmd| {
                                                        cmd.label.to_lowercase().contains(&query)
                                                            || cmd.description.to_lowercase().contains(&query)
                                                            || cmd.category.to_lowercase().contains(&query)
                                                    })
                                                    .cloned()
                                                    .collect()
                                            };
                                            let max = cmds.len().saturating_sub(1);
                                            set_selected_index.set((selected_index.get() + 1).min(max));
                                        }
                                        "ArrowUp" => {
                                            ev.prevent_default();
                                            set_selected_index
                                                .set(selected_index.get().saturating_sub(1));
                                        }
                                        "Enter" => {
                                            ev.prevent_default();
                                            if is_nlp_mode.get() {
                                                if parsed_command.get().is_some() {
                                                    set_show_confirmation.set(true);
                                                }
                                            } else {
                                                let query = search_query.get().to_lowercase();
                                                let cmds = if query.is_empty() {
                                                    (*commands_local).clone()
                                                } else {
                                                    commands_local
                                                        .iter()
                                                        .filter(|cmd| {
                                                            cmd.label.to_lowercase().contains(&query)
                                                                || cmd.description.to_lowercase().contains(&query)
                                                                || cmd.category.to_lowercase().contains(&query)
                                                        })
                                                        .cloned()
                                                        .collect::<Vec<_>>()
                                                };
                                                if let Some(cmd) = cmds.get(selected_index.get()) {
                                                    on_execute
                                                        .set(Some(format!("{}:{}", cmd.plugin, cmd.action)));
                                                    on_close.set(true);
                                                }
                                            }
                                        }
                                        "Escape" => {
                                            ev.prevent_default();
                                            on_close.set(true);
                                        }
                                        "/" => {
                                            set_is_nlp_mode.set(false);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            autofocus=true
                        />
                    </div>

                    {{
                        let commands_view_clone = commands_for_view.clone();
                        move || {
                            let commands_for_mode = commands_view_clone.clone();
                            if is_nlp_mode.get() {
                                // NLP Mode
                                view! {
                                    <div class="p-4">
                                        {move || {
                                            if is_parsing.get() {
                                                view! {
                                                    <div class="flex items-center justify-center py-8">
                                                        <div class="flex items-center space-x-3">
                                                            <div class="w-6 h-6 border-2 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
                                                            <span class="text-gray-600">
                                                                Understanding your command...
                                                            </span>
                                                        </div>
                                                    </div>
                                                }
                                                    .into_any()
                                            } else if let Some(cmd) = parsed_command.get() {
                                                let confidence = cmd.confidence;
                                                let confidence_text = format!(
                                                    "{}% confident",
                                                    (confidence * 100.0) as u32,
                                                );
                                                let action_text = cmd
                                                    .action
                                                    .replace('_', " ")
                                                    .to_uppercase();
                                                let tokens = cmd.tokens.clone();

                                                view! {
                                                    <div class="space-y-4">
                                                        <div class="flex items-center justify-between">
                                                            <div class="flex items-center space-x-2">
                                                                <span class="text-lg">{"🤖"}</span>
                                                                <span class="text-sm font-medium text-gray-700">
                                                                    "I understand you want to:"
                                                                </span>
                                                            </div>
                                                            <span class=move || {
                                                                if confidence > 0.8 {
                                                                    "text-xs px-2 py-1 bg-green-100 text-green-700 rounded"
                                                                } else {
                                                                    "text-xs px-2 py-1 bg-yellow-100 text-yellow-700 rounded"
                                                                }
                                                            }>{confidence_text}</span>
                                                        </div>

                                                        <div class="bg-gray-50 rounded-lg p-4">
                                                            <h3 class="font-medium text-gray-900 mb-2">
                                                                {action_text}
                                                            </h3>
                                                            <div class="space-y-2">
                                                                <For
                                                                    each=move || tokens.clone()
                                                                    key=|field| field.label.clone()
                                                                    children=move |field| {
                                                                        view! {
                                                                            <div class="flex items-center space-x-2">
                                                                                <span class="text-sm text-gray-600 w-20">
                                                                                    {field.label}":"
                                                                                </span>
                                                                                <span class="text-sm font-medium text-gray-900">
                                                                                    {field.value}
                                                                                </span>
                                                                            </div>
                                                                        }
                                                                    }
                                                                />
                                                            </div>
                                                        </div>

                                                        <div class="flex justify-end space-x-3">
                                                            <button
                                                                class="px-4 py-2 text-gray-600 hover:text-gray-800"
                                                                on:click=move |_| {
                                                                    set_parsed_command.set(None);
                                                                    set_search_query.set(String::new());
                                                                }
                                                            >
                                                                "Try Again"
                                                            </button>
                                                            <button
                                                                class="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg"
                                                                on:click=move |_| {
                                                                    if let Some(cmd) = parsed_command.get() {
                                                                        on_execute
                                                                            .set(
                                                                                Some(
                                                                                    format!(
                                                                                        "nlp:{}",
                                                                                        serde_json::to_string(&cmd).unwrap_or_default(),
                                                                                    ),
                                                                                ),
                                                                            );
                                                                        on_close.set(true);
                                                                    }
                                                                }
                                                            >
                                                                "Execute"
                                                            </button>
                                                        </div>
                                                    </div>
                                                }
                                                    .into_any()
                                            } else {
                                                view! {
                                                    <div class="text-center py-8 text-gray-500">
                                                        <p class="mb-2">
                                                            "Keep typing to describe what you want to do"
                                                        </p>
                                                        <p class="text-xs">
                                                            "Or press / to switch to command mode"
                                                        </p>
                                                    </div>
                                                }
                                                    .into_any()
                                            }
                                        }}
                                    </div>
                                }
                                    .into_any()
                            } else {
                                // Command Mode
                                view! {
                                    <div class="max-h-96 overflow-y-auto">
                                        {move || {
                                            let query = search_query.get().to_lowercase();
                                            let cmds_arc = commands_for_mode.clone();
                                            let cmds = if query.is_empty() {
                                                (*cmds_arc).clone()
                                            } else if query.len() > 3 && !query.starts_with('/') {
                                                set_is_nlp_mode.set(true);
                                                vec![]
                                            } else {
                                                set_is_nlp_mode.set(false);
                                                cmds_arc
                                                    .iter()
                                                    .filter(|cmd| {
                                                        cmd.label.to_lowercase().contains(&query)
                                                            || cmd.description.to_lowercase().contains(&query)
                                                            || cmd.category.to_lowercase().contains(&query)
                                                    })
                                                    .cloned()
                                                    .collect()
                                            };
                                            if cmds.is_empty() {
                                                view! {
                                                    <div class="p-8 text-center text-gray-500">
                                                        "No commands found"
                                                    </div>
                                                }
                                                    .into_any()
                                            } else {
                                                view! {
                                                    <div class="py-2">
                                                        <For
                                                            each=move || cmds.clone().into_iter().enumerate()
                                                            key=|(_, cmd)| cmd.id.clone()
                                                            children=move |(index, cmd)| {
                                                                let is_selected = move || selected_index.get() == index;
                                                                let cmd_for_click = cmd.clone();

                                                                view! {
                                                                    <button
                                                                        class=move || {
                                                                            if is_selected() {
                                                                                "w-full px-4 py-3 flex items-center space-x-3 bg-blue-50 hover:bg-blue-100"
                                                                            } else {
                                                                                "w-full px-4 py-3 flex items-center space-x-3 hover:bg-gray-50"
                                                                            }
                                                                        }
                                                                        on:click=move |_| {
                                                                            on_execute
                                                                                .set(
                                                                                    Some(
                                                                                        format!("{}:{}", cmd_for_click.plugin, cmd_for_click.action),
                                                                                    ),
                                                                                );
                                                                            on_close.set(true);
                                                                        }
                                                                        on:mouseenter=move |_| set_selected_index.set(index)
                                                                    >
                                                                        <span class="text-2xl">{cmd.icon.clone()}</span>
                                                                        <div class="flex-1 text-left">
                                                                            <div class="flex items-center space-x-2">
                                                                                <span class="font-medium text-gray-900">
                                                                                    {cmd.label.clone()}
                                                                                </span>
                                                                                <span class="text-xs px-2 py-0.5 bg-gray-200 text-gray-600 rounded">
                                                                                    {cmd.category.clone()}
                                                                                </span>
                                                                            </div>
                                                                            <p class="text-sm text-gray-600 mt-0.5">
                                                                                {cmd.description.clone()}
                                                                            </p>
                                                                        </div>
                                                                        {cmd
                                                                            .shortcut
                                                                            .as_ref()
                                                                            .map(|shortcut| {
                                                                                view! {
                                                                                    <kbd class="text-xs px-2 py-1 bg-gray-100 text-gray-600 rounded">
                                                                                        {shortcut.clone()}
                                                                                    </kbd>
                                                                                }
                                                                                    .into_any()
                                                                            })
                                                                            .unwrap_or_else(|| view! { <span></span> }.into_any())}
                                                                    </button>
                                                                }
                                                            }
                                                        />
                                                    </div>
                                                }
                                                    .into_any()
                                            }
                                        }}
                                    </div>
                                }
                                    .into_any()
                            }
                        }
                    }}

                    <div class="px-4 py-3 border-t border-gray-200 flex items-center justify-between text-xs text-gray-500">
                        <div class="flex items-center space-x-4">
                            <span class="flex items-center space-x-1">
                                <kbd class="px-1.5 py-0.5 bg-gray-100 rounded">{"↑↓"}</kbd>
                                <span>Navigate</span>
                            </span>
                            <span class="flex items-center space-x-1">
                                <kbd class="px-1.5 py-0.5 bg-gray-100 rounded">Enter</kbd>
                                <span>Select</span>
                            </span>
                            <span class="flex items-center space-x-1">
                                <kbd class="px-1.5 py-0.5 bg-gray-100 rounded">Esc</kbd>
                                <span>Cancel</span>
                            </span>
                        </div>
                        <div>
                            {move || {
                                if is_nlp_mode.get() {
                                    view! { <span>"Natural Language Mode"</span> }.into_any()
                                } else {
                                    view! { <span>"Command Mode (Press / to filter)"</span> }
                                        .into_any()
                                }
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}

use leptos::task::spawn_local;