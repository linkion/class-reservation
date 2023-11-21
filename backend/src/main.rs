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
mod routes_rooms;
mod routes_students;

#[get("/")]
async fn index() -> NamedFile {
    let file = NamedFile::open("static/index.html").await.expect("index.html not found");
    file
}

/*

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


*/
#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Room>(50_000).0)
        .mount("/", routes![index])
        .mount("/", routes_rooms::routes())
        .mount("/", routes_students::routes())
}

