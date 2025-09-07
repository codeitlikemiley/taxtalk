/// Configuration for the web application
/// 
/// Uses environment variables at build time to configure the API server URL
/// SERVER_PORT defaults to 3000 if not specified
/// 
/// To use: SERVER_PORT=4000 trunk serve --port 4001

use web_sys::window;

// Get the server port from environment variable at compile time, default to 3000
pub fn get_server_port() -> &'static str {
    option_env!("SERVER_PORT").unwrap_or("3000")
}

/// Get the base URL for API calls
/// First checks localStorage for runtime configuration, then falls back to compile-time env
pub fn api_base_url() -> String {
    // Try to get from localStorage first (runtime configuration)
    if let Some(window) = window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Some(api_url) = storage.get_item("API_URL").ok().flatten() {
                return api_url;
            }
        }
    }
    
    // Fall back to compile-time environment variable
    format!("http://localhost:{}", get_server_port())
}

/// Get the full API URL for a given endpoint
pub fn api_url(endpoint: &str) -> String {
    format!("{}/{}", api_base_url(), endpoint.trim_start_matches('/'))
}

/// Set the API URL in localStorage for runtime configuration
pub fn set_api_url(url: &str) {
    if let Some(window) = window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            let _ = storage.set_item("API_URL", url);
        }
    }
}