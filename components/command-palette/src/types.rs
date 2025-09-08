use serde::{Deserialize, Serialize};
use std::fmt;

/// A command that can be executed from the palette
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Command {
    pub id: String,
    pub label: String,
    pub description: String,
    pub icon: Option<String>,
    pub shortcut: Option<String>,
    pub category: CommandCategory,
    pub keywords: Vec<String>,
    pub action: CommandAction,
}

/// Category for grouping commands
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CommandCategory {
    Sales,
    Finance,
    Tax,
    Contacts,
    Reports,
    Settings,
    Navigation,
    Custom(String),
}

impl fmt::Display for CommandCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandCategory::Sales => write!(f, "Sales"),
            CommandCategory::Finance => write!(f, "Finance"),
            CommandCategory::Tax => write!(f, "Tax"),
            CommandCategory::Contacts => write!(f, "Contacts"),
            CommandCategory::Reports => write!(f, "Reports"),
            CommandCategory::Settings => write!(f, "Settings"),
            CommandCategory::Navigation => write!(f, "Navigation"),
            CommandCategory::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Action to execute when command is selected
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CommandAction {
    /// Navigate to a route
    Navigate(String),
    /// Execute a plugin action
    Plugin { plugin: String, action: String },
    /// Run a custom function
    Custom(String),
    /// Open external URL
    OpenUrl(String),
    /// Show a modal/dialog
    ShowModal(String),
    /// Toggle a setting
    ToggleSetting(String),
}

/// Configuration for the command palette
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandPaletteConfig {
    /// Enable fuzzy search
    pub enable_fuzzy_search: bool,
    /// Maximum results to show
    pub max_results: usize,
    /// Enable recent commands section
    pub show_recent: bool,
    /// Number of recent commands to show
    pub recent_count: usize,
    /// Enable categories
    pub show_categories: bool,
    /// Enable NLP mode
    pub enable_nlp: bool,
    /// Debounce delay for search in ms
    pub search_debounce: u32,
    /// Show keyboard shortcuts
    pub show_shortcuts: bool,
}

impl Default for CommandPaletteConfig {
    fn default() -> Self {
        Self {
            enable_fuzzy_search: true,
            max_results: 10,
            show_recent: true,
            recent_count: 3,
            show_categories: true,
            enable_nlp: true,
            search_debounce: 150,
            show_shortcuts: true,
        }
    }
}

/// Result from NLP parsing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NLPResult {
    pub command: Option<Command>,
    pub confidence: f32,
    pub parameters: Vec<NLPParameter>,
    pub raw_text: String,
}

/// Parameter extracted from NLP
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NLPParameter {
    pub name: String,
    pub value: String,
    pub param_type: String,
    pub editable: bool,
}

/// Search result with score
#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult {
    pub command: Command,
    pub score: f64,
    pub matched_fields: Vec<String>,
}

/// Event emitted by command palette
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CommandPaletteEvent {
    CommandExecuted(Command),
    Opened,
    Closed,
    SearchChanged(String),
    CategorySelected(CommandCategory),
}