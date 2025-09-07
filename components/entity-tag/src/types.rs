use serde::{Deserialize, Serialize};

/// Entity token representation
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EntityToken {
    pub id: String,
    pub entity_type: String,
    pub display_name: String,
    pub metadata: Option<serde_json::Value>,
}

/// Entity types in the system
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EntityType {
    Client,
    Supplier,
    Product,
    Employee,
    Invoice,
    Payment,
}

impl EntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityType::Client => "client",
            EntityType::Supplier => "supplier",
            EntityType::Product => "product",
            EntityType::Employee => "employee",
            EntityType::Invoice => "invoice",
            EntityType::Payment => "payment",
        }
    }
    
    pub fn color_class(&self) -> &'static str {
        match self {
            EntityType::Client => "tui-et-client",
            EntityType::Supplier => "tui-et-supplier",
            EntityType::Product => "tui-et-product",
            EntityType::Employee => "tui-et-employee",
            EntityType::Invoice => "tui-et-invoice",
            EntityType::Payment => "tui-et-payment",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "client" => Some(EntityType::Client),
            "supplier" => Some(EntityType::Supplier),
            "product" => Some(EntityType::Product),
            "employee" => Some(EntityType::Employee),
            "invoice" => Some(EntityType::Invoice),
            "payment" => Some(EntityType::Payment),
            _ => None,
        }
    }
}
