use serde::{Deserialize, Serialize};
use crate::types::*;

/// Component state management
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EntityTagState {
    pub token: Option<EntityToken>,
    pub is_removing: bool,
}

/// Events that can occur on the entity tag
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EntityTagEvent {
    SetToken(EntityToken),
    Remove,
    RemovalConfirmed,
}

/// Crux App integration placeholder
#[derive(Clone, Debug)]
pub struct EntityTagApp;

// Note: Full Crux integration will be implemented when Crux dependency is available
// For now, we use a simplified state management approach

impl EntityTagApp {
    pub fn update(&self, event: EntityTagEvent, state: &mut EntityTagState) {
        match event {
            EntityTagEvent::SetToken(token) => {
                state.token = Some(token);
                state.is_removing = false;
            }
            EntityTagEvent::Remove => {
                state.is_removing = true;
            }
            EntityTagEvent::RemovalConfirmed => {
                state.token = None;
                state.is_removing = false;
            }
        }
    }
}

/// View model for the component
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityTagViewModel {
    pub token: Option<EntityToken>,
    pub is_removing: bool,
}

impl From<&EntityTagState> for EntityTagViewModel {
    fn from(state: &EntityTagState) -> Self {
        EntityTagViewModel {
            token: state.token.clone(),
            is_removing: state.is_removing,
        }
    }
}
