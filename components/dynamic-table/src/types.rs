use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use taxtalk_ui_core::ComponentType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColumn {
    pub key: String,
    pub label: String,
    pub component: ComponentType,
    pub width: Option<String>,
    pub computed: Option<String>, // e.g., "quantity * price"
    pub readonly: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    pub id: String,
    pub cells: HashMap<String, Value>,
}

impl TableRow {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            cells: HashMap::new(),
        }
    }
}

impl Default for TableRow {
    fn default() -> Self {
        Self::new()
    }
}