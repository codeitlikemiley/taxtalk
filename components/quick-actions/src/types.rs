use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single quick action that can be selected
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct QuickAction {
    pub id: String,
    pub label: String,
    pub value: String,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub keyboard_shortcut: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Configuration for quick actions component
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuickActionsConfig {
    /// Maximum number of actions to show initially
    pub initial_count: usize,
    /// Enable lazy loading for additional actions
    pub enable_lazy_loading: bool,
    /// Show keyboard shortcuts
    pub show_shortcuts: bool,
    /// Enable action grouping
    pub enable_grouping: bool,
    /// Animation duration in ms
    pub animation_duration: u32,
    /// Cache frequently used actions
    pub enable_caching: bool,
}

impl Default for QuickActionsConfig {
    fn default() -> Self {
        Self {
            initial_count: 6,
            enable_lazy_loading: true,
            show_shortcuts: true,
            enable_grouping: true,
            animation_duration: 200,
            enable_caching: true,
        }
    }
}

/// Group of related quick actions
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ActionGroup {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub actions: Vec<QuickAction>,
    pub priority: i32,
}

/// Provider interface for generating quick actions
pub trait QuickActionProvider {
    fn get_actions(&self, context: &str, query: &str) -> Vec<QuickAction>;
    fn get_groups(&self, context: &str) -> Vec<ActionGroup>;
    fn can_provide(&self, context: &str) -> bool;
}

/// Usage analytics for quick actions
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ActionUsageStats {
    pub selection_count: HashMap<String, usize>,
    pub last_used: HashMap<String, chrono::DateTime<chrono::Utc>>,
    pub context_associations: HashMap<String, Vec<String>>,
}

/// Events emitted by quick actions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum QuickActionEvent {
    Selected(QuickAction),
    Dismissed,
    LoadMore,
    Search(String),
    GroupToggled(String),
}