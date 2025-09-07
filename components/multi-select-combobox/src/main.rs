use leptos::prelude::*;
use multi_select_combobox::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    // Demo data
    let demo_entities = vec![
        Entity::new("client", "ABC Corporation")
            .with_description("Premium client since 2020"),
        Entity::new("client", "XYZ Industries")
            .with_description("Manufacturing partner"),
        Entity::new("client", "Tech Solutions Inc")
            .with_description("IT services provider"),
        Entity::new("client", "Global Traders")
            .with_description("Import/export business"),
        Entity::new("client", "Local Market Co")
            .with_description("Retail chain"),
        Entity::new("client", "Digital Agency")
            .with_description("Marketing services"),
        Entity::new("client", "Construction Corp")
            .with_description("Building contractor"),
        Entity::new("client", "Healthcare Plus")
            .with_description("Medical services"),
        Entity::new("client", "Education First")
            .with_description("Training provider"),
        Entity::new("client", "Finance Partners")
            .with_description("Investment firm"),
    ];
    
    // Generate large dataset for virtual scrolling test
    let mut large_dataset = demo_entities.clone();
    for i in 1..=100 {
        large_dataset.push(
            Entity::new("client", &format!("Test Client {}", i))
                .with_description(&format!("Auto-generated client #{}", i))
        );
    }
    
    // State for selected entities
    let (selected_basic, set_selected_basic) = signal::<Vec<Entity>>(vec![]);
    let (selected_limited, set_selected_limited) = signal::<Vec<Entity>>(vec![]);
    let (selected_large, set_selected_large) = signal::<Vec<Entity>>(vec![]);
    
    view! {
        <div style="padding: 2rem; max-width: 1200px; margin: 0 auto; font-family: system-ui, -apple-system, sans-serif;">
            <h1 style="font-size: 2rem; font-weight: bold; margin-bottom: 2rem; color: #1f2937;">
                "Multi-Select Combobox Component Demo"
            </h1>
            
            <div style="display: grid; gap: 3rem;">
                // Basic example
                <div style="background: white; padding: 2rem; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #374151;">
                        "Basic Multi-Select"
                    </h2>
                    <p style="color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;">
                        "Select multiple clients with no limit"
                    </p>
                    
                    <MultiSelectCombobox
                        placeholder="Select clients..."
                        _entity_type="client"
                        available_entities=demo_entities.clone()
                        on_change=move |entities| {
                            set_selected_basic.set(entities);
                        }
                    />
                    
                    <div style="margin-top: 1rem; padding: 1rem; background: #f9fafb; border-radius: 8px;">
                        <strong style="color: #4b5563;">Selected:</strong>
                        <div style="margin-top: 0.5rem;">
                            {move || {
                                let selected = selected_basic.get();
                                if selected.is_empty() {
                                    view! { <span style="color: #9ca3af;">"None"</span> }.into_any()
                                } else {
                                    view! {
                                        <ul style="list-style: none; padding: 0;">
                                            <For
                                                each=move || selected_basic.get()
                                                key=|e| e.id.clone()
                                                children=|entity| {
                                                    view! {
                                                        <li style="padding: 0.25rem 0; color: #374151;">
                                                            "• " {entity.name}
                                                        </li>
                                                    }
                                                }
                                            />
                                        </ul>
                                    }.into_any()
                                }
                            }}
                        </div>
                    </div>
                </div>
                
                // Limited selections example
                <div style="background: white; padding: 2rem; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #374151;">
                        "Limited Selections (Max 3)"
                    </h2>
                    <p style="color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;">
                        "You can only select up to 3 clients"
                    </p>
                    
                    <MultiSelectCombobox
                        placeholder="Select up to 3 clients..."
                        _entity_type="client"
                        max_selections=Some(3)
                        available_entities=demo_entities.clone()
                        on_change=move |entities| {
                            set_selected_limited.set(entities);
                        }
                    />
                    
                    <div style="margin-top: 1rem; padding: 1rem; background: #f9fafb; border-radius: 8px;">
                        <strong style="color: #4b5563;">Selected ({move || selected_limited.get().len()}/3):</strong>
                        <div style="margin-top: 0.5rem;">
                            {move || {
                                let selected = selected_limited.get();
                                if selected.is_empty() {
                                    view! { <span style="color: #9ca3af;">"None"</span> }.into_any()
                                } else {
                                    view! {
                                        <ul style="list-style: none; padding: 0;">
                                            <For
                                                each=move || selected_limited.get()
                                                key=|e| e.id.clone()
                                                children=|entity| {
                                                    view! {
                                                        <li style="padding: 0.25rem 0; color: #374151;">
                                                            "• " {entity.name}
                                                        </li>
                                                    }
                                                }
                                            />
                                        </ul>
                                    }.into_any()
                                }
                            }}
                        </div>
                    </div>
                </div>
                
                // Virtual scrolling example
                <div style="background: white; padding: 2rem; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #374151;">
                        "Virtual Scrolling (100+ items)"
                    </h2>
                    <p style="color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;">
                        "Efficiently handles large datasets with virtual scrolling"
                    </p>
                    
                    <MultiSelectCombobox
                        placeholder="Select from large dataset..."
                        _entity_type="client"
                        available_entities=large_dataset.clone()
                        on_change=move |entities| {
                            set_selected_large.set(entities);
                        }
                    />
                    
                    <div style="margin-top: 1rem; padding: 1rem; background: #f9fafb; border-radius: 8px;">
                        <strong style="color: #4b5563;">
                            {move || format!("{} items selected", selected_large.get().len())}
                        </strong>
                    </div>
                </div>
                
                // Pre-selected items
                <div style="background: white; padding: 2rem; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #374151;">
                        "With Initial Selections"
                    </h2>
                    <p style="color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;">
                        "Component starts with pre-selected items"
                    </p>
                    
                    <MultiSelectCombobox
                        placeholder="Add more clients..."
                        _entity_type="client"
                        initial_selections=vec![
                            Entity::new("client", "ABC Corporation")
                                .with_description("Premium client since 2020"),
                            Entity::new("client", "XYZ Industries")
                                .with_description("Manufacturing partner"),
                        ]
                        available_entities=demo_entities.clone()
                        on_change=move |entities| {
                            leptos::logging::log!("Updated selections: {:?}", entities);
                        }
                    />
                </div>
            </div>
            
            <div style="margin-top: 3rem; padding: 2rem; background: #f0f9ff; border-radius: 12px; border: 1px solid #bfdbfe;">
                <h3 style="font-size: 1.125rem; font-weight: 600; margin-bottom: 1rem; color: #1e40af;">
                    "Keyboard Shortcuts"
                </h3>
                <ul style="color: #3b82f6; line-height: 1.75;">
                    <li><strong>"Arrow Down/Up:"</strong> " Navigate through options"</li>
                    <li><strong>"Enter:"</strong> " Select highlighted option"</li>
                    <li><strong>"Escape:"</strong> " Close dropdown"</li>
                    <li><strong>"Backspace:"</strong> " Remove last selection (when input is empty)"</li>
                    <li><strong>"Type to search:"</strong> " Filter available options"</li>
                </ul>
            </div>
            
            <div style="margin-top: 2rem; padding: 2rem; background: #fef3c7; border-radius: 12px; border: 1px solid #fcd34d;">
                <h3 style="font-size: 1.125rem; font-weight: 600; margin-bottom: 1rem; color: #92400e;">
                    "Features"
                </h3>
                <ul style="color: #78350f; line-height: 1.75;">
                    <li>"✓ Virtual scrolling for large datasets (50+ items)"</li>
                    <li>"✓ Keyboard navigation support"</li>
                    <li>"✓ Optional selection limit"</li>
                    <li>"✓ Search/filter functionality"</li>
                    <li>"✓ Custom search API callback"</li>
                    <li>"✓ Initial selections support"</li>
                    <li>"✓ Accessible and responsive design"</li>
                    <li>"✓ Tag-based selection display"</li>
                </ul>
            </div>
        </div>
    }
}