use leptos::prelude::*;
use crate::types::*;
use crate::providers::PhilippineBusinessProvider;
use gloo_storage::{LocalStorage, Storage};
use std::collections::HashMap;

const USAGE_STATS_KEY: &str = "quick_actions_usage";

/// Main quick actions component with lazy loading support
#[component]
pub fn QuickActions<F>(
    /// Context for determining which actions to show
    context: ReadSignal<String>,
    /// Optional search query to filter actions
    #[prop(optional)] query: Option<ReadSignal<String>>,
    /// Configuration
    #[prop(optional)] config: Option<QuickActionsConfig>,
    /// Callback when action is selected
    on_select: F,
    /// Optional CSS class
    #[prop(optional)] class: Option<String>,
) -> impl IntoView
where
    F: Fn(QuickAction) + Clone + 'static,
{
    let config = config.unwrap_or_default();
    let provider = PhilippineBusinessProvider::new();
    
    // State
    let (shown_count, set_shown_count) = signal(config.initial_count);
    let (expanded_groups, set_expanded_groups) = signal::<HashMap<String, bool>>(HashMap::new());
    let (usage_stats, set_usage_stats) = signal(
        LocalStorage::get::<ActionUsageStats>(USAGE_STATS_KEY).unwrap_or_default()
    );
    
    // Derive actions based on context and query
    let actions = Memo::new(move |_| {
        let ctx = context.get();
        let q = query.as_ref().map(|s| s.get()).unwrap_or_default();
        
        let mut actions = provider.get_actions(&ctx, &q);
        
        // Sort by usage frequency if caching is enabled
        if config.enable_caching {
            let stats = usage_stats.get();
            actions.sort_by(|a, b| {
                let a_count = stats.selection_count.get(&a.id).unwrap_or(&0);
                let b_count = stats.selection_count.get(&b.id).unwrap_or(&0);
                b_count.cmp(a_count)
            });
        }
        
        actions
    });
    
    // Derive groups if enabled
    let groups = Memo::new(move |_| {
        if config.enable_grouping {
            let ctx = context.get();
            let mut groups = provider.get_groups(&ctx);
            groups.sort_by_key(|g| -g.priority);
            groups
        } else {
            vec![]
        }
    });
    
    // Derive visible actions (for lazy loading)
    let visible_actions = Memo::new(move |_| {
        let all_actions = actions.get();
        let count = shown_count.get();
        
        if config.enable_lazy_loading {
            all_actions.into_iter().take(count).collect::<Vec<_>>()
        } else {
            all_actions
        }
    });
    
    // Check if there are more actions to load
    let has_more = Memo::new(move |_| {
        config.enable_lazy_loading && actions.get().len() > shown_count.get()
    });
    
    // Handle action selection
    let handle_select = move |action: QuickAction| {
        // Update usage stats
        if config.enable_caching {
            set_usage_stats.update(|stats| {
                let count = stats.selection_count.entry(action.id.clone()).or_insert(0);
                *count += 1;
                stats.last_used.insert(action.id.clone(), chrono::Utc::now());
                
                // Save to localStorage
                let _ = LocalStorage::set(USAGE_STATS_KEY, &*stats);
            });
        }
        
        // Call parent handler
        on_select(action);
    };
    
    // Handle load more
    let handle_load_more = move |_| {
        set_shown_count.update(|c| *c += config.initial_count);
    };
    
    // Handle group toggle
    let handle_group_toggle = move |group_id: String| {
        set_expanded_groups.update(|groups| {
            let is_expanded = groups.get(&group_id).unwrap_or(&false);
            groups.insert(group_id, !is_expanded);
        });
    };
    
    view! {
        <div class=move || {
            format!("qa-container {}", class.clone().unwrap_or_default())
        }>
            <Show when=move || !visible_actions.get().is_empty() || !groups.get().is_empty()>
                // Grouped actions
                <Show when=move || config.enable_grouping && !groups.get().is_empty()>
                    <div class="qa-groups">
                        <For
                            each=move || groups.get()
                            key=|g| g.id.clone()
                            children=move |group| {
                                let group_id = group.id.clone();
                                let group_id_for_toggle = group_id.clone();
                                let is_expanded = move || {
                                    expanded_groups.get().get(&group_id).unwrap_or(&false).clone()
                                };
                                
                                view! {
                                    <div class="qa-group">
                                        <button
                                            class="qa-group-header"
                                            on:click=move |_| handle_group_toggle(group_id_for_toggle.clone())
                                        >
                                            {group.icon.as_ref().map(|i| {
                                                view! { <span class="qa-group-icon">{i.clone()}</span> }
                                            })}
                                            <span class="qa-group-label">{group.label.clone()}</span>
                                            <span class="qa-group-toggle">
                                                {move || if is_expanded() { "▼" } else { "▶" }}
                                            </span>
                                        </button>
                                        
                                        <Show when=is_expanded>
                                            <div class="qa-group-actions">
                                                <For
                                                    each=move || group.actions.clone()
                                                    key=|a| a.id.clone()
                                                    children=move |action| {
                                                        let action_for_select = action.clone();
                                                        view! {
                                                            <QuickActionButton
                                                                action=action
                                                                show_shortcuts=config.show_shortcuts
                                                                on_click=move |_| handle_select(action_for_select.clone())
                                                            />
                                                        }
                                                    }
                                                />
                                            </div>
                                        </Show>
                                    </div>
                                }
                            }
                        />
                    </div>
                </Show>
                
                // Ungrouped actions
                <div class="qa-actions">
                    <For
                        each=move || visible_actions.get()
                        key=|a| a.id.clone()
                        children=move |action| {
                            let action_for_select = action.clone();
                            view! {
                                <QuickActionButton
                                    action=action
                                    show_shortcuts=config.show_shortcuts
                                    on_click=move |_| handle_select(action_for_select.clone())
                                />
                            }
                        }
                    />
                </div>
                
                // Load more button
                <Show when=has_more>
                    <button
                        class="qa-load-more"
                        on:click=handle_load_more
                    >
                        <span class="qa-load-more-icon">"⬇"</span>
                        <span>"Load More Actions"</span>
                        <span class="qa-load-more-count">
                            {move || format!("({} more)", actions.get().len() - shown_count.get())}
                        </span>
                    </button>
                </Show>
            </Show>
            
            // Empty state
            <Show when=move || visible_actions.get().is_empty() && groups.get().is_empty()>
                <div class="qa-empty">
                    <span class="qa-empty-icon">"🔍"</span>
                    <p class="qa-empty-text">"No quick actions available for this context"</p>
                </div>
            </Show>
        </div>
    }
}

