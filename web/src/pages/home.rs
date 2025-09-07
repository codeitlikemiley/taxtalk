use leptos::prelude::*;
use crate::components::smart_command_input::SmartCommandInput;
use crate::components::plugins::PluginManager;

#[component]
pub fn Home() -> impl IntoView {
    let (show_plugins, set_show_plugins) = signal(false);
    
    view! {
        <div class="flex h-screen bg-gradient-to-br from-slate-50 to-slate-100">
            // Sidebar
            <div class="w-80 bg-white border-r border-gray-200 shadow-lg flex flex-col">
                // Header
                <div class="p-6 border-b border-gray-200">
                    <div class="flex items-center space-x-3">
                        <div class="w-10 h-10 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center">
                            <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"></path>
                            </svg>
                        </div>
                        <div>
                            <h1 class="text-xl font-bold text-gray-800">"TaxTalk"</h1>
                            <p class="text-xs text-gray-500">"Natural Language Bookkeeping"</p>
                        </div>
                    </div>
                </div>
                
                // Navigation
                <nav class="flex-1 p-4">
                    <button
                        class="w-full text-left px-4 py-3 rounded-lg hover:bg-gray-100 transition-colors flex items-center space-x-3 mb-2"
                        on:click=move |_| set_show_plugins.set(false)
                    >
                        <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
                        </svg>
                        <span class="font-medium text-gray-700">"Chat"</span>
                    </button>
                    
                    <button
                        class="w-full text-left px-4 py-3 rounded-lg hover:bg-gray-100 transition-colors flex items-center space-x-3"
                        on:click=move |_| set_show_plugins.set(true)
                    >
                        <svg class="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"></path>
                        </svg>
                        <span class="font-medium text-gray-700">"Plugins"</span>
                    </button>
                </nav>
                
                // Quick Commands
                <div class="p-4 border-t border-gray-200">
                    <h3 class="text-xs font-semibold text-gray-500 uppercase mb-3">"Quick Commands"</h3>
                    <div class="space-y-2">
                        <QuickCommand 
                            icon="📄" 
                            title="Create Invoice" 
                            command="create invoice for @client" 
                        />
                        <QuickCommand 
                            icon="💰" 
                            title="Record Payment" 
                            command="@client paid 5000" 
                        />
                        <QuickCommand 
                            icon="📊" 
                            title="VAT Report" 
                            command="generate VAT return for this month" 
                        />
                    </div>
                </div>
                
                // User section
                <div class="p-4 border-t border-gray-200">
                    <div class="flex items-center space-x-3">
                        <div class="w-10 h-10 bg-gradient-to-br from-green-400 to-blue-500 rounded-full flex items-center justify-center">
                            <span class="text-white font-semibold">"U"</span>
                        </div>
                        <div class="flex-1">
                            <p class="text-sm font-medium text-gray-700">"User"</p>
                            <p class="text-xs text-gray-500">"Online"</p>
                        </div>
                    </div>
                </div>
            </div>
            
            // Main Content
            <div class="flex-1 flex flex-col">
                {move || if show_plugins.get() {
                    view! {
                        <div class="flex-1 overflow-y-auto p-6">
                            <PluginManager />
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="flex-1 flex flex-col p-6">
                            <div class="mb-4">
                                <h2 class="text-2xl font-bold text-gray-800 mb-2">"Smart Command Input"</h2>
                                <p class="text-gray-600">"Type commands like 'create invoice' to enter tokenization mode, or ask questions in AI mode"</p>
                            </div>
                            
                            <div class="flex-1 flex items-center justify-center">
                                <div class="w-full max-w-4xl">
                                    <SmartCommandInput />
                                </div>
                            </div>
                        </div>
                    }.into_any()
                }}
            </div>
        </div>
    }
}

#[component]
fn QuickCommand(
    icon: &'static str,
    title: &'static str,
    command: &'static str,
) -> impl IntoView {
    view! {
        <button class="w-full text-left px-3 py-2 rounded-lg hover:bg-gray-100 transition-colors flex items-center space-x-2">
            <span class="text-lg">{icon}</span>
            <div class="flex-1">
                <p class="text-sm font-medium text-gray-700">{title}</p>
                <p class="text-xs text-gray-500 truncate">{command}</p>
            </div>
        </button>
    }
}