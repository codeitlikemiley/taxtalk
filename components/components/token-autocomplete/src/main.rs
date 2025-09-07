use leptos::prelude::*;
use taxtalk_token_autocomplete::TokenAutocomplete;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (value, set_value) = signal("Initial value".to_string());
    let (log, set_log) = signal(Vec::<String>::new());
    
    let handle_change = move |new_value: String| {
        set_value.set(new_value.clone());
        set_log.update(|l| l.push(format!("Changed to: {}", new_value)));
    };
    
    view! {
        <div class="demo-container">
            <h1>"TokenAutocomplete Component Demo"</h1>
            <p class="subtitle">"Interactive demonstration of the TokenAutocomplete component"</p>
            
            <div class="demo-section">
                <h2>"Basic Usage"</h2>
                <TokenAutocomplete
                    value=Signal::derive(move || value.get())
                    on_change=Callback::new(handle_change)
                />
            </div>
            
            <div class="demo-section">
                <h2>"Disabled State"</h2>
                <TokenAutocomplete
                    value=Signal::derive(move || "Disabled component".to_string())
                    on_change=Callback::new(|_| {})
                    disabled=true
                />
            </div>
            
            <div class="demo-section">
                <h2>"Event Log"</h2>
                <div style="font-family: monospace; font-size: 0.875rem;">
                    <For
                        each=move || log.get()
                        key=|entry| entry.clone()
                        children=move |entry| {
                            view! {
                                <div style="padding: 4px; background: #f0f0f0; margin: 2px 0;">
                                    {entry}
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}
