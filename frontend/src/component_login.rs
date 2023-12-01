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
      <form>
          <div class="form-group">
          <label for="studentFirstName">{"First Name"}</label>
          <input type="firstName" class="form-control" id="studentFirstName" placeholder="First Name" />
        </div>
        <div class="form-group">
          <label for="studentLastName">{"Last Name"}</label>
          <input type="lastName" class="form-control" id="studentLastName" placeholder="Last Name" />
        </div>
        <button type="submit" class="btn btn-primary">{"Submit"}</button>
      </form>
    </>
  }
}