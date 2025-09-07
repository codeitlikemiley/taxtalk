use leptos::prelude::*;
use crate::types::*;
use crate::state::*;

/// The EntityTag component - displays an entity token as a removable tag
#[component]
pub fn EntityTag(
    /// The entity token to display
    token: Signal<Option<EntityToken>>,
    
    /// Callback when remove button is clicked
    on_remove: Callback<String, ()>,
    
    /// Optional CSS class
    #[prop(optional)] class: Option<String>,
    
    /// Whether the component is disabled
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    // Component state management with Crux integration
    let (state, set_state) = signal(EntityTagState::default());
    
    // Sync token changes with state
    Effect::new(move |_| {
        if let Some(token) = token.get() {
            set_state.update(|s| {
                s.token = Some(token);
                s.is_removing = false;
            });
        } else {
            set_state.update(|s| {
                s.token = None;
                s.is_removing = false;
            });
        }
    });
    
    // Determine styling based on entity type
    let get_color_class = move |entity_type: &str| {
        match entity_type {
            "client" => "tui-et-client",
            "supplier" => "tui-et-supplier",
            "product" => "tui-et-product",
            "employee" => "tui-et-employee",
            "invoice" => "tui-et-invoice",
            "payment" => "tui-et-payment",
            _ => "tui-et-default",
        }
    };
    
    let custom_class = class.unwrap_or_default();
    
    view! {
        {move || {
            let current_state = state.get();
            if let Some(token) = current_state.token {
                let token_id = token.id.clone();
                let display_name = token.display_name.clone();
                let entity_type = token.entity_type.clone();
                let color_class = get_color_class(&entity_type);
                let tag_class = format!("tui-et-tag {} {}", color_class, custom_class);
                
                view! {
                    <span class=tag_class>
                        <span class="tui-et-label">{display_name}</span>
                        {move || {
                            if !disabled && !state.get().is_removing {
                                let token_id = token_id.clone();
                                view! {
                                    <button
                                        class="tui-et-remove"
                                        on:click=move |_| {
                                            set_state.update(|s| s.is_removing = true);
                                            on_remove.run(token_id.clone());
                                        }
                                        disabled=move || state.get().is_removing
                                        aria-label="Remove tag"
                                    >
                                        "×"
                                    </button>
                                }.into_any()
                            } else {
                                ().into_any()
                            }
                        }}
                    </span>
                }.into_any()
            } else {
                view! { <span style="display: none;"></span> }.into_any()
            }
        }}
    }
}

