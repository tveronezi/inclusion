#![doc = include_str ! ("../README.md")]

use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

use derive_more::Display;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Routable, PartialEq, Clone, Debug)]
enum Route {
    #[at("/organizations/:uuid")]
    Organization { uuid: uuid::Uuid },
    #[at("/organizations")]
    Organizations,
    #[at("/articles/:uuid")]
    Article { uuid: uuid::Uuid },
    #[at("/articles")]
    Articles,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    use crate::pages::articles::{Article, Articles};
    use crate::pages::organizations::{Organization, Organizations};
    match routes {
        Route::Organization { uuid } => {
            html! { <Organization uuid={*uuid} /> }
        }
        Route::Organizations => {
            html! { <Organizations /> }
        }
        Route::Article { uuid } => {
            html! { <Article uuid={*uuid} /> }
        }
        Route::Articles => {
            html! { <Articles /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

#[function_component(Home)]
fn home() -> Html {
    use crate::components::HeaderLinks;
    html! {
        <>
            <HeaderLinks />
            <div>{ "home" }</div>
        </>
    }
}

#[function_component(PageNotFound)]
fn not_found() -> Html {
    use crate::components::HeaderLinks;
    html! {
        <>
            <HeaderLinks />
            <div>{ "not found" }</div>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    use crate::components::Footer;
    html! {
        <>
            <main>
                <Router<crate::Route> render={Router::render(switch)} />
            </main>
            <Footer />
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
struct Error(String);

type WebResult<T> = Result<T, Error>;

impl From<wasm_bindgen::JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Self(format!("{:?}", value))
    }
}

pub fn start_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
