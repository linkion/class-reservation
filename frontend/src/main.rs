use yew::prelude::*;
use yew_bootstrap::util::*;

mod component_dorms;
mod component_rooms;

use crate::component_dorms::*;
use crate::component_rooms::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            {include_cdn()}
            <h1>{ "UIUC Dorm Reservation" }</h1>
            <div>
                <h3>{"Dorms: "}</h3>
                <DormList />
                <RoomList dorm_id=1/>
            </div>
            {include_cdn_js()}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}