use leptos::prelude::*;
use leptos::logging;
use crate::types::*;
use crate::ml_engine::MLEngine;
use gloo_storage::{LocalStorage, Storage};
use chrono::Utc;
use std::collections::HashMap;

const STORAGE_KEY: &str = "smart_suggestions_ml_state";

#[component]
pub fn SmartSuggestions<F>(
    context: ReadSignal<String>,
    current_input: ReadSignal<String>,
    #[prop(optional)] config: Option<MLConfig>,
    #[prop(default = false)] enable_learning: bool,
    on_select: F,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView
where
    F: Fn(Suggestion) + Clone + 'static,
{
    let config = config.unwrap_or_default();
    
    // Load ML state from localStorage
    let initial_state = LocalStorage::get::<MLState>(STORAGE_KEY).unwrap_or_default();
    let (ml_engine, set_ml_engine) = signal(MLEngine::with_state(config.clone(), initial_state));
    
    // State
    let (selected_index, set_selected_index) = signal(0);
    let (hover_index, set_hover_index) = signal::<Option<usize>>(None);
    
    // Derive suggestions from input using Memo to avoid infinite loops
    let suggestions = Memo::new(move |_| {
        let input = current_input.get();
        let ctx = context.get();
        
        // Only generate suggestions if input has content
        if !input.is_empty() {
            let mut engine = ml_engine.get();
            let new_suggestions = engine.generate_suggestions(&ctx, &input);
            set_ml_engine.set(engine);
            new_suggestions
        } else {
            Vec::new()
        }
    });
    
    // Derive visibility from suggestions
    let is_visible = Memo::new(move |_| {
        !suggestions.get().is_empty()
    });
    
    // Use a signal to trigger selection handling
    let (pending_selection, set_pending_selection) = signal::<Option<Suggestion>>(None);
    
    // Handle selection when pending_selection changes
    Effect::new(move |prev: Option<Option<Suggestion>>| {
        if let Some(suggestion) = pending_selection.get() {
            // Only process if this is a new selection
            if prev.is_none() || prev != Some(Some(suggestion.clone())) {
                let suggestion_id = suggestion.id.clone();
                
                // Learn from the interaction if enabled
                if enable_learning {
                    let interaction = UserInteraction {
                        timestamp: Utc::now(),
                        context: context.get(),
                        input: current_input.get(),
                        selected_suggestion: Some(suggestion_id),
                        rejected_suggestions: suggestions.get()
                            .iter()
                            .filter(|s| s.id != suggestion.id)
                            .map(|s| s.id.clone())
                            .collect(),
                        metadata: HashMap::new(),
                    };
                    
                    set_ml_engine.update(|engine| {
                        engine.learn(interaction);
                        // Save state to localStorage
                        if let Ok(_) = LocalStorage::set(STORAGE_KEY, engine.get_state()) {
                            logging::log!("ML state saved");
                        }
                    });
                }
                
                // Call parent handler
                on_select(suggestion);
                
                // Clear the pending selection
                set_pending_selection.set(None);
                
                // Reset selected index for next time
                set_selected_index.set(0);
            }
        }
        
        pending_selection.get()
    });
    
    // Keyboard navigation - simpler approach without Effect
    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        if !is_visible.get() {
            return;
        }
        
        let key = ev.key();
        match key.as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                let max = suggestions.get().len().saturating_sub(1);
                set_selected_index.update(|idx| *idx = (*idx + 1).min(max));
            },
            "ArrowUp" => {
                ev.prevent_default();
                set_selected_index.update(|idx| *idx = idx.saturating_sub(1));
            },
            "Enter" => {
                ev.prevent_default();
                if let Some(suggestion) = suggestions.get().get(selected_index.get()) {
                    set_pending_selection.set(Some(suggestion.clone()));
                }
            },
            "Escape" => {
                ev.prevent_default();
                // Can't set is_visible since it's derived, but we can clear the input
                // which will hide suggestions
            },
            _ => {}
        }
    };
    
    view! {
        <div 
            class=move || {
                format!("ss-container {}", class.clone().unwrap_or_default())
            }
            on:keydown=move |ev| handle_keydown(ev)
            tabindex="0"
        >
            <Show when=move || is_visible.get() && !suggestions.get().is_empty()>
                <div class="ss-suggestions-panel">
                    <div class="ss-header">
                        <span class="ss-title">Smart Suggestions</span>
                        <span class="ss-badge">{move || suggestions.get().len()}</span>
                    </div>
                    
                    <div class="ss-suggestions-list">
                        <For
                            each=move || suggestions.get().into_iter().enumerate()
                            key=|(_, s)| s.id.clone()
                            children=move |(index, suggestion)| {
                                let is_selected = move || selected_index.get() == index;
                                let is_hovered = move || hover_index.get() == Some(index);
                                let suggestion_for_click = suggestion.clone();
                                
                                view! {
                                    <div
                                        class="ss-suggestion-item"
                                        class:ss-selected=is_selected
                                        class:ss-hovered=is_hovered
                                        on:click=move |_| set_pending_selection.set(Some(suggestion_for_click.clone()))
                                        on:mouseenter=move |_| {
                                            set_hover_index.set(Some(index));
                                            set_selected_index.set(index);
                                        }
                                        on:mouseleave=move |_| set_hover_index.set(None)
                                    >
                                        <div class="ss-suggestion-content">
                                            <div class="ss-suggestion-main">
                                                <span class="ss-suggestion-text">{suggestion.text.clone()}</span>
                                                <span class="ss-confidence" style=move || {
                                                    format!("opacity: {}", suggestion.confidence)
                                                }>
                                                    {format!("{:.0}%", suggestion.confidence * 100.0)}
                                                </span>
                                            </div>
                                            {suggestion.description.as_ref().map(|desc| {
                                                view! {
                                                    <div class="ss-suggestion-desc">{desc.clone()}</div>
                                                }
                                            })}
                                        </div>
                                        
                                        <div class="ss-category-badge" data-category=format!("{:?}", suggestion.category)>
                                            {match suggestion.category {
                                                SuggestionCategory::Command => "CMD",
                                                SuggestionCategory::Entity => "ENT",
                                                SuggestionCategory::Value => "VAL",
                                                SuggestionCategory::TimeBased => "TIME",
                                                SuggestionCategory::Contextual => "CTX",
                                                SuggestionCategory::Historical => "HIST",
                                                SuggestionCategory::Philippine => "PH",
                                            }}
                                        </div>
                                    </div>
                                }
                            }
                        />
                    </div>
                    
                    <div class="ss-footer">
                        <span class="ss-hint">"Use arrow keys to navigate, Enter to select, Esc to close"</span>
                        {move || if enable_learning {
                            view! {
                                <span class="ss-learning-indicator" title="Learning enabled">
                                    "🧠"
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>
                </div>
            </Show>
        </div>
    }
}