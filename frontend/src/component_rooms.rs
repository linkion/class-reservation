use gloo::console::log;
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

/*
use yew_bootstrap::component::*;
use yew_bootstrap::component::card::*;
*/
#[derive(Deserialize, Debug, Clone)]
pub struct RoomJSON {
    id: i32,
    room_number: i32,
    max_occupants: i32,
    occupants: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DormJSON {
    pub id: i32,
    pub dorm_name: String,
    pub dorm_group: String,
    pub rooms: i32,
    pub rooms_available: i32,
}

#[derive(Properties, PartialEq)]
pub struct RoomProps {
    pub dorm_id: i32,
    pub on_click: Callback<bool>,
    pub room_on_click: Callback<i32>,
  }

#[derive(Deserialize, Serialize, Clone)]
pub struct RoomStateInput {
  pub room_id: i32,
  pub student_id: i32,
}

#[function_component]
pub fn RoomList(props: &RoomProps) -> Html {
    let dorm_id = props.dorm_id;
    let dorm_json = use_state(|| DormJSON { id: 0, dorm_name: String::from(""), dorm_group: String::from(""), rooms: 0, rooms_available: 0 });
    {
      let dorm_json = dorm_json.clone();
      use_effect_with((), move |_| {
        let dorm_json = dorm_json.clone();
        wasm_bindgen_futures::spawn_local(async move {
          let fetched_dorms_json: DormJSON = Request::get(&format!("http://localhost:8081/dorms/{}", dorm_id).to_string())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
          dorm_json.set(fetched_dorms_json);
        });
        || ()
      });
    }
    

    let rooms_json = use_state(|| vec![]);
    {
        let rooms_json = rooms_json.clone();
        use_effect_with((), move |_| {
            let rooms_json = rooms_json.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_dorms_json: Vec<RoomJSON> = Request::get(format!("http://localhost:8081/dorms/{}/rooms", dorm_id).as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                rooms_json.set(fetched_dorms_json);
            });
            || ()
        });
    }

    let rooms : Vec<RoomJSON> = rooms_json.to_vec();

    let rooms_html = rooms.iter().map(|room| {
      let on_card_select = {
        let room_on_click = props.room_on_click.clone();
        let room = room.clone();
        let room_state = RoomStateInput {
          room_id: room.id,
          student_id: 0,
        };
        Callback::from(move |_| {
          let room_state = room_state.clone();
          // Reserve Button
          log!(room_state.room_id.clone());
          wasm_bindgen_futures::spawn_local(async move {
            Request::get(format!("http://localhost:8081/rooms/reserve/{}/{}", room_state.room_id, room_state.student_id).as_str()).send().await;
          });
        })
      };
      
      html_nested! {
        <>
          <div class="col-sm-2">
            <div class="card">
              <div class="card-body">
                <h5 class="card-title">{format!("Room Number: {}", room.room_number)}</h5>
                <h6 class="card-title">{format!("Current Occupants: {} / {}", room.occupants, room.max_occupants)}</h6>
                <button onclick={on_card_select} class="stretched-link btn btn-primary">{"Reserve"}</button>
              </div>
            </div>
          </div>
        </>
      }
      }).collect::<Vec<_>>();

      let on_return_select = {
        let on_click = props.on_click.clone();
        Callback::from(move |_| {
          on_click.emit(true);
        })
      };

    html! {
        <>
        <button onclick={on_return_select} class="btn btn-primary">{"Return"}</button>
        <h2>{format!("Dorm: {}", &*dorm_json.dorm_name)}</h2>
        <h3>{"Rooms:"}</h3>
            <div class="row">
                {rooms_html}
            </div>
        </>
    }
}