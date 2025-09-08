use leptos::prelude::*;
use quick_actions::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    // Different contexts to demonstrate
    let (context, set_context) = signal("amount".to_string());
    let (search_query, set_search_query) = signal(String::new());
    let (selected_actions, set_selected_actions) = signal::<Vec<QuickAction>>(vec![]);
    let (show_custom_input, set_show_custom_input) = signal(false);
    
    // Configuration variations
    let default_config = QuickActionsConfig::default();
    
    let lazy_config = QuickActionsConfig {
        initial_count: 3,
        enable_lazy_loading: true,
        show_shortcuts: true,
        enable_grouping: false,
        animation_duration: 200,
        enable_caching: true,
    };
    
    let grouped_config = QuickActionsConfig {
        initial_count: 10,
        enable_lazy_loading: false,
        show_shortcuts: false,
        enable_grouping: true,
        animation_duration: 150,
        enable_caching: false,
    };
    
    view! {
        <div style="padding: 2rem; max-width: 1200px; margin: 0 auto; font-family: system-ui, -apple-system, sans-serif;">
            <style>{include_str!("styles.css")}</style>
            
            <h1 style="font-size: 2rem; font-weight: bold; margin-bottom: 2rem; color: #1f2937;">
                "Quick Actions Component Demo"
            </h1>
            
            <div style="display: grid; gap: 2rem;">
                // Context Selector
                <div style="background: white; padding: 1.5rem; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #374151;">
                        "Select Context"
                    </h2>
                    <p style="color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;">
                        "Choose different contexts to see relevant quick actions"
                    </p>
                    
                    <div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
                        {["amount", "payment_method", "due_date", "tax", "discount"].into_iter().map(|ctx| {
                            let ctx_str = ctx.to_string();
                            let ctx_for_click = ctx_str.clone();
                            view! {
                                <button
                                    style=move || {
                                        if context.get() == ctx {
                                            "padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 0.5rem; cursor: pointer; font-weight: 500;"
                                        } else {
                                            "padding: 0.5rem 1rem; background: #e5e7eb; color: #374151; border: none; border-radius: 0.5rem; cursor: pointer; font-weight: 500;"
                                        }
                                    }
                                    on:click=move |_| set_context.set(ctx_for_click.clone())
                                >
                                    {ctx.replace("_", " ").chars().enumerate().map(|(i, c)| {
                                        if i == 0 || ctx.chars().nth(i.saturating_sub(1)) == Some('_') {
                                            c.to_uppercase().to_string()
                                        } else {
                                            c.to_string()
                                        }
                                    }).collect::<String>()}
                                </button>
                            }
                        }).collect_view()}
                    </div>
                </div>
                
                // Default Configuration Demo
                <div style="background: white; padding: 1.5rem; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #374151;">
                        "Default Configuration"
                    </h2>
                    <p style="color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;">
                        "Standard quick actions with all features enabled"
                    </p>
                    
                    <QuickActions
                        context=context
                        config=default_config
                        on_select=move |action| {
                            set_selected_actions.update(|list| list.push(action.clone()));
                            leptos::logging::log!("Selected: {:?}", action);
                        }
                    />
                </div>
                
                // Lazy Loading Demo
                <div style="background: white; padding: 1.5rem; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #374151;">
                        "With Lazy Loading"
                    </h2>
                    <p style="color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;">
                        "Shows 3 actions initially, load more on demand"
                    </p>
                    
                    <QuickActions
                        context=context
                        config=lazy_config
                        on_select=move |action| {
                            leptos::logging::log!("Lazy selected: {:?}", action);
                        }
                    />
                </div>
                
                // Grouped Actions Demo
                <div style="background: white; padding: 1.5rem; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #374151;">
                        "Grouped Actions"
                    </h2>
                    <p style="color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;">
                        "Actions organized in collapsible groups (works best with 'tax' context)"
                    </p>
                    
                    <QuickActions
                        context=context
                        config=grouped_config
                        on_select=move |action| {
                            leptos::logging::log!("Grouped selected: {:?}", action);
                        }
                    />
                </div>
                
                // Quick Action Bar Demo
                <div style="background: white; padding: 1.5rem; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #374151;">
                        "Quick Action Bar"
                    </h2>
                    <p style="color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;">
                        "Compact bar style for inline forms"
                    </p>
                    
                    {
                        let provider = PhilippineBusinessProvider::new();
                        let actions = provider.get_actions(&context.get(), "");
                        
                        view! {
                            <QuickActionBar
                                actions=actions.into_iter().take(5).collect()
                                title=Some(format!("Quick {} options", context.get()))
                                show_custom=true
                                on_select=move |action| {
                                    set_selected_actions.update(|list| list.push(action.clone()));
                                    leptos::logging::log!("Bar selected: {:?}", action);
                                }
                                on_custom=Some(set_show_custom_input.write_only())
                            />
                        }
                    }
                    
                    <Show when=move || show_custom_input.get()>
                        <div style="margin-top: 1rem; padding: 1rem; background: #f9fafb; border-radius: 0.5rem;">
                            <p style="color: #4b5563; font-size: 0.875rem;">
                                "Custom input modal would open here..."
                            </p>
                            <button
                                style="margin-top: 0.5rem; padding: 0.25rem 0.75rem; background: #6b7280; color: white; border: none; border-radius: 0.25rem; font-size: 0.875rem; cursor: pointer;"
                                on:click=move |_| set_show_custom_input.set(false)
                            >
                                "Close"
                            </button>
                        </div>
                    </Show>
                </div>
                
                // Selection History
                <div style="background: white; padding: 1.5rem; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #374151;">
                        "Selection History"
                    </h2>
                    <p style="color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;">
                        "Actions you've selected (affects sorting with caching enabled)"
                    </p>
                    
                    <div style="max-height: 200px; overflow-y: auto;">
                        {move || {
                            let actions = selected_actions.get();
                            if actions.is_empty() {
                                view! {
                                    <p style="color: #9ca3af; font-style: italic;">
                                        "No actions selected yet. Try clicking some quick actions above."
                                    </p>
                                }.into_any()
                            } else {
                                view! {
                                    <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                        <For
                                            each=move || selected_actions.get().into_iter().enumerate()
                                            key=|(i, a)| format!("{}_{}", i, a.id)
                                            children=|(_, action)| {
                                                view! {
                                                    <div style="display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem; background: #f9fafb; border-radius: 0.375rem;">
                                                        {action.icon.as_ref().map(|i| {
                                                            view! { <span>{i.clone()}</span> }
                                                        })}
                                                        <span style="flex: 1;">{action.label}</span>
                                                        <code style="font-size: 0.75rem; padding: 0.125rem 0.375rem; background: #e5e7eb; border-radius: 0.25rem;">
                                                            {action.value}
                                                        </code>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>
                
                // Features List
                <div style="background: #f0f9ff; padding: 1.5rem; border-radius: 12px; border: 1px solid #bfdbfe;">
                    <h3 style="font-size: 1.125rem; font-weight: 600; margin-bottom: 1rem; color: #1e40af;">
                        "Component Features"
                    </h3>
                    <ul style="color: #3b82f6; line-height: 1.75; font-size: 0.875rem;">
                        <li>"⚡ <strong>Lazy Loading:</strong> Load actions on demand for better performance"</li>
                        <li>"📦 <strong>Action Groups:</strong> Organize related actions in collapsible groups"</li>
                        <li>"⌨️ <strong>Keyboard Shortcuts:</strong> Quick selection with keyboard"</li>
                        <li>"💾 <strong>Usage Caching:</strong> Remembers frequently used actions"</li>
                        <li>"🎨 <strong>Color Coding:</strong> Visual distinction for action types"</li>
                        <li>"📱 <strong>Responsive:</strong> Adapts to mobile screens"</li>
                        <li>"🇵🇭 <strong>Philippine Context:</strong> VAT, EWT, BIR-specific actions"</li>
                        <li>"🔍 <strong>Search/Filter:</strong> Find actions quickly (can be integrated)"</li>
                    </ul>
                </div>
                
                // Usage Tips
                <div style="background: #fef3c7; padding: 1.5rem; border-radius: 12px; border: 1px solid #fcd34d;">
                    <h3 style="font-size: 1.125rem; font-weight: 600; margin-bottom: 1rem; color: #92400e;">
                        "Usage Tips"
                    </h3>
                    <ul style="color: #78350f; line-height: 1.75; font-size: 0.875rem;">
                        <li>"• Try different contexts to see context-specific actions"</li>
                        <li>"• 'Tax' context shows grouped actions (VAT, EWT)"</li>
                        <li>"• 'Amount' shows common peso amounts"</li>
                        <li>"• 'Payment Method' includes GCash, Maya, etc."</li>
                        <li>"• 'Due Date' calculates relative dates"</li>
                        <li>"• Actions are sorted by usage frequency when caching is enabled"</li>
                        <li>"• Lazy loading helps with performance on large action sets"</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}