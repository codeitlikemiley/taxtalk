use leptos::prelude::*;
use mixed_input::{MixedInput, EntityToken, MixedContent, create_entity};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (event_log, set_event_log) = signal(Vec::<String>::new());
    let (submissions, set_submissions) = signal(Vec::<String>::new());
    let (show_entity_selector, set_show_entity_selector) = signal(false);
    let (entity_filter, set_entity_filter) = signal(String::new());
    
    let log_event = move |msg: String| {
        set_event_log.update(|log| {
            log.push(format!("{}: {}", chrono::Local::now().format("%H:%M:%S"), msg));
            if log.len() > 10 {
                log.remove(0);
            }
        });
    };
    
    // Sample entities for demonstration
    let sample_entities = vec![
        create_entity("client", "Juan Dela Cruz"),
        create_entity("client", "ABC Corporation"),
        create_entity("supplier", "Office Depot"),
        create_entity("product", "Coffee Beans 1kg"),
        create_entity("invoice", "INV-2024-001"),
        create_entity("employee", "Maria Santos"),
    ];
    
    let handle_submit = move |content: MixedContent| {
        let display = content.to_display_string();
        log_event(format!("Submitted: {}", display));
        
        let entities = content.get_entities();
        let entity_count = entities.len();
        log_event(format!("Contains {} entities", entity_count));
        
        set_submissions.update(|subs| {
            subs.push(display);
            if subs.len() > 5 {
                subs.remove(0);
            }
        });
    };
    
    let handle_at_typed = move |position: usize, text: String| {
        log_event(format!("@ typed at position {} with text: '{}'", position, text));
        set_entity_filter.set(text);
        set_show_entity_selector.set(true);
    };
    
    // Filter entities based on current filter
    let filtered_entities = create_memo(move |_| {
        let filter = entity_filter.get();
        if filter.is_empty() {
            sample_entities.clone()
        } else {
            sample_entities
                .iter()
                .filter(|e| {
                    e.display_name.to_lowercase().contains(&filter.to_lowercase()) ||
                    e.entity_type.to_lowercase().contains(&filter.to_lowercase())
                })
                .cloned()
                .collect()
        }
    });
    
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
                
                .demo-grid {
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    gap: 2rem;
                }
                
                @media (max-width: 768px) {
                    .demo-grid {
                        grid-template-columns: 1fr;
                    }
                }
                
                .input-section h3 {
                    font-size: 1.125rem;
                    font-weight: 600;
                    margin-bottom: 1rem;
                    color: #374151;
                }
                
                .sample-entities {
                    margin-top: 1rem;
                    padding: 1rem;
                    background: #f9fafb;
                    border: 1px solid #e5e7eb;
                    border-radius: 0.5rem;
                }
                
                .entity-grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
                    gap: 0.5rem;
                    margin-top: 0.5rem;
                }
                
                .sample-entity {
                    padding: 0.5rem;
                    background: white;
                    border: 1px solid #d1d5db;
                    border-radius: 0.375rem;
                    font-size: 0.875rem;
                    cursor: pointer;
                    transition: all 0.15s;
                }
                
                .sample-entity:hover {
                    background: #f3f4f6;
                    border-color: #9ca3af;
                }
                
                .entity-type {
                    font-size: 0.75rem;
                    color: #6b7280;
                    text-transform: uppercase;
                }
                
                .entity-name {
                    font-weight: 500;
                    color: #111827;
                }
                
                .submissions {
                    margin-top: 1rem;
                    padding: 1rem;
                    background: #eff6ff;
                    border: 1px solid #bfdbfe;
                    border-radius: 0.5rem;
                }
                
                .submission-item {
                    padding: 0.5rem;
                    margin-bottom: 0.5rem;
                    background: white;
                    border: 1px solid #dbeafe;
                    border-radius: 0.375rem;
                    font-size: 0.875rem;
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
            
            <h1>"MixedInput Demo"</h1>
            <p class="subtitle">"Multi-entity coordination with mixed text and entity inputs"</p>
            
            <div class="demo-section">
                <h2>"Try Mixed Input"</h2>
                <div class="demo-grid">
                    <div class="input-section">
                        <h3>"Basic Mixed Input"</h3>
                        <MixedInput
                            placeholder=Some("Type a message with @mentions...".to_string())
                            on_submit=Some(handle_submit.clone())
                            on_at_typed=Some(handle_at_typed.clone())
                            show_helper=true
                        />
                        
                        <div class="sample-entities">
                            <strong>"Click to add entities:"</strong>
                            <div class="entity-grid">
                                {sample_entities.iter().map(|entity| {
                                    let entity_clone = entity.clone();
                                    view! {
                                        <div class="sample-entity">
                                            <div class="entity-type">{entity.entity_type.clone()}</div>
                                            <div class="entity-name">{entity.display_name.clone()}</div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    </div>
                    
                    <div class="input-section">
                        <h3>"With Character Limit"</h3>
                        <MixedInput
                            placeholder=Some("Limited to 200 characters...".to_string())
                            max_length=Some(200)
                            on_submit=Some(handle_submit.clone())
                            on_at_typed=Some(handle_at_typed.clone())
                            show_helper=true
                        />
                        
                        <h3 style="margin-top: 2rem;">"Pre-populated Example"</h3>
                        <MixedInput
                            initial_value=Some("Send invoice to ".to_string())
                            initial_entities=Some(vec![
                                create_entity("client", "Juan Dela Cruz"),
                            ])
                            placeholder=Some("Continue typing...".to_string())
                            on_submit=Some(handle_submit)
                            on_at_typed=Some(handle_at_typed)
                            show_helper=true
                        />
                    </div>
                </div>
                
                <Show when=move || !submissions.get().is_empty()>
                    <div class="submissions">
                        <strong>"Recent Submissions:"</strong>
                        <For
                            each=move || submissions.get()
                            key=|s| s.clone()
                            children=move |submission| {
                                view! {
                                    <div class="submission-item">{submission}</div>
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
                        <div class="feature-title">"🔗 Mixed Content"</div>
                        <div class="feature-desc">"Seamlessly mix plain text with entity references"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"🏷️ Entity Tags"</div>
                        <div class="feature-desc">"Visual entity tags with type-based colors"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"@ Mentions"</div>
                        <div class="feature-desc">"Type @ to trigger entity selection"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"📝 Rich Text Area"</div>
                        <div class="feature-desc">"Multi-line input with automatic resizing"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"✨ Smart Parsing"</div>
                        <div class="feature-desc">"Intelligent entity detection and serialization"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"📊 Character Limits"</div>
                        <div class="feature-desc">"Optional character counting with visual warnings"</div>
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