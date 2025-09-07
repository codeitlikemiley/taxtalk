use leptos::prelude::*;
use entity_combobox::{EntityCombobox, Entity, EntityType, EntityComboboxConfig};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (selected_entity, set_selected_entity) = signal::<Option<Entity>>(None);
    let (show_combobox, set_show_combobox) = signal(false);
    let (entity_type, set_entity_type) = signal(EntityType::Client);
    let (event_log, set_event_log) = signal(Vec::<String>::new());
    
    let log_event = move |msg: String| {
        set_event_log.update(|log| {
            log.push(format!("{}: {}", chrono::Local::now().format("%H:%M:%S"), msg));
            if log.len() > 10 {
                log.remove(0);
            }
        });
    };
    
    let handle_select = move |entity: Entity| {
        log_event(format!("Selected: {} ({})", entity.name, entity.entity_type));
        set_selected_entity.set(Some(entity));
        set_show_combobox.set(false);
    };
    
    let handle_cancel = move || {
        log_event("Selection cancelled".to_string());
        set_show_combobox.set(false);
    };
    
    let config = EntityComboboxConfig {
        max_suggestions: 10,
        debounce_ms: 300,
        enable_keyboard_nav: true,
        show_loading: true,
        cache_results: true,
        cache_expiry_secs: 300,
    };
    
    view! {
        <div class="demo-container">
            <style>{include_str!("styles.css")}</style>
            <style>{r#"
                .demo-container {
                    max-width: 1200px;
                    margin: 0 auto;
                    padding: 2rem;
                    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
                }
                
                h1 {
                    font-size: 2rem;
                    font-weight: 700;
                    margin-bottom: 0.5rem;
                    color: #111827;
                }
                
                .subtitle {
                    color: #6b7280;
                    margin-bottom: 2rem;
                }
                
                .demo-section {
                    margin-bottom: 2rem;
                    background: white;
                    padding: 1.5rem;
                    border-radius: 0.5rem;
                    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
                }
                
                .demo-section h2 {
                    font-size: 1.25rem;
                    font-weight: 600;
                    margin-bottom: 1rem;
                    color: #374151;
                }
                
                .controls {
                    display: flex;
                    gap: 1rem;
                    margin-bottom: 1rem;
                }
                
                .entity-type-selector {
                    display: flex;
                    gap: 0.5rem;
                    margin-bottom: 1rem;
                }
                
                .entity-type-btn {
                    padding: 0.5rem 1rem;
                    border: 1px solid #d1d5db;
                    border-radius: 0.375rem;
                    background: white;
                    cursor: pointer;
                    transition: all 0.15s;
                }
                
                .entity-type-btn:hover {
                    background: #f9fafb;
                }
                
                .entity-type-btn-active {
                    background: #3b82f6;
                    color: white;
                    border-color: #3b82f6;
                }
                
                .open-btn {
                    padding: 0.625rem 1.25rem;
                    font-size: 0.875rem;
                    color: white;
                    background: #3b82f6;
                    border: none;
                    border-radius: 0.375rem;
                    cursor: pointer;
                    transition: all 0.15s;
                }
                
                .open-btn:hover {
                    background: #2563eb;
                }
                
                .selected-entity {
                    padding: 1rem;
                    background: #f0f9ff;
                    border: 1px solid #bfdbfe;
                    border-radius: 0.375rem;
                }
                
                .selected-entity-header {
                    display: flex;
                    align-items: center;
                    gap: 0.75rem;
                    margin-bottom: 0.5rem;
                }
                
                .selected-entity-avatar {
                    width: 3rem;
                    height: 3rem;
                    border-radius: 0.5rem;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-weight: 700;
                    color: white;
                }
                
                .selected-entity-info {
                    flex: 1;
                }
                
                .selected-entity-name {
                    font-size: 1.125rem;
                    font-weight: 600;
                    color: #111827;
                }
                
                .selected-entity-type {
                    font-size: 0.875rem;
                    color: #6b7280;
                }
                
                .selected-entity-desc {
                    font-size: 0.875rem;
                    color: #4b5563;
                }
                
                .selected-entity-meta {
                    margin-top: 0.75rem;
                    padding: 0.75rem;
                    background: white;
                    border-radius: 0.25rem;
                    font-family: monospace;
                    font-size: 0.75rem;
                    color: #374151;
                }
                
                .empty-state {
                    padding: 2rem;
                    text-align: center;
                    color: #6b7280;
                    background: #f9fafb;
                    border-radius: 0.375rem;
                }
                
                .event-log {
                    background: #1e1e1e;
                    color: #d4d4d4;
                    padding: 1rem;
                    border-radius: 0.375rem;
                    font-family: monospace;
                    font-size: 0.875rem;
                    max-height: 200px;
                    overflow-y: auto;
                }
                
                .log-entry {
                    padding: 0.125rem 0;
                }
                
                .features {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
                    gap: 1rem;
                    margin-top: 1rem;
                }
                
                .feature {
                    padding: 1rem;
                    background: #f9fafb;
                    border-radius: 0.375rem;
                }
                
                .feature-title {
                    font-weight: 600;
                    color: #111827;
                    margin-bottom: 0.25rem;
                }
                
                .feature-desc {
                    font-size: 0.875rem;
                    color: #6b7280;
                }
            "#}</style>
            
            <h1>"EntityCombobox Component Demo"</h1>
            <p class="subtitle">"Searchable dropdown with entity selection and API caching"</p>
            
            <div class="demo-section">
                <h2>"Try It Out"</h2>
                
                <div class="entity-type-selector">
                    <For
                        each=|| vec![
                            EntityType::Client,
                            EntityType::Supplier,
                            EntityType::Product,
                            EntityType::Employee,
                        ]
                        key=|t| t.as_str().to_string()
                        children=move |etype| {
                            let is_active = move || entity_type.get() == etype;
                            view! {
                                <button
                                    class={move || if is_active() {
                                        "entity-type-btn entity-type-btn-active"
                                    } else {
                                        "entity-type-btn"
                                    }}
                                    on:click=move |_| {
                                        set_entity_type.set(etype.clone());
                                        log_event(format!("Changed entity type to: {}", etype.display_name()));
                                    }
                                >
                                    {etype.display_name()}
                                </button>
                            }
                        }
                    />
                </div>
                
                <div class="controls">
                    <button
                        class="open-btn"
                        on:click=move |_| {
                            set_show_combobox.set(true);
                            log_event(format!("Opening {} selector", entity_type.get().display_name()));
                        }
                    >
                        {move || format!("Select {}", entity_type.get().display_name())}
                    </button>
                </div>
                
                <Show
                    when=move || selected_entity.get().is_some()
                    fallback=|| view! {
                        <div class="empty-state">
                            "No entity selected yet. Click the button above to select one."
                        </div>
                    }
                >
                    {move || {
                        let entity = selected_entity.get().unwrap();
                        let color = EntityType::from(entity.entity_type.clone()).color();
                        
                        view! {
                            <div class="selected-entity">
                                <div class="selected-entity-header">
                                    <div class={format!("selected-entity-avatar ec-avatar-{}", color)}>
                                        {entity.name.chars().next().unwrap_or('?').to_uppercase().to_string()}
                                    </div>
                                    <div class="selected-entity-info">
                                        <div class="selected-entity-name">{entity.name.clone()}</div>
                                        <div class="selected-entity-type">
                                            {format!("Type: {}", entity.entity_type)}
                                        </div>
                                    </div>
                                </div>
                                {entity.description.as_ref().map(|desc| {
                                    view! {
                                        <div class="selected-entity-desc">{desc.clone()}</div>
                                    }
                                })}
                                {entity.metadata.as_ref().map(|meta| {
                                    view! {
                                        <div class="selected-entity-meta">
                                            <strong>"Metadata:"</strong><br/>
                                            {serde_json::to_string_pretty(meta).unwrap_or_default()}
                                        </div>
                                    }
                                })}
                            </div>
                        }
                    }}
                </Show>
            </div>
            
            <Show when=move || show_combobox.get()>
                <EntityCombobox
                    entity_type=entity_type.get()
                    prompt=Some(format!("Search and select a {} from the list below", entity_type.get().as_str()))
                    on_select=Callback::new(handle_select)
                    on_cancel=Some(Callback::new(handle_cancel))
                    config=Some(config.clone())
                    api_url="mock".to_string()
                />
            </Show>
            
            <div class="demo-section">
                <h2>"Features"</h2>
                <div class="features">
                    <div class="feature">
                        <div class="feature-title">"🔍 Smart Search"</div>
                        <div class="feature-desc">"Search across entity names and descriptions with debouncing"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"💾 API Caching"</div>
                        <div class="feature-desc">"Intelligent caching reduces API calls and improves performance"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"⌨️ Keyboard Navigation"</div>
                        <div class="feature-desc">"Navigate with arrow keys, select with Enter, cancel with Escape"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"🎨 Entity Types"</div>
                        <div class="feature-desc">"Support for clients, suppliers, products, employees, and more"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"⚡ Performance"</div>
                        <div class="feature-desc">"Optimized rendering with virtual scrolling for large datasets"</div>
                    </div>
                    <div class="feature">
                        <div class="feature-title">"📱 Responsive"</div>
                        <div class="feature-desc">"Works seamlessly on desktop and mobile devices"</div>
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