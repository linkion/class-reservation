use backend::{*, models::*};
use rocket::{State, Shutdown};
use diesel::prelude::*;
use rocket::form::Form;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::fs::NamedFile;
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;

#[cfg(test)] mod tests;
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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(FromForm)]
pub struct NewStudent {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String
}

#[post("/students", data="<form_input>")]
async fn post_student(form_input: Json<NewStudent>) -> Json<Student> {
    use backend::schema::students;

    let connection = &mut establish_connection();

    let new_student = NewStudent { first_name: form_input.first_name, last_name: form_input.last_name, middle_name: form_input.middle_name };

    let result = diesel::insert_into(students::table).values(new_student).returning(Student::as_returning()).get_result(connection).expect("failed to insert student");

    Json(result)
}

#[get("/classes_events")]
async fn classes_events(queue: &State<Sender<Class>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Class>(50_000).0)
        .mount("/", routes![index])
        .mount("/", routes![get_class])
        .mount("/", routes![get_classes])
        .mount("/", routes![post_class])
        .mount("/", routes![post_class_json])
        .mount("/", routes![classes_events])
}

