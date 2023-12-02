use yew::{function_component, html, Html, Callback, virtual_dom::VNode};
use gloo::storage::SessionStorage;
use gloo_storage::Storage;

use crate::component_login::*;



#[function_component]
pub fn Navbar() -> Html {
  let session: Result<StudentLogin, _> = SessionStorage::get("user");
  let user: VNode = match session {
    Ok(v) => html!{
      <>
      <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
        {"Signed in"}
      </a>
      <ul class="dropdown-menu">
        <li><a class="dropdown-item" href="#">{"See Selected Dorms"}</a></li>
        <li><a class="dropdown-item" href="#">{"Sign Out"}</a></li>
      </ul>
      </>
    },
    _ => html!{
      <>
        <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
          {"Sign In"}
        </a>
        <ul class="dropdown-menu">
          <li>
            <div style="padding: 20px; min-width: 300px;">
              <StudentLoginHTML />
            </div>
          </li>
        </ul>
      </>
    },
  };

  html! {
    <nav class="navbar navbar-expand-lg bg-body-tertiary">
      <div class="container-fluid">
        <div class="navbar-header">
          <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNavDropdown" aria-controls="navbarNavDropdown" aria-expanded="false" aria-label="Toggle navigation">
          <span class="navbar-toggler-icon"></span>
          </button>
          <a class="navbar-brand">{"Room Reservation"}</a>
        </div>
        <div class="collapse navbar-collapse" id="navbarNavDropdown">
          <ul class="navbar-nav d-flex">
            <li class="nav-item dropdown">
              {user}
            </li>
          </ul>
        </div>
      </div>
    </nav>
  }
}