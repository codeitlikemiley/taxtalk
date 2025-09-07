use leptos::prelude::*;
use command_entity_input::{CommandEntityInput, ParseResult, InputMode};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (event_log, set_event_log) = signal(Vec::<String>::new());
    let (submitted_commands, set_submitted_commands) = signal(Vec::<(String, InputMode)>::new());
    
    let log_event = move |msg: String| {
        set_event_log.update(|log| {
            log.push(format!("{}: {}", chrono::Local::now().format("%H:%M:%S"), msg));
            if log.len() > 10 {
                log.remove(0);
            }
        });
    };
    
    let handle_submit = move |text: String, result: ParseResult| {
        log_event(format!("Submitted: {} (Mode: {:?})", text, result.mode));
        
        let mode_str = match result.mode {
            InputMode::Command => {
                if let Some(cmd) = result.command {
                    if let Some(entity) = result.entity {
                        format!("Command: {} {}", cmd, entity)
                    } else {
                        format!("Command: {}", cmd)
                    }
                } else {
                    "Command (no action)".to_string()
                }
            },
            InputMode::AI => format!("AI Query: {}", text),
        };
        
        set_submitted_commands.update(|commands| {
            commands.push((mode_str, result.mode));
            if commands.len() > 5 {
                commands.remove(0);
            }
        });
    };
    
    view! {
        <div class="demo-container">
            <style>{r#"
                body {
                    margin: 0;
                    padding: 0;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    min-height: 100vh;
                }
                
                .demo-container {
                    max-width: 1200px;
                    margin: 0 auto;
                    padding: 2rem;
                    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
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
                    background: white;
                    padding: 2rem;
                    border-radius: 0.75rem;
                    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
                    margin-bottom: 2rem;
                }
                
                .demo-section h2 {
                    font-size: 1.5rem;
                    font-weight: 600;
                    margin-bottom: 1rem;
                    color: #111827;
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
                
                .submitted-commands {
                    margin-top: 1rem;
                    padding: 1rem;
                    background: #f9fafb;
                    border: 1px solid #e5e7eb;
                    border-radius: 0.5rem;
                }
                
                .command-item {
                    display: flex;
                    align-items: center;
                    gap: 0.5rem;
                    padding: 0.5rem;
                    margin-bottom: 0.5rem;
                    background: white;
                    border: 1px solid #e5e7eb;
                    border-radius: 0.375rem;
                }
                
                .command-mode {
                    padding: 0.125rem 0.5rem;
                    border-radius: 9999px;
                    font-size: 0.75rem;
                    font-weight: 500;
                }
                
                .mode-command {
                    background: #dbeafe;
                    color: #1e40af;
                }
                
                .mode-ai {
                    background: #e9d5ff;
                    color: #6b21a8;
                }
                
                .command-text {
                    flex: 1;
                    font-size: 0.875rem;
                    color: #374151;
                }
                
                .examples {
                    margin-top: 1rem;
                    padding: 1rem;
                    background: #fef3c7;
                    border: 1px solid #fcd34d;
                    border-radius: 0.5rem;
                }
                
                .example-list {
                    font-size: 0.875rem;
                    color: #92400e;
                    line-height: 1.6;
                }
                
                .example-list code {
                    background: #fef9c3;
                    padding: 0.125rem 0.25rem;
                    border-radius: 0.25rem;
                    font-family: monospace;
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
            "#}</style>
            
            <h1>"CommandEntityInput Demo"</h1>
            <p class="subtitle">"Real-time command parsing with entity selection and validation"</p>
            
            <div class="demo-section">
                <h2>"Try the Command Input"</h2>
                <CommandEntityInput
                    placeholder=Some("Try: 'create invoice', 'update @client', or just type naturally...".to_string())
                    on_submit=Some(handle_submit)
                    show_debug=true
                />
                
                <div class="examples">
                    <strong>"Try these examples:"</strong>
                    <div class="example-list">
                        <div>"• " <code>"create invoice"</code> " - Creates a new invoice (needs entity)"</div>
                        <div>"• " <code>"update @"</code> " - Shows entity selector for update"</div>
                        <div>"• " <code>"list invoices"</code> " - Lists all invoices"</div>
                        <div>"• " <code>"delete @client"</code> " - Delete with entity selection"</div>
                        <div>"• " <code>"search for recent payments"</code> " - Natural search query"</div>
                        <div>"• " <code>"generate report"</code> " - Generate business reports"</div>
                        <div>"• " <code>"how do I create an invoice?"</code> " - AI mode for questions"</div>
                    </div>
                </div>
                
                <Show when=move || !submitted_commands.get().is_empty()>
                    <div class="submitted-commands">
                        <strong>"Recent Submissions:"</strong>
                        <For
                            each=move || submitted_commands.get()
                            key=|(text, _)| text.clone()
                            children=move |(text, mode)| {
                                view! {
                                    <div class="command-item">
                                        <span class=move || match mode {
                                            InputMode::Command => "command-mode mode-command",
                                            InputMode::AI => "command-mode mode-ai",
                                        }>
                                            {match mode {
                                                InputMode::Command => "CMD",
                                                InputMode::AI => "AI",
                                            }}
                                        </span>
                                        <span class="command-text">{text}</span>
                                    </div>
                                }
                            }
                        />
                    </div>
                </Show>
            </div>
            
            <div class="demo-section">
                <h2>"Features"</h2>
                <div class="features">
                    <div class="feature">
                        <div class="feature-title">"🎯 Smart Command Detection"</div>
                        <div class="feature-desc">"Automatically detects commands like create, update, delete, list, search"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"🔍 Entity Selection"</div>
                        <div class="feature-desc">"Type @ to see available entities with keyboard navigation"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"🤖 Dual Mode"</div>
                        <div class="feature-desc">"Switches between Command and AI mode based on input"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"✅ Real-time Validation"</div>
                        <div class="feature-desc">"Validates commands and shows helpful error messages"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"⚡ Fast Parsing"</div>
                        <div class="feature-desc">"Instant tokenization and parsing as you type"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"🎨 Visual Feedback"</div>
                        <div class="feature-desc">"Color-coded tokens for commands, entities, and parameters"</div>
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