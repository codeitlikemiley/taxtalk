use leptos::prelude::*;
use leptos::ev::KeyboardEvent;
use crate::types::*;
use crate::search::CommandSearchEngine;
use crate::providers::default_commands;
use gloo_timers::future::TimeoutFuture;
use leptos::task::spawn_local;

/// Main command palette component
#[component]
pub fn CommandPalette<F>(
    /// Whether the palette is shown
    show: ReadSignal<bool>,
    /// Commands to display
    #[prop(optional)] commands: Option<Vec<Command>>,
    /// Configuration
    #[prop(optional)] config: Option<CommandPaletteConfig>,
    /// Callback when command is selected
    on_execute: F,
    /// Optional CSS class
    #[prop(optional)] class: Option<String>,
) -> impl IntoView
where
    F: Fn(CommandAction) + Clone + 'static,
{
    let config = config.unwrap_or_default();
    let commands = commands.unwrap_or_else(default_commands);
    
    // State
    let (search_query, set_search_query) = signal(String::new());
    let (selected_index, set_selected_index) = signal(0);
    let (is_nlp_mode, set_is_nlp_mode) = signal(false);
    let (nlp_result, set_nlp_result) = signal::<Option<NLPResult>>(None);
    let (is_parsing, set_is_parsing) = signal(false);
    let (show_internal, set_show_internal) = signal(false);
    
    // Search engine
    let search_engine = CommandSearchEngine::new(config.clone());
    
    // Recent commands (stored in localStorage)
    let (recent_commands, set_recent_commands) = signal::<Vec<String>>(
        load_recent_commands()
    );
    
    // Derive search results
    let search_results = Memo::new(move |_| {
        let query = search_query.get();
        
        if is_nlp_mode.get() {
            vec![]
        } else {
            let results = search_engine.search(&commands, &query);
            
            // If showing recent and query is empty, prepend recent commands
            if config.show_recent && query.is_empty() && !recent_commands.get().is_empty() {
                let recent_ids = recent_commands.get();
                let mut recent_results = Vec::new();
                
                for id in recent_ids.iter().take(config.recent_count) {
                    if let Some(cmd) = commands.iter().find(|c| c.id == *id) {
                        recent_results.push(SearchResult {
                            command: cmd.clone(),
                            score: 999.0, // High score for recent
                            matched_fields: vec!["recent".to_string()],
                        });
                    }
                }
                
                recent_results.extend(results);
                recent_results.truncate(config.max_results);
                recent_results
            } else {
                results
            }
        }
    });
    
    // Parse NLP when enabled
    Effect::new(move |_| {
        if !config.enable_nlp {
            return;
        }
        
        let query = search_query.get();
        
        // Switch modes based on query length and content
        if query.len() > 4 && !query.starts_with('/') {
            if !is_nlp_mode.get() {
                set_is_nlp_mode.set(true);
            }
            
            // Debounce NLP parsing
            set_is_parsing.set(true);
            let search_engine_clone = CommandSearchEngine::new(config.clone());
            
            spawn_local(async move {
                TimeoutFuture::new(config.search_debounce).await;
                
                let result = search_engine_clone.parse_nlp(&query);
                set_nlp_result.set(result);
                set_is_parsing.set(false);
            });
        } else if query.starts_with('/') {
            set_is_nlp_mode.set(false);
            set_search_query.set(query.trim_start_matches('/').to_string());
        } else if query.is_empty() {
            set_is_nlp_mode.set(false);
        }
    });
    
    // Handle keyboard navigation
    let handle_keydown = move |ev: KeyboardEvent| {
        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                let max = search_results.get().len().saturating_sub(1);
                set_selected_index.update(|idx| *idx = (*idx + 1).min(max));
            }
            "ArrowUp" => {
                ev.prevent_default();
                set_selected_index.update(|idx| *idx = idx.saturating_sub(1));
            }
            "Enter" => {
                ev.prevent_default();
                
                if is_nlp_mode.get() {
                    if let Some(result) = nlp_result.get() {
                        // Execute NLP command
                        on_execute(CommandAction::Custom(
                            serde_json::to_string(&result).unwrap_or_default()
                        ));
                        set_show_internal.set(false);
                        set_search_query.set(String::new());
                        set_selected_index.set(0);
                    }
                } else {
                    let results = search_results.get();
                    if let Some(result) = results.get(selected_index.get()) {
                        // Add to recent commands
                        add_to_recent(&result.command.id, &set_recent_commands);
                        
                        // Execute command
                        on_execute(result.command.action.clone());
                        set_show_internal.set(false);
                        set_search_query.set(String::new());
                        set_selected_index.set(0);
                    }
                }
            }
            "Escape" => {
                ev.prevent_default();
                set_show_internal.set(false);
                set_search_query.set(String::new());
                set_selected_index.set(0);
            }
            "Tab" => {
                ev.prevent_default();
                // Switch between categories if enabled
                if config.show_categories {
                    // Implementation for category navigation
                }
            }
            _ => {}
        }
    };
    
    // Watch for external show signal changes
    Effect::new(move |_| {
        set_show_internal.set(show.get());
    });
    
    view! {
        <Show when=move || show_internal.get()>
            <div class=format!("cp-overlay {}", class.clone().unwrap_or_default())
                on:click=move |_| set_show_internal.set(false)
            >
                <div class="cp-container" on:click=move |ev| ev.stop_propagation()>
                    // Search input
                    <div class="cp-search">
                        <input
                            type="text"
                            class="cp-search-input"
                            placeholder=if is_nlp_mode.get() {
                                "Describe what you want to do..."
                            } else {
                                "Search commands or type / to filter..."
                            }
                            prop:value=move || search_query.get()
                            on:input=move |ev| set_search_query.set(event_target_value(&ev))
                            on:keydown=handle_keydown
                            autofocus=true
                        />
                        <Show when=move || is_parsing.get()>
                            <div class="cp-search-spinner">
                                <div class="cp-spinner"></div>
                            </div>
                        </Show>
                    </div>
                    
                    // Results area
                    <div class="cp-results">
                        <Show when=move || is_nlp_mode.get()>
                            // NLP Mode
                            <div class="cp-nlp">
                                <Show when=move || nlp_result.get().is_some()>
                                    {move || {
                                        nlp_result.get().map(|result| {
                                            view! {
                                                <div class="cp-nlp-result">
                                                    <div class="cp-nlp-header">
                                                        <span class="cp-nlp-icon">{"🤖"}</span>
                                                        <span class="cp-nlp-title">I understand:</span>
                                                        <span class="cp-nlp-confidence">
                                                            {format!("{:.0}%", result.confidence * 100.0)}
                                                        </span>
                                                    </div>
                                                    
                                                    <div class="cp-nlp-params">
                                                        <For
                                                            each=move || result.parameters.clone()
                                                            key=|p| p.name.clone()
                                                            children=move |param| {
                                                                view! {
                                                                    <div class="cp-nlp-param">
                                                                        <span class="cp-nlp-param-name">
                                                                            {param.name}":"
                                                                        </span>
                                                                        <span class="cp-nlp-param-value">
                                                                            {param.value}
                                                                        </span>
                                                                    </div>
                                                                }
                                                            }
                                                        />
                                                    </div>
                                                    
                                                    <div class="cp-nlp-actions">
                                                        <button class="cp-nlp-action cp-nlp-action-primary"
                                                            on:click=move |_| {
                                                                if let Some(result) = nlp_result.get() {
                                                                    on_execute(CommandAction::Custom(
                                                                        serde_json::to_string(&result).unwrap_or_default()
                                                                    ));
                                                                    set_show_internal.set(false);
                        set_search_query.set(String::new());
                        set_selected_index.set(0);
                                                                }
                                                            }
                                                        >
                                                            "Execute"
                                                        </button>
                                                        <button class="cp-nlp-action"
                                                            on:click=move |_| {
                                                                set_nlp_result.set(None);
                                                                set_search_query.set(String::new());
                                                            }
                                                        >
                                                            "Try Again"
                                                        </button>
                                                    </div>
                                                </div>
                                            }
                                        })
                                    }}
                                </Show>
                                
                                <Show when=move || nlp_result.get().is_none() && !is_parsing.get()>
                                    <div class="cp-nlp-empty">
                                        <p>"Keep typing to describe what you want to do"</p>
                                        <p class="cp-nlp-hint">"Or press / to switch to command mode"</p>
                                    </div>
                                </Show>
                            </div>
                        </Show>
                        
                        <Show when=move || !is_nlp_mode.get()>
                            // Command Mode
                            <div class="cp-commands">
                                <Show when=move || config.show_recent && search_query.get().is_empty() && !recent_commands.get().is_empty()>
                                    <div class="cp-section">
                                        <div class="cp-section-title">Recent</div>
                                    </div>
                                </Show>
                                
                                <For
                                    each=move || search_results.get().into_iter().enumerate()
                                    key=|(_, r)| r.command.id.clone()
                                    children=move |(index, result)| {
                                        let is_selected = move || selected_index.get() == index;
                                        let command = result.command.clone();
                                        let action = command.action.clone();
                                        
                                        view! {
                                            <button
                                                class=move || {
                                                    if is_selected() {
                                                        "cp-command cp-command-selected"
                                                    } else {
                                                        "cp-command"
                                                    }
                                                }
                                                on:click=move |_| {
                                                    add_to_recent(&command.id, &set_recent_commands);
                                                    on_execute(action.clone());
                                                    set_show_internal.set(false);
                        set_search_query.set(String::new());
                        set_selected_index.set(0);
                                                }
                                                on:mouseenter=move |_| set_selected_index.set(index)
                                            >
                                                <div class="cp-command-content">
                                                    {command.icon.as_ref().map(|icon| {
                                                        view! { <span class="cp-command-icon">{icon.clone()}</span> }
                                                    })}
                                                    <div class="cp-command-text">
                                                        <div class="cp-command-label">{command.label}</div>
                                                        <div class="cp-command-desc">{command.description}</div>
                                                    </div>
                                                </div>
                                                
                                                <div class="cp-command-meta">
                                                    <span class="cp-command-category">
                                                        {command.category.to_string()}
                                                    </span>
                                                    <Show when=move || config.show_shortcuts && command.shortcut.is_some()>
                                                        {command.shortcut.as_ref().map(|shortcut| {
                                                            view! {
                                                                <kbd class="cp-command-shortcut">{shortcut.clone()}</kbd>
                                                            }
                                                        })}
                                                    </Show>
                                                </div>
                                            </button>
                                        }
                                    }
                                />
                                
                                <Show when=move || search_results.get().is_empty()>
                                    <div class="cp-empty">
                                        <p>"No commands found"</p>
                                        <p class="cp-empty-hint">"Try a different search term"</p>
                                    </div>
                                </Show>
                            </div>
                        </Show>
                    </div>
                    
                    // Footer
                    <div class="cp-footer">
                        <div class="cp-footer-hints">
                            <span class="cp-hint">
                                <kbd>"↑↓"</kbd> Navigate
                            </span>
                            <span class="cp-hint">
                                <kbd>"Enter"</kbd> Select
                            </span>
                            <span class="cp-hint">
                                <kbd>"Esc"</kbd> Close
                            </span>
                        </div>
                        <div class="cp-footer-mode">
                            {move || if is_nlp_mode.get() {
                                "Natural Language Mode"
                            } else {
                                "Command Mode"
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}

// Helper functions
fn load_recent_commands() -> Vec<String> {
    // Load from localStorage
    use gloo_storage::{LocalStorage, Storage};
    LocalStorage::get::<Vec<String>>("cp_recent_commands").unwrap_or_default()
}

fn add_to_recent(command_id: &str, set_recent: &WriteSignal<Vec<String>>) {
    use gloo_storage::{LocalStorage, Storage};
    
    set_recent.update(|recent| {
        // Remove if already exists
        recent.retain(|id| id != command_id);
        // Add to front
        recent.insert(0, command_id.to_string());
        // Limit to 10
        recent.truncate(10);
        // Save to localStorage
        let _ = LocalStorage::set("cp_recent_commands", &*recent);
    });
}