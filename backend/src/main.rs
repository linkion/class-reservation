use rocket::fs::NamedFile;

#[cfg(test)] mod tests;
#[macro_use] extern crate rocket;
mod routes_rooms;
mod routes_students;
mod routes_dorms;
mod routes_rooms_reservation;

#[get("/")]
async fn index() -> NamedFile {
    NamedFile::open("static/index.html").await.expect("index.html not found")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes_rooms::routes())
        .mount("/", routes_students::routes())
        .mount("/", routes_dorms::routes())
}

