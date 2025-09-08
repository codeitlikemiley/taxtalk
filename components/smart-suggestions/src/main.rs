use leptos::prelude::*;
use leptos::logging;
use smart_suggestions::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    // Different contexts and inputs to demonstrate suggestions
    let (context, set_context) = signal("invoice".to_string());
    let (input, set_input) = signal(String::new());
    let (selected_suggestions, set_selected_suggestions) = signal::<Vec<Suggestion>>(vec![]);
    
    // Custom ML config for demo
    let ml_config = MLConfig {
        max_suggestions: 8,
        min_confidence: 0.2,
        learning_rate: 0.15,
        pattern_threshold: 0.4,
        history_limit: 100,
        enable_time_patterns: true,
        enable_philippine_context: true,
    };
    
    view! {
        <div style="padding: 2rem; max-width: 1200px; margin: 0 auto; font-family: system-ui, -apple-system, sans-serif;">
            <h1 style="font-size: 2rem; font-weight: bold; margin-bottom: 2rem; color: #1f2937;">
                "Smart Suggestions Component Demo"
            </h1>
            
            <div style="display: grid; gap: 3rem;">
                // Context selector
                <div style="background: white; padding: 2rem; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #374151;">
                        "Context Selector"
                    </h2>
                    <p style="color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;">
                        "Change context to see different suggestions"
                    </p>
                    
                    <div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
                        <button
                            class=move || if context.get() == "invoice" { 
                                "px-4 py-2 bg-blue-600 text-white rounded-lg" 
                            } else { 
                                "px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300" 
                            }
                            on:click=move |_| set_context.set("invoice".to_string())
                        >
                            "Invoice"
                        </button>
                        <button
                            class=move || if context.get() == "payment" { 
                                "px-4 py-2 bg-blue-600 text-white rounded-lg" 
                            } else { 
                                "px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300" 
                            }
                            on:click=move |_| set_context.set("payment".to_string())
                        >
                            "Payment"
                        </button>
                        <button
                            class=move || if context.get() == "expense" { 
                                "px-4 py-2 bg-blue-600 text-white rounded-lg" 
                            } else { 
                                "px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300" 
                            }
                            on:click=move |_| set_context.set("expense".to_string())
                        >
                            "Expense"
                        </button>
                        <button
                            class=move || if context.get() == "report" { 
                                "px-4 py-2 bg-blue-600 text-white rounded-lg" 
                            } else { 
                                "px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300" 
                            }
                            on:click=move |_| set_context.set("report".to_string())
                        >
                            "Report"
                        </button>
                        <button
                            class=move || if context.get() == "tax" { 
                                "px-4 py-2 bg-blue-600 text-white rounded-lg" 
                            } else { 
                                "px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300" 
                            }
                            on:click=move |_| set_context.set("tax".to_string())
                        >
                            "Tax/BIR"
                        </button>
                    </div>
                </div>
                
                // Basic demo with learning enabled
                <div style="background: white; padding: 2rem; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #374151;">
                        "With Learning Enabled"
                    </h2>
                    <p style="color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;">
                        "The AI learns from your selections and improves over time"
                    </p>
                    
                    <div style="position: relative;">
                        <input
                            type="text"
                            placeholder=move || format!("Type something in {} context...", context.get())
                            style="width: 100%; padding: 12px 16px; border: 1px solid #d1d5db; border-radius: 8px; font-size: 14px;"
                            prop:value=move || input.get()
                            on:input=move |ev| set_input.set(event_target_value(&ev))
                        />
                        
                        <SmartSuggestions
                            context=context
                            current_input=input
                            config=ml_config.clone()
                            enable_learning=true
                            on_select=move |suggestion| {
                                set_selected_suggestions.update(|list| {
                                    list.push(suggestion.clone());
                                });
                                logging::log!("Selected: {:?}", suggestion);
                                set_input.set(String::new());
                            }
                        />
                    </div>
                    
                    <div style="margin-top: 1rem;">
                        <strong style="color: #4b5563;">Current Context:</strong>
                        <span style="margin-left: 0.5rem; color: #3b82f6; font-weight: 500;">
                            {move || context.get()}
                        </span>
                    </div>
                </div>
                
                // Demo without learning
                <div style="background: white; padding: 2rem; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #374151;">
                        "Without Learning (Static Suggestions)"
                    </h2>
                    <p style="color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;">
                        "Shows base suggestions without learning from interactions"
                    </p>
                    
                    <div style="position: relative;">
                        {
                            let (static_input, set_static_input) = signal(String::new());
                            view! {
                                <>
                                    <input
                                        type="text"
                                        placeholder="Type to see static suggestions..."
                                        style="width: 100%; padding: 12px 16px; border: 1px solid #d1d5db; border-radius: 8px; font-size: 14px;"
                                        prop:value=move || static_input.get()
                                        on:input=move |ev| set_static_input.set(event_target_value(&ev))
                                    />
                                    
                                    <SmartSuggestions
                                        context=context
                                        current_input=static_input
                                        enable_learning=false
                                        on_select=move |suggestion| {
                                            logging::log!("Selected (no learning): {:?}", suggestion);
                                            set_static_input.set(String::new());
                                        }
                                    />
                                </>
                            }
                        }
                    </div>
                </div>
                
                // Selected suggestions history
                <div style="background: white; padding: 2rem; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #374151;">
                        "Selection History"
                    </h2>
                    <p style="color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;">
                        "Shows what you've selected (affects future suggestions with learning)"
                    </p>
                    
                    <div style="max-height: 200px; overflow-y: auto;">
                        {move || {
                            let suggestions = selected_suggestions.get();
                            if suggestions.is_empty() {
                                view! {
                                    <div style="color: #9ca3af; font-style: italic;">
                                        "No selections yet. Try selecting suggestions above."
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <ul style="list-style: none; padding: 0;">
                                        <For
                                            each=move || selected_suggestions.get()
                                            key=|s| format!("{}_{}", s.id, s.text)
                                            children=|suggestion| {
                                                view! {
                                                    <li style="padding: 8px; margin: 4px 0; background: #f9fafb; border-radius: 6px; display: flex; justify-content: space-between;">
                                                        <span>{suggestion.text}</span>
                                                        <span style="font-size: 12px; color: #6b7280;">
                                                            {format!("{:.0}% confidence", suggestion.confidence * 100.0)}
                                                        </span>
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
                
                // Test scenarios
                <div style="background: white; padding: 2rem; border-radius: 12px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem; color: #374151;">
                        "Test Scenarios"
                    </h2>
                    <p style="color: #6b7280; margin-bottom: 1rem; font-size: 0.875rem;">
                        "Try these inputs with different contexts to see smart suggestions"
                    </p>
                    
                    <ul style="line-height: 1.8; color: #4b5563;">
                        <li><strong>"Invoice" context:</strong> Type "professional", "service", "vat"</li>
                        <li><strong>"Payment" context:</strong> Type "pay", "gcash", "bank"</li>
                        <li><strong>"Expense" context:</strong> Type "bought", "purchase", "supplies"</li>
                        <li><strong>"Tax/BIR" context:</strong> Will show Philippine-specific deadlines</li>
                        <li><strong>Time-based:</strong> Suggestions change based on time of day and month</li>
                    </ul>
                </div>
            </div>
            
            <div style="margin-top: 3rem; padding: 2rem; background: #f0f9ff; border-radius: 12px; border: 1px solid #bfdbfe;">
                <h3 style="font-size: 1.125rem; font-weight: 600; margin-bottom: 1rem; color: #1e40af;">
                    "Features"
                </h3>
                <ul style="color: #3b82f6; line-height: 1.75;">
                    <li>"🧠 <strong>Machine Learning:</strong> Learns from user interactions"</li>
                    <li>"⏰ <strong>Time-based Suggestions:</strong> Context-aware based on time/date"</li>
                    <li>"🇵🇭 <strong>Philippine Context:</strong> BIR deadlines, VAT, withholding tax"</li>
                    <li>"📊 <strong>Pattern Recognition:</strong> Detects and suggests based on patterns"</li>
                    <li>"💾 <strong>Persistent Learning:</strong> Saves ML state to localStorage"</li>
                    <li>"🎯 <strong>Confidence Scores:</strong> Shows prediction confidence"</li>
                    <li>"🏷️ <strong>Categorized:</strong> Command, Entity, Time-based, etc."</li>
                    <li>"⌨️ <strong>Keyboard Navigation:</strong> Arrow keys, Enter, Escape"</li>
                </ul>
            </div>
            
            <div style="margin-top: 2rem; padding: 2rem; background: #fef3c7; border-radius: 12px; border: 1px solid #fcd34d;">
                <h3 style="font-size: 1.125rem; font-weight: 600; margin-bottom: 1rem; color: #92400e;">
                    "ML Algorithm Details"
                </h3>
                <ul style="color: #78350f; line-height: 1.75;">
                    <li>"• <strong>Frequency Analysis:</strong> Tracks selection frequency per context"</li>
                    <li>"• <strong>Context Association:</strong> Links inputs to contexts"</li>
                    <li>"• <strong>Pattern Detection:</strong> Identifies time-of-day and periodic patterns"</li>
                    <li>"• <strong>Confidence Calculation:</strong> Based on frequency and recency"</li>
                    <li>"• <strong>Learning Rate:</strong> Adjustable (default 0.1)"</li>
                    <li>"• <strong>History Limit:</strong> Maintains last 1000 interactions"</li>
                </ul>
            </div>
        </div>
    }
}