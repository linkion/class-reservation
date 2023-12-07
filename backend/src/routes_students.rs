use backend::models::{Student, NewStudent};
use backend::*;
use rocket::Route;
use rocket::serde::{json::Json, Serialize, Deserialize};
use diesel::prelude::*;

// CREATE student
#[post("/students", data="<form_input>")]
async fn post_student(form_input: Json<StudentInput>) -> Json<Student> {
    use backend::schema::students;

    let connection = &mut establish_connection();

    let new_student = NewStudent { first_name: &form_input.first_name, last_name: &form_input.last_name };

    let result = diesel::insert_into(students::table).values(new_student).returning(Student::as_returning()).get_result(connection).expect("failed to insert student");

    Json(result)
}

// RETURN student given id
#[get("/students/<id>")]
fn get_student(id: i32) -> Json<StudentJsonRet> {
  use backend::schema::students::dsl::students;

  let connection = &mut establish_connection();
  let student: Student = students.find(id).select(Student::as_select()).first(connection).expect("student not found");

  Json(StudentJsonRet { id, first_name: student.first_name, last_name: student.last_name, links: vec![] })
}

// UPDATE

// DELETE

// STRUCTS
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct StudentJsonRet {
    id: i32,
    first_name: String,
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
}

pub fn routes() -> Vec<Route> {
  routes![get_student, post_student]
}