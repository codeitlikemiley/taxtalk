use leptos::prelude::*;
use command_palette::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}

#[component]
fn App() -> impl IntoView {
    // State for showing the palette
    let (show_palette, set_show_palette) = signal(false);
    let (last_command, set_last_command) = signal::<Option<String>>(None);
    
    // Custom commands for demo
    let demo_commands = vec![
        // Add some demo-specific commands
        Command {
            id: "demo_hello".to_string(),
            label: "Say Hello".to_string(),
            description: "Shows a greeting message".to_string(),
            icon: Some("👋".to_string()),
            shortcut: Some("Ctrl+H".to_string()),
            category: CommandCategory::Custom("Demo".to_string()),
            keywords: vec!["hello", "greeting", "welcome"].into_iter().map(String::from).collect(),
            action: CommandAction::Custom("hello".to_string()),
        },
        Command {
            id: "demo_toggle_theme".to_string(),
            label: "Toggle Theme".to_string(),
            description: "Switch between light and dark theme".to_string(),
            icon: Some("🎨".to_string()),
            shortcut: Some("Ctrl+T".to_string()),
            category: CommandCategory::Settings,
            keywords: vec!["theme", "dark", "light", "color"].into_iter().map(String::from).collect(),
            action: CommandAction::ToggleSetting("theme".to_string()),
        },
    ];
    
    // Combine with default commands
    let mut all_commands = default_commands();
    all_commands.extend(philippine_commands());
    all_commands.extend(demo_commands);
    
    // Different configurations for testing
    let default_config = CommandPaletteConfig::default();
    
    let custom_config = CommandPaletteConfig {
        enable_fuzzy_search: true,
        max_results: 5,
        show_recent: true,
        recent_count: 2,
        show_categories: true,
        enable_nlp: true,
        search_debounce: 100,
        show_shortcuts: true,
    };
    
    // Handle keyboard shortcut to open palette
    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        // Ctrl+K or Cmd+K to open
        if (ev.ctrl_key() || ev.meta_key()) && ev.key() == "k" {
            ev.prevent_default();
            set_show_palette.set(true);
        }
    };
    
    // Set up global keyboard listener
    use leptos::ev::keydown;
    window_event_listener(keydown, handle_keydown);
    
    view! {
        <div style="min-height: 100vh; padding: 2rem;">
            <style>{include_str!("styles.css")}</style>
            
            <div style="max-width: 1200px; margin: 0 auto;">
                <div style="text-align: center; color: white; margin-bottom: 3rem;">
                    <h1 style="font-size: 3rem; font-weight: bold; margin-bottom: 1rem;">
                        "Command Palette Demo"
                    </h1>
                    <p style="font-size: 1.25rem; opacity: 0.9;">
                        "A powerful command interface with fuzzy search and NLP support"
                    </p>
                </div>
                
                // Demo Controls
                <div style="display: grid; gap: 2rem; margin-bottom: 3rem;">
                    <div style="background: white; padding: 2rem; border-radius: 1rem; box-shadow: 0 10px 40px rgba(0,0,0,0.1);">
                        <h2 style="font-size: 1.5rem; font-weight: 600; margin-bottom: 1.5rem; color: #1f2937;">
                            "Try the Command Palette"
                        </h2>
                        
                        <div style="display: flex; gap: 1rem; flex-wrap: wrap; margin-bottom: 2rem;">
                            <button
                                style="padding: 0.75rem 1.5rem; background: #3b82f6; color: white; border: none; border-radius: 0.5rem; font-size: 1rem; font-weight: 500; cursor: pointer; transition: all 0.2s;"
                                on:click=move |_| set_show_palette.set(true)
                            >
                                "Open Command Palette"
                            </button>
                            
                            <div style="display: flex; align-items: center; gap: 0.5rem; padding: 0.75rem 1.5rem; background: #f3f4f6; border-radius: 0.5rem;">
                                <span style="color: #6b7280;">Or press</span>
                                <kbd style="padding: 0.25rem 0.5rem; background: white; border: 1px solid #d1d5db; border-radius: 0.25rem; font-family: monospace; font-size: 0.875rem;">
                                    "Ctrl+K"
                                </kbd>
                            </div>
                        </div>
                        
                        <Show when=move || last_command.get().is_some()>
                            <div style="padding: 1rem; background: #eff6ff; border-left: 4px solid #3b82f6; border-radius: 0.5rem;">
                                <p style="font-size: 0.875rem; color: #1e40af; margin: 0;">
                                    <strong>"Last command executed:"</strong> {move || last_command.get().unwrap_or_default()}
                                </p>
                            </div>
                        </Show>
                    </div>
                    
                    // Instructions
                    <div style="background: white; padding: 2rem; border-radius: 1rem; box-shadow: 0 10px 40px rgba(0,0,0,0.1);">
                        <h2 style="font-size: 1.5rem; font-weight: 600; margin-bottom: 1.5rem; color: #1f2937;">
                            "How to Use"
                        </h2>
                        
                        <div style="display: grid; gap: 1rem;">
                            <div style="display: flex; gap: 1rem;">
                                <span style="font-size: 1.5rem;">🔍</span>
                                <div>
                                    <h3 style="font-weight: 600; color: #374151; margin-bottom: 0.25rem;">
                                        "Search Commands"
                                    </h3>
                                    <p style="color: #6b7280; font-size: 0.875rem;">
                                        "Type to search through all available commands. Fuzzy search finds matches even with typos."
                                    </p>
                                </div>
                            </div>
                            
                            <div style="display: flex; gap: 1rem;">
                                <span style="font-size: 1.5rem;">🤖</span>
                                <div>
                                    <h3 style="font-weight: 600; color: #374151; margin-bottom: 0.25rem;">
                                        "Natural Language Mode"
                                    </h3>
                                    <p style="color: #6b7280; font-size: 0.875rem;">
                                        "Type longer sentences like \"create invoice for 5000\" to trigger NLP mode."
                                    </p>
                                </div>
                            </div>
                            
                            <div style="display: flex; gap: 1rem;">
                                <span style="font-size: 1.5rem;">⌨️</span>
                                <div>
                                    <h3 style="font-weight: 600; color: #374151; margin-bottom: 0.25rem;">
                                        "Keyboard Navigation"
                                    </h3>
                                    <p style="color: #6b7280; font-size: 0.875rem;">
                                        "Use arrow keys to navigate, Enter to select, Escape to close."
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    // Example Queries
                    <div style="background: white; padding: 2rem; border-radius: 1rem; box-shadow: 0 10px 40px rgba(0,0,0,0.1);">
                        <h2 style="font-size: 1.5rem; font-weight: 600; margin-bottom: 1.5rem; color: #1f2937;">
                            "Try These Searches"
                        </h2>
                        
                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;">
                            {
                                let examples = vec![
                                    ("invoice", "Find invoice commands"),
                                    ("vat", "VAT and tax commands"),
                                    ("payment", "Payment recording"),
                                    ("gcash", "GCash payments"),
                                    ("report", "Financial reports"),
                                    ("/", "Filter command mode"),
                                ];
                                
                                examples.into_iter().map(|(query, desc)| {
                                    view! {
                                        <button
                                            style="padding: 0.75rem; background: #f9fafb; border: 1px solid #e5e7eb; border-radius: 0.5rem; text-align: left; cursor: pointer; transition: all 0.2s;"
                                            on:click=move |_| {
                                                set_show_palette.set(true);
                                                // Would set the search query if we had access to it
                                            }
                                        >
                                            <code style="font-size: 0.875rem; color: #3b82f6; font-weight: 500;">
                                                {query}
                                            </code>
                                            <p style="font-size: 0.75rem; color: #6b7280; margin: 0.25rem 0 0 0;">
                                                {desc}
                                            </p>
                                        </button>
                                    }
                                }).collect_view()
                            }
                        </div>
                    </div>
                    
                    // Features
                    <div style="background: white; padding: 2rem; border-radius: 1rem; box-shadow: 0 10px 40px rgba(0,0,0,0.1);">
                        <h2 style="font-size: 1.5rem; font-weight: 600; margin-bottom: 1.5rem; color: #1f2937;">
                            "Features"
                        </h2>
                        
                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem;">
                            <div style="padding: 1rem; background: #f0f9ff; border-radius: 0.5rem;">
                                <h3 style="color: #1e40af; font-size: 0.875rem; font-weight: 600; margin-bottom: 0.5rem;">
                                    "🔍 Fuzzy Search"
                                </h3>
                                <p style="color: #3b82f6; font-size: 0.75rem;">
                                    "Find commands even with typos"
                                </p>
                            </div>
                            
                            <div style="padding: 1rem; background: #f0fdf4; border-radius: 0.5rem;">
                                <h3 style="color: #14532d; font-size: 0.875rem; font-weight: 600; margin-bottom: 0.5rem;">
                                    "🤖 NLP Support"
                                </h3>
                                <p style="color: #16a34a; font-size: 0.75rem;">
                                    "Natural language understanding"
                                </p>
                            </div>
                            
                            <div style="padding: 1rem; background: #fef3c7; border-radius: 0.5rem;">
                                <h3 style="color: #78350f; font-size: 0.875rem; font-weight: 600; margin-bottom: 0.5rem;">
                                    "⏱️ Recent Commands"
                                </h3>
                                <p style="color: #f59e0b; font-size: 0.75rem;">
                                    "Quick access to frequently used"
                                </p>
                            </div>
                            
                            <div style="padding: 1rem; background: #fce7f3; border-radius: 0.5rem;">
                                <h3 style="color: #831843; font-size: 0.875rem; font-weight: 600; margin-bottom: 0.5rem;">
                                    "🇵🇭 Philippine Context"
                                </h3>
                                <p style="color: #ec4899; font-size: 0.75rem;">
                                    "BIR, VAT, GCash, Maya support"
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            
            // Command Palette Component
            <CommandPalette
                show=show_palette
                commands=all_commands
                config=custom_config
                on_execute=move |action| {
                    set_show_palette.set(false);
                    
                    // Handle the action and update last command
                    let action_str = match action {
                        CommandAction::Navigate(route) => format!("Navigate to {}", route),
                        CommandAction::Plugin { plugin, action } => format!("{}:{}", plugin, action),
                        CommandAction::Custom(data) => format!("Custom: {}", data),
                        CommandAction::OpenUrl(url) => format!("Open URL: {}", url),
                        CommandAction::ShowModal(modal) => format!("Show modal: {}", modal),
                        CommandAction::ToggleSetting(setting) => format!("Toggle: {}", setting),
                    };
                    
                    set_last_command.set(Some(action_str));
                    
                    // Log to console
                    leptos::logging::log!("Command executed: {:?}", action);
                }
            />
        </div>
    }
}