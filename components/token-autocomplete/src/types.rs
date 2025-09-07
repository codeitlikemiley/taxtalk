use serde::{Deserialize, Serialize};

/// Token hint for autocomplete suggestions
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TokenHint {
    pub token: String,
    pub description: String,
    pub example: String,
    pub category: String,
}

/// Category types for token hints
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenCategory {
    Entity,
    Value,
    Modifier,
    Text,
}

impl TokenCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenCategory::Entity => "entity",
            TokenCategory::Value => "value",
            TokenCategory::Modifier => "modifier",
            TokenCategory::Text => "text",
        }
    }
    
    pub fn color(&self) -> &'static str {
        match self {
            TokenCategory::Entity => "blue",
            TokenCategory::Value => "green",
            TokenCategory::Modifier => "purple",
            TokenCategory::Text => "gray",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "entity" => Some(TokenCategory::Entity),
            "value" => Some(TokenCategory::Value),
            "modifier" => Some(TokenCategory::Modifier),
            "text" => Some(TokenCategory::Text),
            _ => None,
        }
    }
}

/// Configuration for token autocomplete behavior
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenAutocompleteConfig {
    pub max_suggestions: usize,
    pub debounce_ms: u64,
    pub enable_fuzzy_search: bool,
    pub show_examples: bool,
    pub show_categories: bool,
}

impl Default for TokenAutocompleteConfig {
    fn default() -> Self {
        Self {
            max_suggestions: 10,
            debounce_ms: 300,
            enable_fuzzy_search: true,
            show_examples: true,
            show_categories: true,
        }
    }
}

/// Event types for token autocomplete
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TokenAutocompleteEvent {
    ShowHints,
    HideHints,
    UpdateQuery(String),
    SelectToken(TokenHint),
    NavigateUp,
    NavigateDown,
    CursorPositionChanged(usize),
}