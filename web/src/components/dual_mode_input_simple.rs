use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use web_sys::KeyboardEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandInfo {
    pub name: String,  // Changed from 'command' to 'name' to match server
    pub aliases: Vec<String>,
    pub description: String,
    pub plugin_id: String,
    pub required_tokens: Vec<TokenInfo>,
    pub optional_tokens: Vec<TokenInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub token_type: String,
    pub description: String,
    pub multiple: bool,
    pub entity_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Tokenization,
    AI,
}

#[component]
pub fn DualModeInputSimple() -> impl IntoView {
    let (input_value, set_input_value) = signal("".to_string());
    let (mode, set_mode) = signal(InputMode::AI);
    let (valid_commands, set_valid_commands) = signal(Vec::<CommandInfo>::new());
    let (current_command, set_current_command) = signal(Option::<String>::None);
    let (command_result, set_command_result) = signal(Option::<String>::None);
    
    // Load valid commands on mount
    Effect::new(move |_| {
        spawn_local(async move {
            // Use env var or default to localhost:3000
            let api_base = option_env!("API_URL").unwrap_or("http://localhost:3000");
            let url = format!("{}/api/commands", api_base);
            match reqwest::get(&url).await {
                Ok(resp) => {
                    if let Ok(commands) = resp.json::<Vec<CommandInfo>>().await {
                        set_valid_commands.set(commands);
                    }
                }
                Err(_e) => {
                    // Failed to load commands
                }
            }
        });
    });
    
    // Detect command in input and switch modes
    Effect::new(move |_| {
        let value = input_value.get();
        let commands = valid_commands.get();
        
        // Check if input contains a valid command
        let mut found_command = None;
        for cmd_info in &commands {
            // Check main command name
            if value.to_lowercase().contains(&cmd_info.name.to_lowercase()) {
                found_command = Some(cmd_info.name.clone());
                break;
            }
            // Check aliases
            for alias in &cmd_info.aliases {
                if value.to_lowercase().contains(&alias.to_lowercase()) {
                    found_command = Some(cmd_info.name.clone());
                    break;
                }
            }
        }
        
        if let Some(cmd) = found_command {
            // Switch to tokenization mode
            set_mode.set(InputMode::Tokenization);
            set_current_command.set(Some(cmd));
        } else {
            // Switch to AI mode
            set_mode.set(InputMode::AI);
            set_current_command.set(None);
        }
    });
    
    let handle_keydown = move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            // Just Enter to submit (no Cmd needed)
            e.prevent_default();
            
            let mode_val = mode.get();
            let value = input_value.get();
            
            if mode_val == InputMode::Tokenization {
                set_command_result.set(Some(format!("Tokenized command: {}", value)));
                // Auto-clear for next command
                set_input_value.set("".to_string());
            } else {
                set_command_result.set(Some(format!("AI query: {}", value)));
                set_input_value.set("".to_string());
            }
        }
    };
    
    view! {
        <div class="dual-mode-input">
            <div class="mode-indicator mb-4">
                {move || match mode.get() {
                    InputMode::Tokenization => view! {
                        <span class="mode-badge tokenization">
                            "📝 Tokenization Mode"
                        </span>
                    }.into_view(),
                    InputMode::AI => view! {
                        <span class="mode-badge ai">
                            "🤖 AI Mode"
                        </span>
                    }.into_view(),
                }}
            </div>
            
            {move || {
                current_command.get().map(|cmd| view! {
                    <div class="command-tag mb-2">
                        "Command: " {cmd}
                    </div>
                })
            }}
            
            <div class="input-area">
                <input
                    type="text"
                    class="w-full p-4 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-lg"
                    placeholder="Type a command like 'create invoice' or ask a question..."
                    prop:value=move || input_value.get()
                    on:input=move |e| {
                        let value = event_target_value(&e);
                        set_input_value.set(value);
                    }
                    on:keydown=handle_keydown
                />
            </div>
            
            <div class="help-text text-xs text-gray-500 mt-2">
                "Press Enter to submit"
            </div>
            
            {move || {
                command_result.get().map(|result| view! {
                    <div class="mt-4 p-4 bg-gray-100 rounded-lg">
                        <div class="text-sm font-semibold text-gray-700">Result:</div>
                        <div class="text-sm text-gray-600">{result}</div>
                    </div>
                })
            }}
            
            // Show loaded commands for debugging
            <div class="mt-4 text-xs text-gray-400">
                "Loaded commands: "
                {move || valid_commands.get().iter().map(|cmd| cmd.name.clone()).collect::<Vec<_>>().join(", ")}
            </div>
        </div>
    }
}