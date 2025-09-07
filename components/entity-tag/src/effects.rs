use leptos::prelude::*;

/// Side effects and async operations for the component
pub struct ComponentEffects;

impl ComponentEffects {
    /// Load data asynchronously
    pub async fn load_data(query: String) -> Result<Vec<String>, String> {
        // Simulate API call
        Ok(vec![
            format!("Result 1 for {}", query),
            format!("Result 2 for {}", query),
        ])
    }
    
    /// Validate input
    pub fn validate(value: &str) -> Result<(), String> {
        if value.is_empty() {
            Err("Value cannot be empty".to_string())
        } else {
            Ok(())
        }
    }
}
