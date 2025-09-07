use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use crate::config::api_url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub commands: Vec<CommandInfo>,
    pub enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandInfo {
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
}

#[component]
pub fn PluginManager() -> impl IntoView {
    let (plugins, set_plugins) = signal::<Vec<PluginInfo>>(vec![]);
    let (is_loading, set_is_loading) = signal(false);
    let (show_upload, set_show_upload) = signal(false);
    
    // Load plugins on mount
    Effect::new(move || {
        spawn_local(async move {
            if let Ok(plugin_list) = fetch_plugins().await {
                set_plugins.set(plugin_list);
            }
        });
    });
    
    let refresh_plugins = move |_| {
        spawn_local(async move {
            set_is_loading.set(true);
            if let Ok(plugin_list) = fetch_plugins().await {
                set_plugins.set(plugin_list);
            }
            set_is_loading.set(false);
        });
    };
    
    // Simplified for now - actual file upload would require more complex handling
    let toggle_upload = move |_| {
        set_show_upload.update(|v| *v = !*v);
    };
    
    let toggle_plugin = move |plugin_id: String, enabled: bool| {
        spawn_local(async move {
            let endpoint = if enabled { "enable" } else { "disable" };
            let _ = reqwest::Client::new()
                .post(&api_url(&format!("api/plugins/{}/{}", plugin_id, endpoint)))
                .send()
                .await;
            
            // Refresh plugin list
            if let Ok(plugin_list) = fetch_plugins().await {
                set_plugins.set(plugin_list);
            }
        });
    };
    
    let uninstall_plugin = move |plugin_id: String| {
        spawn_local(async move {
            let _ = reqwest::Client::new()
                .delete(&api_url(&format!("api/plugins/{}", plugin_id)))
                .send()
                .await;
            
            // Refresh plugin list
            if let Ok(plugin_list) = fetch_plugins().await {
                set_plugins.set(plugin_list);
            }
        });
    };
    
    view! {
        <div class="p-6 bg-white rounded-lg shadow-lg">
            <div class="flex justify-between items-center mb-6">
                <h2 class="text-2xl font-bold text-gray-800">"Plugin Manager"</h2>
                <button
                    class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition-colors"
                    on:click=refresh_plugins
                    disabled=move || is_loading.get()
                >
                    "Refresh"
                </button>
            </div>
            
            // Upload section placeholder
            <div class="mb-6 p-4 bg-gray-50 rounded-lg">
                <div class="flex justify-between items-center">
                    <h3 class="text-lg font-semibold">"Install New Plugin"</h3>
                    <button
                        class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
                        on:click=toggle_upload
                    >
                        {move || if show_upload.get() { "Cancel" } else { "Upload WASM" }}
                    </button>
                </div>
                {move || if show_upload.get() {
                    view! {
                        <div class="mt-4 p-4 bg-white rounded border border-gray-200">
                            <p class="text-sm text-gray-600">
                                "To install a plugin, upload a .wasm file compiled for the TaxTalk plugin system."
                            </p>
                            <p class="text-xs text-gray-500 mt-2">
                                "Note: File upload UI is simplified for demo purposes."
                            </p>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>
            
            // Plugin list
            <div class="space-y-4">
                {move || if plugins.get().is_empty() {
                    view! {
                        <div class="text-center py-8 text-gray-500">
                            "No plugins installed. Upload a .wasm file to get started!"
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <For
                            each=move || plugins.get()
                            key=|plugin| plugin.id.clone()
                            children=move |plugin| {
                                let plugin_id = plugin.id.clone();
                                let plugin_id_toggle = plugin_id.clone();
                                let plugin_id_uninstall = plugin_id.clone();
                                
                                view! {
                                    <div class="border border-gray-200 rounded-lg p-4">
                                        <div class="flex justify-between items-start mb-2">
                                            <div>
                                                <h3 class="text-lg font-semibold">{plugin.name.clone()}</h3>
                                                <p class="text-sm text-gray-600">{format!("v{}", plugin.version)}</p>
                                            </div>
                                            <div class="flex space-x-2">
                                                <button
                                                    class={if plugin.enabled {
                                                        "px-3 py-1 bg-yellow-500 text-white rounded text-sm hover:bg-yellow-600"
                                                    } else {
                                                        "px-3 py-1 bg-green-500 text-white rounded text-sm hover:bg-green-600"
                                                    }}
                                                    on:click=move |_| toggle_plugin(plugin_id_toggle.clone(), !plugin.enabled)
                                                >
                                                    {if plugin.enabled { "Disable" } else { "Enable" }}
                                                </button>
                                                <button
                                                    class="px-3 py-1 bg-red-500 text-white rounded text-sm hover:bg-red-600"
                                                    on:click=move |_| uninstall_plugin(plugin_id_uninstall.clone())
                                                >
                                                    "Uninstall"
                                                </button>
                                            </div>
                                        </div>
                                        
                                        <p class="text-gray-700 mb-3">{plugin.description.clone()}</p>
                                        
                                        <div class="bg-gray-50 rounded p-2">
                                            <p class="text-sm font-semibold text-gray-600 mb-1">"Commands:"</p>
                                            <div class="flex flex-wrap gap-2">
                                                <For
                                                    each=move || plugin.commands.clone()
                                                    key=|cmd| cmd.name.clone()
                                                    children=move |cmd| {
                                                        view! {
                                                            <span class="px-2 py-1 bg-blue-100 text-blue-700 rounded text-xs">
                                                                {cmd.name.clone()}
                                                            </span>
                                                        }
                                                    }
                                                />
                                            </div>
                                        </div>
                                        
                                        <div class="mt-2">
                                            <span class={if plugin.enabled {
                                                "px-2 py-1 bg-green-100 text-green-700 rounded text-xs"
                                            } else {
                                                "px-2 py-1 bg-gray-100 text-gray-700 rounded text-xs"
                                            }}>
                                                {if plugin.enabled { "Enabled" } else { "Disabled" }}
                                            </span>
                                        </div>
                                    </div>
                                }
                            }
                        />
                    }.into_any()
                }}
            </div>
        </div>
    }
}

async fn fetch_plugins() -> Result<Vec<PluginInfo>, String> {
    let response = reqwest::get(api_url("api/plugins"))
        .await
        .map_err(|e| format!("Failed to fetch plugins: {}", e))?;
    
    response
        .json::<Vec<PluginInfo>>()
        .await
        .map_err(|e| format!("Failed to parse plugins: {}", e))
}