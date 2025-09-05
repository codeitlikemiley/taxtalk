use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::token::traits::{TokenPosition, TokenMetadata};
use std::time::{SystemTime, UNIX_EPOCH};

/// Helper macro to implement common Token trait methods
#[macro_export]
macro_rules! impl_token_base {
    ($type:ty) => {
        fn uuid(&self) -> Uuid {
            self.base.uuid
        }
        
        fn raw_text(&self) -> &str {
            &self.base.raw_text
        }
        
        fn position(&self) -> &TokenPosition {
            &self.base.position
        }
        
        fn clone_box(&self) -> Box<dyn Token> {
            panic!("Clone not implemented for this token type")
        }
        
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
        
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        
        fn type_id(&self) -> std::any::TypeId {
            std::any::TypeId::of::<$type>()
        }
        
        fn metadata(&self) -> TokenMetadata {
            self.base.metadata.clone()
        }
    };
}

/// Base fields that most tokens will have
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseTokenFields {
    pub uuid: Uuid,
    pub raw_text: String,
    pub position: TokenPosition,
    pub metadata: TokenMetadata,
}

impl BaseTokenFields {
    pub fn new(raw_text: String, position: TokenPosition) -> Self {
        Self::with_plugin(raw_text, position, None)
    }

    pub fn with_plugin(raw_text: String, position: TokenPosition, plugin: Option<String>) -> Self {
        let uuid = Uuid::new_v4(); // Or use v5 for deterministic
        Self {
            uuid,
            raw_text,
            position,
            metadata: TokenMetadata {
                source_plugin: plugin,
                confidence: 1.0,
                alternatives: vec![],
                tags: vec![],
                created_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
        }
    }

    pub fn with_metadata(
        raw_text: String, 
        position: TokenPosition,
        confidence: f32,
        tags: Vec<String>
    ) -> Self {
        let uuid = Uuid::new_v4();
        Self {
            uuid,
            raw_text,
            position,
            metadata: TokenMetadata {
                source_plugin: None,
                confidence,
                alternatives: vec![],
                tags,
                created_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
        }
    }
}