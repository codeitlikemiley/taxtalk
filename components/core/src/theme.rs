use leptos::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub primary: &'static str,
    pub secondary: &'static str,
    pub success: &'static str,
    pub warning: &'static str,
    pub error: &'static str,
    pub info: &'static str,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: "blue-600",
            secondary: "gray-600",
            success: "green-600",
            warning: "yellow-600",
            error: "red-600",
            info: "indigo-600",
        }
    }
}

pub fn provide_theme() {
    provide_context(Theme::default());
}

pub fn use_theme() -> Theme {
    use_context::<Theme>().unwrap_or_default()
}