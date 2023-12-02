use yew::prelude::*;
use yew::{function_component, html, Html, Callback};
use serde::Deserialize;
use gloo_net::http::Request;

#[derive(Deserialize, Debug, Clone)]
pub struct DormJSON {
  pub id: i32,
  pub dorm_name: String,
  pub dorm_group: String,
  pub rooms: i32,
  pub rooms_available: i32,
  pub dorm_pic: String
}

#[derive(Properties, PartialEq)]
pub struct Props {
  pub on_click: Callback<i32>,
}

#[function_component]
pub fn DormList(Props { on_click }: &Props) -> Html {
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

  let urbana_north_dorms : Vec<DormJSON> = dorms_json.to_vec().iter().filter(|&dorm| dorm.dorm_group == "UrbanaNorth").cloned().collect();
  let urbana_south_dorms : Vec<DormJSON> = dorms_json.to_vec().iter().filter(|dorm| dorm.dorm_group == "UrbanaSouth").cloned().collect();
  let ikenberry_commons_north_dorms : Vec<DormJSON> = dorms_json.to_vec().iter().filter(|dorm| dorm.dorm_group == "IkenberryCommonsNorth").cloned().collect();
  let ikenberry_commons_south_dorms : Vec<DormJSON> = dorms_json.to_vec().iter().filter(|dorm| dorm.dorm_group == "IkenberryCommonsSouth").cloned().collect();

  let urbana_north_dorms = urbana_north_dorms.iter().map(|dorm| {
    let on_card_select = {
      let on_click = on_click.clone();
      let dorm = dorm.clone();
      Callback::from(move |_| {
        on_click.emit(dorm.id);
      })
    };

    html_nested! {
    <>
      <div class="col-sm-2">
        <div class="card">
          <img src={dorm.dorm_pic.clone()} class="card-img-top" />
          <div class="card-body">
            <h5 class="card-title">{dorm.dorm_name.clone()}</h5>
            <p>{format!("Available Rooms: {}/{}", dorm.rooms_available, dorm.rooms)}</p>
            <button onclick={on_card_select} class="stretched-link btn btn-primary">{"See Rooms"}</button>
          </div>
        </div>
      </div>
    </>
  }
}).collect::<Vec<_>>();

  let urbana_south_dorms = urbana_south_dorms.iter().map(|dorm| {
    let on_card_select = {
      let on_click = on_click.clone();
      let dorm = dorm.clone();
      Callback::from(move |_| {
        on_click.emit(dorm.id);
      })
    };

    html_nested! {
    <>
      <div class="col-sm-2">
        <div class="card">
          <img src={dorm.dorm_pic.clone()} class="card-img-top" />
          <div class="card-body">
            <h5 class="card-title">{dorm.dorm_name.clone()}</h5>
            <p>{format!("Available Rooms: {}/{}", dorm.rooms_available, dorm.rooms)}</p>
            <button onclick={on_card_select} class="stretched-link btn btn-primary">{"See Rooms"}</button>
          </div>
        </div>
      </div>
    </>
  }
}).collect::<Vec<_>>();

  let ikenberry_commons_north_dorms = ikenberry_commons_north_dorms.iter().map(|dorm| {
    let on_card_select = {
      let on_click = on_click.clone();
      let dorm = dorm.clone();
      Callback::from(move |_| {
        on_click.emit(dorm.id);
      })
    };

    html_nested! {
    <>
      <div class="col-sm-2">
        <div class="card">
        <img src={dorm.dorm_pic.clone()} class="card-img-top" />
        <div class="card-body">
          <h5 class="card-title">{dorm.dorm_name.clone()}</h5>
          <p>{format!("Available Rooms: {}/{}", dorm.rooms_available, dorm.rooms)}</p>
          <button onclick={on_card_select} class="stretched-link btn btn-primary">{"See Rooms"}</button>
          </div>
        </div>
      </div>
    </>
  }
}).collect::<Vec<_>>();

  let ikenberry_commons_south_dorms = ikenberry_commons_south_dorms.iter().map(|dorm| {
    let on_card_select = {
      let on_click = on_click.clone();
      let dorm = dorm.clone();
      Callback::from(move |_| {
        on_click.emit(dorm.id);
      })
    };

    html_nested! {
    <>
      <div class="col-sm-2">
        <div class="card">
        <img src={dorm.dorm_pic.clone()} class="card-img-top" />
        <div class="card-body">
          <h5 class="card-title">{dorm.dorm_name.clone()}</h5>
          <p>{format!("Available Rooms: {}/{}", dorm.rooms_available, dorm.rooms)}</p>
          <button onclick={on_card_select} class="stretched-link btn btn-primary">{"See Rooms"}</button>
          </div>
        </div>
      </div>
    </>
    }
  }).collect::<Vec<_>>();

  html! {
    <>
      <h2>{"Urbana North"}</h2>
      <div class="row">
        {urbana_north_dorms}
      </div>
      <br />
      <h2>{"Urbana South"}</h2>
      <div class="row">
        {urbana_south_dorms}
      </div>
      <br />
      <h2>{"Ikenberry Commons North"}</h2>
      <div class="row">
        {ikenberry_commons_north_dorms}
      </div>
      <br />
      <h2>{"Ikenberry Commons South"}</h2>
      <div class="row">
        {ikenberry_commons_south_dorms}
      </div>
    </>
  }
}