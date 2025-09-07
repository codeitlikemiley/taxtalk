use serde::{Deserialize, Serialize};
use crate::types::TokenHint;

/// State management for the TokenAutocomplete component
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TokenAutocompleteState {
    /// Whether hints are currently showing
    pub is_showing: bool,
    
    /// Current search query
    pub query: String,
    
    /// Currently selected hint index
    pub selected_index: usize,
    
    /// Cursor position in input
    pub cursor_position: usize,
    
    /// Recently used tokens for quick access
    pub recent_tokens: Vec<TokenHint>,
    
    /// Maximum recent tokens to store
    pub max_recent: usize,
}

impl TokenAutocompleteState {
    pub fn new() -> Self {
        Self {
            is_showing: false,
            query: String::new(),
            selected_index: 0,
            cursor_position: 0,
            recent_tokens: Vec::new(),
            max_recent: 5,
        }
    }
    
    /// Add a token to recent history
    pub fn add_recent(&mut self, hint: TokenHint) {
        // Remove if already exists
        self.recent_tokens.retain(|h| h.token != hint.token);
        
        // Add to front
        self.recent_tokens.insert(0, hint);
        
        // Trim to max size
        if self.recent_tokens.len() > self.max_recent {
            self.recent_tokens.truncate(self.max_recent);
        }
    }
    
    /// Clear the autocomplete state
    pub fn clear(&mut self) {
        self.is_showing = false;
        self.query = String::new();
        self.selected_index = 0;
    }
    
    /// Navigate to next suggestion
    pub fn next_suggestion(&mut self, max_items: usize) {
        if self.selected_index < max_items.saturating_sub(1) {
            self.selected_index += 1;
        }
    }
    
    /// Navigate to previous suggestion
    pub fn prev_suggestion(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
}

/// Crux integration for state management
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TokenAutocompleteAction {
    ShowHints,
    HideHints,
    UpdateQuery(String),
    SelectToken(TokenHint),
    NavigateUp,
    NavigateDown,
    UpdateCursorPosition(usize),
    ClearRecent,
}

/// Effect types for side effects
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TokenAutocompleteEffect {
    FocusInput,
    UpdateInputValue(String),
    ScrollToSelected,
    TriggerCallback(TokenHint),
}