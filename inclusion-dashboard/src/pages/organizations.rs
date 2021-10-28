use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct OrganizationProps {
    pub uuid: uuid::Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrganizationsDto {
    pub uuid: String,
    pub name: String,
}

async fn fetch_organizations() -> crate::WebResult<Vec<OrganizationsDto>> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = Request::new_with_str_and_init("/api/v1/organization?start=0&end=100", &opts)?;
    request.headers().set("Accept", "application/json")?;
    let window = web_sys::window().unwrap();
    let resp = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp.dyn_into().expect("Unable to load Response.");
    let json = JsFuture::from(resp.json()?).await?;
    log::info!("resp: {:?}", json);
    let resp: Vec<OrganizationsDto> = json.into_serde().expect("Invalid Json");
    Ok(resp)
}

#[function_component(Organization)]
pub fn organization(props: &OrganizationProps) -> Html {
    use crate::components::HeaderLinks;
    let value = format!("organization {:?}", props.uuid);
    html! {
        <>
            <HeaderLinks />
            <div>{ value }</div>
        </>
    }
}

#[function_component(Organizations)]
pub fn organizations() -> Html {
    use crate::components::HeaderLinks;
    let onclick = Callback::from(|_| {
        log::info!("Click!");
    });
    let organizations = use_state(Vec::new);
    let move_organizations = organizations.clone();
    use_effect_with_deps(
        move |_| {
            let organizations = move_organizations;
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_organizations().await {
                    Ok(loaded) => {
                        organizations.set(loaded);
                    }
                    Err(e) => {
                        log::warn!("{:?}", e);
                    }
                }
            });
            || ()
        },
        (),
    );
    let map_org = |org: &OrganizationsDto| {
        html! {
            <li><span>{ org.name.clone() }</span></li>
        }
    };
    html! {
        <>
            <HeaderLinks />
            <button onclick={ onclick }>{ "Add" }</button>
            <div>{ "organizations" }</div>
            <ul>
                { organizations.iter().map(map_org).collect::<Html>() }
            </ul>
        </>
    }
}
