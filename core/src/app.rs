use chrono::{DateTime, Utc, serde::ts_milliseconds_option::deserialize as ts_milliseconds_option};
use crux_core::{
    Command,
    macros::effect,
    render::{RenderOperation, render},
};
use crux_http::{HttpError, command::Http, protocol::HttpRequest};
use facet::Facet;
use serde::{Deserialize, Serialize};
use url::Url;

const API_URL: &str = "https://your-api-endpoint.com";

#[derive(Facet, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum HttpResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T> From<crux_http::Result<crux_http::Response<T>>>
    for HttpResult<crux_http::Response<T>, HttpError>
{
    fn from(value: crux_http::Result<crux_http::Response<T>>) -> Self {
        match value {
            Ok(response) => HttpResult::Ok(response),
            Err(error) => HttpResult::Err(error),
        }
    }
}

#[derive(Default, Serialize)]
pub struct Model {
    pub count: Count,
}

#[derive(Facet, Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Count {
    pub value: isize,
    #[serde(deserialize_with = "ts_milliseconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Facet, Serialize, Deserialize, Debug, Clone)]
#[facet(namespace = "view_model")]
pub struct ViewModel {
    pub text: String,
    pub confirmed: bool,
}

#[derive(Facet, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[repr(C)]
pub enum Event {
    // Events from the shell (serializable)
    Get,
    Increment,
    Decrement,
    Reset,

    // Events local to the core (not serializable)
    #[serde(skip)]
    #[facet(skip)]
    Set(HttpResult<crux_http::Response<Count>, HttpError>),
    #[serde(skip)]
    #[facet(skip)]
    Update(Count),
    #[serde(skip)]
    #[facet(skip)]
    UpdateBy(isize),
}

#[effect(facet_typegen)]
#[derive(Debug)]
pub enum Effect {
    Render(RenderOperation),
    Http(HttpRequest),
}

#[derive(Default)]
pub struct App;

impl crux_core::App for App {
    type Model = Model;
    type Event = Event;
    type ViewModel = ViewModel;
    type Capabilities = ();
    type Effect = Effect;

    #[allow(clippy::too_many_lines)]
    fn update(
        &self,
        msg: Self::Event,
        model: &mut Self::Model,
        _caps: &Self::Capabilities,
    ) -> Command<Effect, Event> {
        match msg {
            Event::Get => Http::get(API_URL)
                .expect_json()
                .build()
                .map(Into::into)
                .then_send(Event::Set),
            Event::Increment => {
                model.count = Count {
                    value: model.count.value + 1,
                    updated_at: None,
                };
                let call_api = {
                    let base = Url::parse(API_URL).unwrap();
                    let url = base.join("/inc").unwrap();
                    Http::post(url)
                        .expect_json()
                        .build()
                        .map(Into::into)
                        .then_send(Event::Set)
                };
                render().and(call_api)
            }
            Event::Set(HttpResult::Ok(mut response)) => {
                let count = response.take_body().unwrap();
                Command::event(Event::Update(count))
            }
            Event::Set(HttpResult::Err(_)) => {
                // Handle error
                Command::done()
            }
            Event::Update(count) => {
                model.count = count;
                render()
            }
            Event::UpdateBy(change) => {
                model.count.value += change;
                render()
            }
            _ => Command::done(),
        }
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            text: model.count.value.to_string(),
            confirmed: model.count.updated_at.is_some(),
        }
    }
}