use yew::prelude::*;
use yew::{function_component, html, Html, Callback};
use serde::{Deserialize, Serialize};
use gloo::storage::LocalStorage;
use gloo_storage::Storage;

#[derive(Serialize, Deserialize)]
pub struct StudentLogin {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub middle_name: Option<String>,
}

#[function_component]
pub fn StudentLoginHTML() -> Html {
  let StoredStudentLogins: Vec<StudentLogin> = match LocalStorage::get("student_logins") {
      Ok(v) => v,
      _ => vec![],
  };

  html! {
    <>
      <form class="form-horizontal" method="post" accept-charset="UTF-8">
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