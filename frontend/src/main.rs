use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;


struct Class {
    id: i32,
    class_name: String,
    max_students: i32,
    registered_students: i32,
    teacher: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DormJSON {
    pub id: i32,
    pub dorm_name: String,
    pub dorm_group: String,
    pub rooms: i32,
    pub rooms_available: i32,
}

#[function_component(App)]
fn app() -> Html {
    
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
    }/* 

    let rooms_json = use_state(|| vec![]);
    {
        let rooms_json = rooms_json.clone();
        use_effect_with((), move |_| {
            let rooms_json = rooms_json.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_rooms_json: Vec<DormJSON> = Request::get("http://localhost:8081/rooms")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                rooms_json.set(fetched_rooms_json);
            });
            || ()
        })
    } */

    let dorms : Vec<DormJSON> = dorms_json.to_vec();

    let classes = vec![
        Class {
            id: 1,
            class_name: "Intro to CS II".to_string(),
            max_students: 250,
            registered_students: 47,
            teacher: "Michael Novak".to_string(),
        },
        Class {
            id: 2,
            class_name: "Calculus II".to_string(),
            max_students: 200,
            registered_students: 89,
            teacher: "Jeremiah Heller".to_string(),
        },
    ];
    let classes = classes.iter().map(|class| html! {
        <p key={class.id}>{format!("{} . . . . . . {} . . . . . . {} . . . . . . {}", class.teacher, class.class_name, class.max_students.to_string(), class.registered_students.to_string())}</p>
    }).collect::<Html>();
    let dorms = dorms.iter().map(|dorm| html! {
        <p key={dorm.id}>{format!("{} . . . . . . {}", dorm.dorm_name, dorm.dorm_group)}</p>
    }).collect::<Html>();
    html! {
        <>
            <h1>{ "UIUC Class Reservation" }</h1>
            <div>
                <h3>{"Classes: "}</h3>
                {dorms}
                <button>{"Hi!"}</button>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}