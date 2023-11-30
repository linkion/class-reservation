use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;

use yew_bootstrap::component::*;
use yew_bootstrap::component::card::*;

#[derive(Deserialize, Debug, Clone)]
pub struct RoomJSON {
    id: i32,
    room_number: i32,
    max_occupants: i32,
    occupants: i32,
}

#[function_component]
pub fn RoomList() -> Html {
    let dorm_id = 32;
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

    let rooms = rooms.iter().map(|room| html! {
        <p key={room.id}>{format!("{} . . . . . . {} . . . . . . {}", room.room_number, room.max_occupants, room.occupants)}</p>
    }).collect::<Html>();

    html! {
        {rooms}
    }
}