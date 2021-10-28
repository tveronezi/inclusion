use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            { "footer" }
        </footer>
    }
}

fn switch(routes: &Route) -> Html {
    let (home, organization, article) = match routes {
        Route::Organization { .. } => (None, Some("active"), None),
        Route::Organizations => (None, Some("active"), None),
        Route::Article { .. } => (None, None, Some("active")),
        Route::Articles => (None, None, Some("active")),
        Route::Home => (Some("active"), None, None),
        Route::NotFound => (None, None, None),
    };
    html! {
        <>
            <li class={ classes!(home) }>
                <a href="/">{ "Home" }</a>
            </li>
            <li class={ classes!(organization) }>
                <a href="/organizations">{ "Organizations" }</a>
            </li>
            <li class={ classes!(article) }>
                <a href="/articles">{ "Articles" }</a>
            </li>
        </>
    }
}

#[function_component(HeaderLinks)]
pub fn header_links() -> Html {
    html! {
        <ul>
            <Router<crate::Route> render={Router::render(switch)} />
        </ul>
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct ElementsProperties<T>
where
    T: PartialEq + std::fmt::Debug,
{
    pub elements: Vec<T>,
}

#[function_component(Elements)]
pub fn elements<T>(props: &ElementsProperties<T>) -> Html
where
    T: PartialEq + std::fmt::Debug,
{
    let elements = props
        .elements
        .iter()
        .map(|e| {
            html! {
                <li>
                    { format!("{:?}", e) }
                </li>
            }
        })
        .collect::<Html>();
    html! {
        <ul>
            { elements }
        </ul>
    }
}
