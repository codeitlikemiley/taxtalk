use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentType {
    EntitySelector,
    TextInput,
    NumberInput,
    AmountInput,
    DatePicker,
    DateRange,
    Select(Vec<String>),
    MultiSelect(Vec<String>),
    Boolean,
    FileUpload,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub entity_type: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    pub value: String,
    pub token_type: String,
    pub display: String,
    pub editable: bool,
    pub cardinality: Cardinality,
    pub component: ComponentType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Cardinality {
    Single,
    Multiple,
    Optional,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldMetadata {
    pub label: String,
    pub description: Option<String>,
    pub placeholder: Option<String>,
    pub required: bool,
    pub disabled: bool,
    pub readonly: bool,
}