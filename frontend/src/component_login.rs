use std::ops::Deref;

use gloo::console::log;
use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{function_component, html, Html, Callback};
use serde::{Deserialize, Serialize};
use gloo::storage::LocalStorage;
use gloo_storage::{Storage, SessionStorage};

#[derive(Serialize, Deserialize, Clone)]
pub struct StudentLogin {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
}

#[derive(Default, Clone, Serialize)]
pub struct LoginData {
  pub first_name: String,
  pub last_name: String,
}

#[function_component]
pub fn StudentLoginHTML() -> Html {
  let mut stored_student_logins: Vec<StudentLogin> = match LocalStorage::get("student_logins") {
      Ok(v) => v,
      _ => vec![],
  };
  
  let state = use_state(|| LoginData::default());

  let cloned_state = state.clone();
  let first_name_changed = Callback::from(move |event: Event| {
    let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
    let mut data = cloned_state.deref().clone();
    data.first_name = value.clone();
    log!(value);
    cloned_state.set(data);
  });

  let cloned_state = state.clone();
  let last_name_changed = Callback::from(move |event: Event| {
    let value = event.target().unwrap().unchecked_into::<HtmlInputElement>().value();
    let mut data = cloned_state.deref().clone();
    data.last_name = value.clone();
    log!(value);
    cloned_state.set(data);
  });


  let on_submit = Callback::from(move |_event| {
    let cloned_state = state.clone();
    let mut stored_student_logins = stored_student_logins.clone();
    wasm_bindgen_futures::spawn_local(async move {
      let response: StudentLogin = Request::post("http://localhost:8081/students").json(cloned_state.deref())
        .unwrap()
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
      
      let _ = SessionStorage::set("current_login", response.clone());
      stored_student_logins.push(response.clone());
      let _ = LocalStorage::set("student_logins", stored_student_logins);
      log!(response.id);
    });
  });

  html! {
    <>
      <form class="form-horizontal" method="get" accept-charset="UTF-8">
          <div class="login-form">
          <label for="studentFirstName">{"First Name"}</label>
          <input type="firstName" onchange={first_name_changed} class="form-control login" id="studentFirstName" placeholder="First Name" />
        </div>
        <div class="form-group">
          <label for="studentLastName">{"Last Name"}</label>
          <input type="lastName" onchange={last_name_changed} class="form-control login" id="studentLastName" placeholder="Last Name" />
        </div>
        <br/>
        <button type="submit" onclick={on_submit} class="btn btn-success">{"Submit"}</button>
      </form>
    </>
  }
}