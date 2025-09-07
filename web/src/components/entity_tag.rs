use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityToken {
    pub id: String,
    pub entity_type: String,
    pub display_name: String,
    pub metadata: Option<serde_json::Value>,
}

#[component]
pub fn EntityTag(
    token: EntityToken,
    on_remove: WriteSignal<Option<String>>,
) -> impl IntoView {
    let token_id = token.id.clone();
    let display_name = token.display_name.clone();
    let entity_type = token.entity_type.clone();
    
    // Determine color based on entity type
    let bg_color = match entity_type.as_str() {
        "client" => "bg-blue-100 text-blue-800 border-blue-200",
        "supplier" => "bg-green-100 text-green-800 border-green-200",
        "product" => "bg-purple-100 text-purple-800 border-purple-200",
        "employee" => "bg-orange-100 text-orange-800 border-orange-200",
        "invoice" => "bg-indigo-100 text-indigo-800 border-indigo-200",
        "payment" => "bg-pink-100 text-pink-800 border-pink-200",
        _ => "bg-gray-100 text-gray-800 border-gray-200",
    };
    
    view! {
        <span class={format!("inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border {}", bg_color)}>
            <span class="mr-1">{display_name}</span>
            <button
                class="ml-1 hover:opacity-70 focus:outline-none"
                on:click=move |_| on_remove.set(Some(token_id.clone()))
            >
                <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"/>
                </svg>
            </button>
        </span>
    }
}