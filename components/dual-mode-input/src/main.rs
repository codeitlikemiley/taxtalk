use leptos::prelude::*;
use dual_mode_input::{DualModeInput, InputMode};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (submissions, set_submissions) = signal(Vec::<(String, String)>::new());
    let (event_log, set_event_log) = signal(Vec::<String>::new());
    
    let log_event = move |msg: String| {
        set_event_log.update(|log| {
            log.push(format!("{}: {}", chrono::Local::now().format("%H:%M:%S"), msg));
            if log.len() > 10 {
                log.remove(0);
            }
        });
    };
    
    let handle_submit = move |text: String, mode: InputMode| {
        let mode_str = match mode {
            InputMode::AI => "AI",
            InputMode::Tokenization => "Token",
        };
        
        log_event(format!("Submitted ({} mode): {}", mode_str, text));
        
        set_submissions.update(|subs| {
            subs.push((mode_str.to_string(), text));
            if subs.len() > 5 {
                subs.remove(0);
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
                
                .demo-section h3 {
                    font-size: 1.125rem;
                    font-weight: 600;
                    margin-bottom: 1rem;
                    margin-top: 2rem;
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
                    line-height: 1.8;
                }
                
                .example-list code {
                    background: #fef9c3;
                    padding: 0.125rem 0.375rem;
                    border-radius: 0.25rem;
                    font-family: monospace;
                    font-weight: 500;
                }
                
                .submissions {
                    margin-top: 1.5rem;
                    padding: 1rem;
                    background: #eff6ff;
                    border: 1px solid #bfdbfe;
                    border-radius: 0.5rem;
                }
                
                .submission-item {
                    display: flex;
                    align-items: center;
                    gap: 0.75rem;
                    padding: 0.5rem;
                    margin-bottom: 0.5rem;
                    background: white;
                    border: 1px solid #dbeafe;
                    border-radius: 0.375rem;
                    font-size: 0.875rem;
                }
                
                .submission-mode {
                    padding: 0.125rem 0.5rem;
                    border-radius: 9999px;
                    font-size: 0.75rem;
                    font-weight: 600;
                    flex-shrink: 0;
                }
                
                .mode-ai {
                    background: #e9d5ff;
                    color: #6b21a8;
                }
                
                .mode-token {
                    background: #dbeafe;
                    color: #1e40af;
                }
                
                .submission-text {
                    flex: 1;
                    color: #374151;
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
            
            <h1>"DualModeInput Demo"</h1>
            <p class="subtitle">"Smart input with automatic mode switching and state preservation"</p>
            
            <div class="demo-section">
                <h2>"Try Different Configurations"</h2>
                
                <h3>"Basic Dual Mode (no persistence)"</h3>
                <DualModeInput
                    on_submit=Some(handle_submit.clone())
                    show_history=false
                    persist_mode=false
                />
                
                <h3>"With History & Mode Persistence"</h3>
                <DualModeInput
                    on_submit=Some(handle_submit.clone())
                    show_history=true
                    persist_mode=true
                />
                
                <h3>"Starting in Token Mode"</h3>
                <DualModeInput
                    initial_mode=Some(InputMode::Tokenization)
                    on_submit=Some(handle_submit)
                    show_history=true
                    persist_mode=false
                />
                
                <div class="examples">
                    <strong>"Try these examples:"</strong>
                    <div class="example-list">
                        <div>"🤖 " <strong>"AI Mode:"</strong></div>
                        <div>"   • " <code>"How do I create an invoice?"</code></div>
                        <div>"   • " <code>"What's the total revenue this month?"</code></div>
                        <div>"   • Type " <code>"create"</code> " or " <code>"invoice"</code> " to auto-switch to token mode"</div>
                        <div></div>
                        <div>"⚡ " <strong>"Token Mode:"</strong></div>
                        <div>"   • Start with: " <code>"create"</code> ", " <code>"invoice"</code> ", " <code>"pay"</code> ", " <code>"report"</code></div>
                        <div>"   • Type " <code>"@"</code> " after a command to add entities"</div>
                        <div>"   • Select entities from the popup selector"</div>
                        <div>"   • Build complex commands with multiple entities"</div>
                        <div></div>
                        <div>"⌨️ " <strong>"Keyboard Shortcuts:"</strong></div>
                        <div>"   • " <code>"Cmd+Enter"</code> " to submit"</div>
                        <div>"   • " <code>"Cmd+↑/↓"</code> " to navigate history"</div>
                        <div>"   • " <code>"Esc"</code> " to clear current command or close selector"</div>
                    </div>
                </div>
                
                <Show when=move || !submissions.get().is_empty()>
                    <div class="submissions">
                        <strong>"Recent Submissions:"</strong>
                        <For
                            each=move || submissions.get()
                            key=|(mode, text)| format!("{}-{}", mode, text)
                            children=move |(mode, text)| {
                                view! {
                                    <div class="submission-item">
                                        <span class=move || format!("submission-mode mode-{}", mode.to_lowercase())>
                                            {mode}
                                        </span>
                                        <span class="submission-text">{text}</span>
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
                        <div class="feature-title">"🔄 Automatic Mode Switching"</div>
                        <div class="feature-desc">"Detects commands and switches to tokenization mode automatically"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"💾 State Preservation"</div>
                        <div class="feature-desc">"Optionally saves mode preference and command history to localStorage"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"🏷️ Visual Tokens"</div>
                        <div class="feature-desc">"Commands and entities displayed as colorful, removable tags"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"📜 Command History"</div>
                        <div class="feature-desc">"Navigate through previous commands with keyboard shortcuts"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"🎯 Smart Entity Selection"</div>
                        <div class="feature-desc">"Context-aware entity suggestions based on command type"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"⚡ Command Aliases"</div>
                        <div class="feature-desc">"Recognizes command aliases (e.g., 'bill' for 'invoice')"</div>
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