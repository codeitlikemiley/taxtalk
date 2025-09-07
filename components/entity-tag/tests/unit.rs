#[cfg(test)]
mod tests {
    use taxtalk_entity_tag::*;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn component_renders() {
        // Test that component renders without panicking
        let _result = leptos::ssr::render_to_string(|| {
            view! {
                <EntityTag
                    value=Signal::derive(|| "test".to_string())
                    on_change=Callback::new(|_| {})
                />
            }
        });
    }
    
    #[test]
    fn test_component_state() {
        let mut state = ComponentState::default();
        assert_eq!(state.value, "");
        
        state.value = "new value".to_string();
        assert_eq!(state.value, "new value");
    }
    
    #[test]
    fn test_component_data() {
        let data = ComponentData {
            value: "test".to_string(),
            metadata: None,
        };
        
        assert_eq!(data.value, "test");
        assert!(data.metadata.is_none());
    }
}