/// Individual quick action button
#[component]
fn QuickActionButton<F>(
    action: QuickAction,
    show_shortcuts: bool,
    on_click: F,
) -> impl IntoView
where
    F: Fn(web_sys::MouseEvent) + 'static,
{
    let color = action.color.clone().unwrap_or_else(|| "gray".to_string());
    let button_class = format!("qa-action qa-action-{}", color);
    
    view! {
        <button
            class=button_class
            title=action.description.clone().unwrap_or_default()
            on:click=on_click
        >
            {action.icon.as_ref().map(|icon| {
                view! { <span class="qa-action-icon">{icon.clone()}</span> }
            })}
            <span class="qa-action-label">{action.label.clone()}</span>
            <Show when=move || show_shortcuts && action.keyboard_shortcut.is_some()>
                {action.keyboard_shortcut.as_ref().map(|shortcut| {
                    view! {
                        <kbd class="qa-action-shortcut">{shortcut.clone()}</kbd>
                    }
                })}
            </Show>
        </button>
    }
}

/// Quick action bar for selecting from a list of actions
#[component]
pub fn QuickActionBar<F>(
    actions: Vec<QuickAction>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] show_custom: bool,
    on_select: F,
    #[prop(optional)] on_custom: Option<WriteSignal<bool>>,
) -> impl IntoView
where
    F: Fn(QuickAction) + Clone + 'static,
{
    view! {
        <div class="qa-bar">
            {title.map(|t| {
                view! {
                    <div class="qa-bar-title">{t}</div>
                }
            })}
            
            <div class="qa-bar-actions">
                <For
                    each=move || actions.clone()
                    key=|a| a.id.clone()
                    children=move |action| {
                        let action_for_select = action.clone();
                        let on_select = on_select.clone();
                        
                        view! {
                            <button
                                class=format!("qa-bar-action qa-bar-action-{}", 
                                    action.color.clone().unwrap_or_else(|| "gray".to_string()))
                                on:click=move |_| on_select(action_for_select.clone())
                            >
                                {action.icon.as_ref().map(|i| {
                                    view! { <span class="qa-bar-icon">{i.clone()}</span> }
                                })}
                                <span>{action.label.clone()}</span>
                            </button>
                        }
                    }
                />
                
                <Show when=move || show_custom && on_custom.is_some()>
                    {on_custom.map(|signal| {
                        view! {
                            <button
                                class="qa-bar-action qa-bar-action-custom"
                                on:click=move |_| signal.set(true)
                            >
                                <span class="qa-bar-icon">"✏️"</span>
                                <span>"Custom"</span>
                            </button>
                        }
                    })}
                </Show>
            </div>
        </div>
    }
}