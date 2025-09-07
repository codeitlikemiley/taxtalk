use serde::{Deserialize, Serialize};

/// Component-specific types
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ComponentData {
    pub value: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ComponentEvent {
    ValueChanged(String),
    Submitted,
    Cancelled,
}
