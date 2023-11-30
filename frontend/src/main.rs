use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_bootstrap::util::*;

mod component_dorms;
mod component_rooms;

use crate::component_dorms::*;
use crate::component_rooms::*;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum CurrentView {
    Dorms,
    Rooms(i32),
}

#[function_component(App)]
fn app() -> Html {
    let view: UseStateHandle<CurrentView> = use_state(|| CurrentView::Dorms);
    let callback_view = view.clone();
    let on_click_dorm = Callback::from(move |dorm_id: i32| {
        callback_view.set(CurrentView::Rooms(dorm_id));
    });
    
    match *view {
        CurrentView::Dorms => return html! {
            <>
                {include_cdn()}
                <h1>{ "UIUC Class Reservation" }</h1>
                <div>
                    <DormList on_click={on_click_dorm} />
                </div>
                {include_cdn_js()}
            </>
        },
        CurrentView::Rooms(dorm_id) => return html! {
            <>
                {include_cdn()}
                <h1>{ "UIUC Class Reservation" }</h1>
                <div>
                    <RoomList dorm_id={dorm_id} />
                </div>
                {include_cdn_js()}
            </>
        },
    };
}

fn main() {
    yew::Renderer::<App>::new().render();
}