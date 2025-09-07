use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityToken {
    pub id: String,
    pub entity_type: String,
    pub display_name: String,
    pub value: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl EntityToken {
    pub fn new(entity_type: impl Into<String>, display_name: impl Into<String>) -> Self {
        let id = format!("entity_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
        Self {
            id,
            entity_type: entity_type.into(),
            display_name: display_name.into(),
            value: None,
            metadata: None,
        }
    }
    
    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParsedSegment {
    Text(String),
    Entity(EntityToken),
}

#[derive(Debug, Clone)]
pub struct MixedContent {
    pub segments: Vec<ParsedSegment>,
    pub raw_text: String,
}

impl MixedContent {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            raw_text: String::new(),
        }
    }
    
    pub fn from_text(text: impl Into<String>) -> Self {
        let text = text.into();
        Self {
            segments: vec![ParsedSegment::Text(text.clone())],
            raw_text: text,
        }
    }
    
    pub fn add_text(&mut self, text: impl Into<String>) {
        let text = text.into();
        self.segments.push(ParsedSegment::Text(text.clone()));
        self.raw_text.push_str(&text);
    }
    
    pub fn add_entity(&mut self, entity: EntityToken) {
        let entity_ref = format!("@{}", entity.display_name);
        self.segments.push(ParsedSegment::Entity(entity.clone()));
        self.raw_text.push_str(&entity_ref);
    }
    
    pub fn get_entities(&self) -> Vec<EntityToken> {
        self.segments
            .iter()
            .filter_map(|seg| match seg {
                ParsedSegment::Entity(entity) => Some(entity.clone()),
                _ => None,
            })
            .collect()
    }
    
    pub fn to_display_string(&self) -> String {
        self.segments
            .iter()
            .map(|seg| match seg {
                ParsedSegment::Text(text) => text.clone(),
                ParsedSegment::Entity(entity) => format!("@{}", entity.display_name),
            })
            .collect::<Vec<_>>()
            .join("")
    }
    
    pub fn to_serialized_string(&self) -> String {
        self.segments
            .iter()
            .map(|seg| match seg {
                ParsedSegment::Text(text) => text.clone(),
                ParsedSegment::Entity(entity) => format!("@entity_{}", entity.id),
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

pub mod uuid {
    use wasm_bindgen::prelude::*;
    
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = crypto)]
        fn randomUUID() -> String;
    }
    
    pub struct Uuid;
    
    impl Uuid {
        pub fn new_v4() -> String {
            randomUUID()
        }
    }
}