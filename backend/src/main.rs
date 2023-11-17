use backend::schema::classes_students;
use backend::{*, models::*};
use rocket::{State, Shutdown};
use diesel::{prelude::*, result};
use rocket::form::Form;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::fs::NamedFile;
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;

#[cfg(test)] mod tests;
#[macro_use] extern crate rocket;
mod class_routes;

#[get("/")]
async fn index() -> NamedFile {
    let file = NamedFile::open("static/index.html").await.expect("index.html not found");
    file
}



#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(FromForm)]
pub struct StudentInput {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String
}

#[post("/students", data="<form_input>")]
async fn post_student(form_input: Json<StudentInput>) -> Json<Student> {
    use backend::schema::students;

    let connection = &mut establish_connection();

    let new_student = NewStudent { first_name: &form_input.first_name, last_name: &form_input.last_name, middle_name: &form_input.middle_name };

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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(FromForm)]
pub struct RegisterStudent {
    student_id: i32,
    class_id: i32,
}

#[put("/register", data="<form_input>")]
async fn register_student(form_input: Form<RegisterStudent>) {
    use backend::schema::classes_students;

    let connection = &mut establish_connection();

    let new_classes_student = ClassesStudent { class_id: form_input.class_id, student_id: form_input.student_id };

    let _ = diesel::insert_into(classes_students::table).values(new_classes_student).execute(connection).expect("failed to relate student to class");
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Class>(50_000).0)
        .mount("/", class_routes::routes())
        .mount("/", routes![index])
        .mount("/", routes![classes_events])
        .mount("/", routes![post_student])
        .mount("/", routes![register_student])
}

