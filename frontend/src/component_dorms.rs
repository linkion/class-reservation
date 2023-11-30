use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;

use yew_bootstrap::component::*;
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
pub fn DormList() -> Html {
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

    let urbana_north_dorms : Vec<DormJSON> = dorms_json.to_vec();
    let urbana_south_dorms : Vec<DormJSON> = dorms_json.to_vec();
    let ikenberry_commons_north_dorms : Vec<DormJSON> = dorms_json.to_vec();
    let ikenberry_commons_south_dorms : Vec<DormJSON> = dorms_json.to_vec();


    let urbana_north_dorms = urbana_north_dorms.iter().map(|dorm| html! {
      <>
        <Card class="w-25">
          <CardTitle>{dorm.dorm_name.clone()}</CardTitle>
        </Card>
      </>
    }).collect::<Html>();

    html! {
      <>
      <h2>{"Urbana North"}</h2>
      <Row class="g-4">
        <Column>
          {urbana_north_dorms}
        </Column>
      </Row>
      </>
    }
}
