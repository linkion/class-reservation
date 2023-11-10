use backend::{*, models::*};
use diesel::prelude::*;
use rocket::form::Form;
use rocket::serde::Serialize;
use rocket::serde::json::Json;
use rocket::fs::NamedFile;

#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> NamedFile {
    let file = NamedFile::open("static/index.html").await.expect("index.html not found");
    file
}

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

#[get("/classes")]
fn get_classes() -> Json<ClassesJson> {
    use backend::schema::classes::dsl::classes;

    let connection = &mut establish_connection();

    let results = classes.select(Class::as_select()).load(connection).expect("Failed to load classes");
    Json(ClassesJson { classes: results })
}

#[derive(FromForm)]
struct ClassInput<> {
    // The raw, undecoded value. You _probably_ want `String` instead.
    name: String,
    max_students: i32,
    subject_code: String,
    course_number: i32,
}

#[post("/classes", data="<form_input>")]
async fn post_class(form_input: Form<ClassInput>) -> Json<Class> {
    use backend::schema::classes;

    let connection = &mut establish_connection();

    let new_class = NewClass {class_name: &form_input.name,max_students: &form_input.max_students, subject_code: &form_input.subject_code.to_ascii_uppercase(), course_number: &form_input.course_number };
    
    let result = diesel::insert_into(classes::table).values(new_class).returning(Class::as_returning()).get_result(connection).expect("failed to insert class");
    
    Json(result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
        .mount("/", routes![get_class])
        .mount("/", routes![get_classes])
        .mount("/", routes![post_class])
}

