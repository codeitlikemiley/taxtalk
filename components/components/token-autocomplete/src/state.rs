use serde::{Deserialize, Serialize};
use crate::types::*;

/// Component state management
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ComponentState {
    pub value: String,
    pub loading: bool,
    pub error: Option<String>,
    pub data: Option<ComponentData>,
}

/// Crux App integration (placeholder for now)
#[derive(Clone, Debug)]
pub struct ComponentApp;

// Note: Full Crux integration will be implemented when Crux dependency is available
// impl crux::App for ComponentApp {
//     type Model = ComponentState;
//     type Event = ComponentEvent;
//     type ViewModel = ComponentViewModel;
//     type Capabilities = ();
//     
//     fn update(&self, event: Self::Event, model: &mut Self::Model, _caps: &()) {
//         match event {
//             ComponentEvent::ValueChanged(value) => {
//                 model.value = value;
//             }
//             ComponentEvent::Submitted => {
//                 // Handle submission
//             }
//             ComponentEvent::Cancelled => {
//                 // Handle cancellation
//             }
//         }
//     }
//     
//     fn view(&self, model: &Self::Model) -> Self::ViewModel {
//         ComponentViewModel {
//             value: model.value.clone(),
//             loading: model.loading,
//             error: model.error.clone(),
//         }
//     }
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentViewModel {
    pub value: String,
    pub loading: bool,
    pub error: Option<String>,
}
