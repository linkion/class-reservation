use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;

use yew_bootstrap::component::*;
use yew_bootstrap::util::*;
use yew_bootstrap::component::card::*;


#[derive(Deserialize, Debug, Clone)]
pub struct DormJSON {
    pub id: i32,
    pub dorm_name: String,
    pub dorm_group: String,
    pub rooms: i32,
    pub rooms_available: i32,
}

#[function_component]
fn DormList() -> Html {
    let dorms_json = use_state(|| vec![]);
    {
        let dorms_json = dorms_json.clone();
        use_effect_with((), move |_| {
            let dorms_json = dorms_json.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_dorms_json: Vec<DormJSON> = Request::get("http://localhost:8081/dorms")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                dorms_json.set(fetched_dorms_json);
            });
            || ()
        });
    }

    let dorms : Vec<DormJSON> = dorms_json.to_vec();

    let dorms = dorms.iter().map(|dorm| html! {
        <p key={dorm.id}>{format!("{} . . . . . . {}", dorm.dorm_name, dorm.dorm_group)}</p>
    }).collect::<Html>();

    html! {
        {dorms}
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            {include_cdn()}
            <h1>{ "UIUC Class Reservation" }</h1>
            <div>
                <h3>{"Classes: "}</h3>
                <DormList />
                <button>{"Hi!"}</button>
            </div>
            <Button style={Color::Primary}>{"Primary"}</Button>
            {include_cdn_js()}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}