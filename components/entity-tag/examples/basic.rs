use leptos::prelude::*;
use taxtalk_entity_tag::EntityTag;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(BasicExample);
}

#[component]
fn BasicExample() -> impl IntoView {
    let (value, set_value) = signal("Hello World".to_string());
    
    view! {
        <div style="padding: 20px;">
            <h1>"Basic EntityTag Example"</h1>
            
            <EntityTag
                value=Signal::derive(move || value.get())
                on_change=Callback::new(move |v| set_value.set(v))
            />
            
            <p>"Current value: " {move || value.get()}</p>
        </div>
    }
}
