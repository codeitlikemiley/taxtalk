use leptos::prelude::*;
use taxtalk_entity_tag::{EntityTag, EntityToken};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    // Sample tokens for demonstration
    let initial_tokens = vec![
        EntityToken {
            id: "1".to_string(),
            entity_type: "client".to_string(),
            display_name: "ABC Corporation".to_string(),
            metadata: None,
        },
        EntityToken {
            id: "2".to_string(),
            entity_type: "supplier".to_string(),
            display_name: "Tech Solutions Inc".to_string(),
            metadata: None,
        },
        EntityToken {
            id: "3".to_string(),
            entity_type: "product".to_string(),
            display_name: "Widget Pro 2000".to_string(),
            metadata: None,
        },
    ];
    
    let (tokens, set_tokens) = signal(initial_tokens);
    let (event_log, set_event_log) = signal(Vec::<String>::new());
    
    let log_event = move |msg: String| {
        set_event_log.update(|log| {
            log.push(msg);
            // Keep only last 10 events
            if log.len() > 10 {
                log.remove(0);
            }
        });
    };
    
    let handle_remove = move |id: String| {
        set_tokens.update(|tokens| {
            if let Some(token) = tokens.iter().find(|t| t.id == id) {
                log_event(format!("Removed: {}", token.display_name));
            }
            tokens.retain(|t| t.id != id);
        });
    };
    
    view! {
        <div class="demo-container">
            <h1>"EntityTag Component Demo"</h1>
            <p class="subtitle">"Interactive demonstration of the EntityTag component for token resolution"</p>
            
            <div class="demo-section">
                <h2>"Active Tags"</h2>
                <div style="display: flex; flex-wrap: wrap; gap: 0.5rem; min-height: 60px; padding: 1rem; background: #f8f9fa; border-radius: 8px;">
                    <Show
                        when=move || !tokens.get().is_empty()
                        fallback=|| view! { <p style="color: #6c757d; margin: auto;">"No tags available"</p> }
                    >
                        <For
                            each=move || tokens.get()
                            key=|token| token.id.clone()
                            children=move |token| {
                                let token_signal = Signal::derive(move || Some(token.clone()));
                                let remove_callback = Callback::new(move |id: String| handle_remove(id));
                                
                                view! {
                                    <EntityTag
                                        token=token_signal
                                        on_remove=remove_callback
                                    />
                                }
                            }
                        />
                    </Show>
                </div>
            </div>
            
            <div class="demo-section">
                <h2>"Event Log"</h2>
                <div style="font-family: monospace; font-size: 0.875rem; background: #1e1e1e; color: #d4d4d4; padding: 1rem; border-radius: 4px; min-height: 100px;">
                    <Show
                        when=move || !event_log.get().is_empty()
                        fallback=|| view! { <p style="color: #6c757d;">"No events yet..."</p> }
                    >
                        <For
                            each=move || event_log.get()
                            key=|entry| entry.clone()
                            children=move |entry| {
                                view! {
                                    <div style="padding: 2px 0;">
                                        {entry}
                                    </div>
                                }
                            }
                        />
                    </Show>
                </div>
            </div>
        </div>
    }
}
