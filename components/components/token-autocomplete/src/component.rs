use leptos::prelude::*;
use crate::types::*;
use crate::state::*;

/// The TokenAutocomplete component
#[component]
pub fn TokenAutocomplete(
    /// The current value
    #[prop(into)] value: Signal<String>,
    
    /// Callback when value changes
    on_change: Callback<String, ()>,
    
    /// Optional CSS class
    #[prop(optional)] class: Option<String>,
    
    /// Whether the component is disabled
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    // Component state management
    let (local_state, set_local_state) = signal(ComponentState::default());
    
    // Sync with external value
    Effect::new(move |_| {
        let new_value = value.get();
        set_local_state.update(|state| {
            state.value = new_value.clone();
        });
    });
    
    // Handle changes
    let handle_change = move |new_value: String| {
        set_local_state.update(|state| {
            state.value = new_value.clone();
        });
        on_change.run(new_value);
    };
    
    let css_class = format!(
        "tui-token-autocomplete {}",
        class.unwrap_or_default()
    );
    
    view! {
        <div class=css_class>
            <div class="tui-token-autocomplete-content">
                {move || local_state.get().value.clone()}
            </div>
            
            <Show
                when=move || !disabled
                fallback=|| ()
            >
                <button
                    class="tui-token-autocomplete-button"
                    on:click=move |_| handle_change("clicked".to_string())
                >
                    "Click Me"
                </button>
            </Show>
        </div>
    }
}
