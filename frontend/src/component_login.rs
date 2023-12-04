use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{function_component, html, Html, Callback};
use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement};
use gloo::storage::{LocalStorage, SessionStorage};
use gloo_storage::Storage;

#[derive(Serialize, Deserialize, Clone)]
pub struct StudentLogin {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
}

#[function_component]
pub fn StudentLoginHTML() -> Html {
  let StoredStudentLogins: Vec<StudentLogin> = match LocalStorage::get("student_logins") {
      Ok(v) => v,
      _ => vec![],
  };

  let first_name_ref = use_node_ref();
  let last_name_ref = use_node_ref();

  let on_click_past_student = Callback::from(move |student_id: i32| {

  });

  Callback::from(move |event: SubmitEvent| {
    event.prevent_default();

    let first_name: String = "".into();
  });

  let html_prev_students: Vec<VNode> = StoredStudentLogins.iter().map(move |login| {
    let on_click_past_student = on_click_past_student.clone();
    let on_click = {
      let login = login.clone();
      Callback::from(move |_| {on_click_past_student.emit(login.id);})
    };
    html! {
      <>
        <a href="#" onclick={on_click} class="list-group-item list-group-item-action">{login.first_name.clone() + " " + &login.last_name.clone()}</a>
      </>
    }
  }).collect::<Vec<_>>();

  html! {
    <>
      <div class="list-group">
        {html_prev_students}
      </div>
      <form class="form-horizontal" method="get" accept-charset="UTF-8">
          <div class="login-form">
          <label for="studentFirstName">{"First Name"}</label>
          <input type="firstName" class="form-control login" id="studentFirstName" placeholder="First Name" />
        </div>
        <div class="form-group">
          <label for="studentLastName">{"Last Name"}</label>
          <input type="lastName" class="form-control login" id="studentLastName" placeholder="Last Name" />
        </div>
        <br/>
        <button type="submit" class="btn btn-success">{"Submit"}</button>
      </form>
    </>
  }
}