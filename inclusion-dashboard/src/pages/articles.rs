use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ArticleProps {
    pub uuid: uuid::Uuid,
}

#[function_component(Article)]
pub fn article(props: &ArticleProps) -> Html {
    use crate::components::HeaderLinks;
    let value = format!("article {:?}", props.uuid);
    html! {
        <>
            <HeaderLinks />
            <div>{ value }</div>
        </>
    }
}

#[function_component(Articles)]
pub fn articles() -> Html {
    use crate::components::HeaderLinks;
    html! {
        <>
            <HeaderLinks />
            <div>{ "articles" }</div>
        </>
    }
}
