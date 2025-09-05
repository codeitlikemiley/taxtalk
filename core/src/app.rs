use crux_core::{
    Command,
    macros::effect,
    render::{RenderOperation, render},
};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize)]
pub struct Model {
    // Add your application state here
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ViewModel {
    // Add your view model fields here
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum Event {
    // Add your application events here
    Initialize,
}

#[effect(facet_typegen)]
#[derive(Debug)]
pub enum Effect {
    Render(RenderOperation),
}

#[derive(Default)]
pub struct App;

impl crux_core::App for App {
    type Model = Model;
    type Event = Event;
    type ViewModel = ViewModel;
    type Capabilities = ();
    type Effect = Effect;

    fn update(
        &self,
        msg: Self::Event,
        _model: &mut Self::Model,
        _caps: &Self::Capabilities,
    ) -> Command<Effect, Event> {
        match msg {
            Event::Initialize => {
                render()
            }
        }
    }

    fn view(&self, _model: &Self::Model) -> Self::ViewModel {
        Self::ViewModel::default()
    }
}