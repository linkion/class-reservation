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
    
    let view_html = match *view {
        CurrentView::Dorms => html!(<DormList on_click={on_click_dorm} />),
        CurrentView::Rooms(dorm_id) => html!(<RoomList dorm_id={dorm_id} />),
    };

    html! {
      <>
          {include_cdn()}
          <h1>{ "UIUC Dorm Reservation" }</h1>
          <div>
              {view_html}
          </div>
          {include_cdn_js()}
      </>
  }
}

fn main() {
    yew::Renderer::<App>::new().render();
}