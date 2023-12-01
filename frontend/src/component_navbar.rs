use yew::{function_component, html, Html, Callback, virtual_dom::VNode};
use gloo::storage::SessionStorage;
use gloo_storage::Storage;

use crate::component_login::*;



#[function_component]
pub fn Navbar() -> Html {
  let session: Result<StudentLogin, _> = SessionStorage::get("user");
  let user: VNode = match session {
    Ok(v) => html!{"signed in"},
    _ => html!{"not signed in"},
  };

  html! {
    <nav class="navbar bg-body-tertiary">
      <div class="container-fluid">
        <a class="navbar-brand">{"Room Reservation"}</a>
        {user}
      </div>
    </nav>
  }
}