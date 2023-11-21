use backend::models::{Student, NewStudent};
use backend::*;
use rocket::{State, route, Route};
use rocket::form::Form;
use rocket::serde::{json::Json, Serialize, Deserialize};
use diesel::{prelude::*, result};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct StudentJsonRet {
    id: i32,
    first_name: String,
    middle_name: Option<String>,
    last_name: String,
    links: Vec<LinkJson>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LinkJson {
  href: String,
  rel: String,
  method: rocket::http::Method,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(FromForm)]
pub struct StudentInput {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String
}

#[get("/students/<id>")]
fn get_student(id: i32) -> Json<StudentJsonRet> {
  use backend::schema::students::dsl::students;

  let connection = &mut establish_connection();
  let student: Student = students.find(id).select(Student::as_select()).first(connection).expect("class not found");

  Json(StudentJsonRet { id, first_name: student.first_name, middle_name: student.middle_name, last_name: student.last_name, links: vec![] })
}

#[post("/students", data="<form_input>")]
async fn post_student(form_input: Json<StudentInput>) -> Json<Student> {
    use backend::schema::students;

    let connection = &mut establish_connection();

    let new_student = NewStudent { first_name: &form_input.first_name, last_name: &form_input.last_name, middle_name: &form_input.middle_name };

    let result = diesel::insert_into(students::table).values(new_student).returning(Student::as_returning()).get_result(connection).expect("failed to insert student");

    Json(result)
}

pub fn routes() -> Vec<Route> {
  routes![get_student, post_student]
}