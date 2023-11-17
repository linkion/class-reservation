use backend::models::{NewClass, Class};
use backend::*;
use rocket::{State, route, Route};
use rocket::form::Form;
use rocket::serde::{json::Json, Serialize, Deserialize};
use diesel::{prelude::*, result};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;

#[get("/classes/<id>")]
fn get_class(id: i32) -> Json<Class> {
    use backend::schema::classes::dsl::classes;

    let connection = &mut establish_connection();

    let class = classes.find(id).select(Class::as_select()).first(connection).expect("class not found");
    Json(class)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ClassesJson {
    classes: Vec<Class>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ClassJson {
    pub id: i32,
    pub class_name: String,
    pub max_students: i32,
    pub registered_students: i32,
    pub subject_code: String,
    pub course_number: i32,
    pub teacher_name: Option<String>,
    pub links: Vec<LinkJson>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LinkJson {
    href: String,
    rel: String,
    method: rocket::http::Method,
}

#[get("/classes")]
fn get_classes() -> Json<ClassesJson> {
    use backend::schema::classes::dsl::classes;

    let connection = &mut establish_connection();

    let results = classes.select(Class::as_select()).load(connection).expect("Failed to load classes");
    Json(ClassesJson { classes: results })
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(FromForm)]
struct ClassInput<> {
    name: String,
    max_students: i32,
    subject_code: String,
    course_number: i32,
}

#[post("/classes", data="<form_input>")]
async fn post_class_json(form_input: Json<ClassInput>, queue: &State<Sender<Class>>) -> Json<Class> {
    use backend::schema::classes;

    let connection = &mut establish_connection();

    let new_class = NewClass {class_name: &form_input.name,max_students: &form_input.max_students, subject_code: &form_input.subject_code.to_ascii_uppercase(), course_number: &form_input.course_number };
    
    let result = diesel::insert_into(classes::table).values(new_class).returning(Class::as_returning()).get_result(connection).expect("failed to insert class");
    let _res = queue.send(result.clone());
    Json(result)
}

#[post("/classes", data="<form_input>", rank=1)]
async fn post_class(form_input: Form<ClassInput>, queue: &State<Sender<Class>>) -> Json<Class> {
    use backend::schema::classes;

    let connection = &mut establish_connection();

    let new_class = NewClass {class_name: &form_input.name,max_students: &form_input.max_students, subject_code: &form_input.subject_code.to_ascii_uppercase(), course_number: &form_input.course_number };
    
    let result = diesel::insert_into(classes::table).values(new_class).returning(Class::as_returning()).get_result(connection).expect("failed to insert class");
    
    let _res = queue.send(result.clone());
    Json(result)
}

pub fn routes() -> Vec<Route> {
  routes![get_class, get_classes, post_class, post_class_json]
}