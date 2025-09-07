use serde::{Deserialize, Serialize};

/// Entity data structure
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub entity_type: String,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Configuration for the EntityCombobox component
#[derive(Clone, Debug)]
pub struct EntityComboboxConfig {
    /// Maximum number of suggestions to show
    pub max_suggestions: usize,
    /// Debounce delay in milliseconds for search
    pub debounce_ms: u32,
    /// Enable keyboard navigation
    pub enable_keyboard_nav: bool,
    /// Show loading indicator
    pub show_loading: bool,
    /// Cache search results
    pub cache_results: bool,
    /// Cache expiry in seconds
    pub cache_expiry_secs: u64,
}

impl Default for EntityComboboxConfig {
    fn default() -> Self {
        Self {
            max_suggestions: 10,
            debounce_ms: 300,
            enable_keyboard_nav: true,
            show_loading: true,
            cache_results: true,
            cache_expiry_secs: 300, // 5 minutes
        }
    }
}

/// Search response from API
#[derive(Clone, Debug, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<Entity>,
    pub total: usize,
}

/// Entity type enum for easier handling
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntityType {
    Client,
    Supplier,
    Product,
    Employee,
    Invoice,
    Payment,
    Custom(String),
}

impl EntityType {
    pub fn as_str(&self) -> &str {
        match self {
            EntityType::Client => "client",
            EntityType::Supplier => "supplier",
            EntityType::Product => "product",
            EntityType::Employee => "employee",
            EntityType::Invoice => "invoice",
            EntityType::Payment => "payment",
            EntityType::Custom(s) => s,
        }
    }
    
    pub fn display_name(&self) -> &str {
        match self {
            EntityType::Client => "Client",
            EntityType::Supplier => "Supplier",
            EntityType::Product => "Product",
            EntityType::Employee => "Employee",
            EntityType::Invoice => "Invoice",
            EntityType::Payment => "Payment",
            EntityType::Custom(s) => s,
        }
    }
    
    pub fn color(&self) -> &str {
        match self {
            EntityType::Client => "blue",
            EntityType::Supplier => "green",
            EntityType::Product => "purple",
            EntityType::Employee => "orange",
            EntityType::Invoice => "indigo",
            EntityType::Payment => "pink",
            EntityType::Custom(_) => "gray",
        }
    }
}

impl From<String> for EntityType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "client" => EntityType::Client,
            "supplier" => EntityType::Supplier,
            "product" => EntityType::Product,
            "employee" => EntityType::Employee,
            "invoice" => EntityType::Invoice,
            "payment" => EntityType::Payment,
            _ => EntityType::Custom(s),
        }
    }
}