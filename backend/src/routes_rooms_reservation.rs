use rocket::{serde::{json::Json, Serialize}, Route};

#[put("/rooms/hold/<room_id>")]
fn put_hold_room(room_id: i32) -> Json<RoomJsonRet> {
    todo!()
}

#[put("/rooms/reserve/<room_id>")]
fn put_reserve_room(room_id: i32) -> Json<RoomJsonRet> {
    todo!()
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RoomJsonRet {
    id: i32,
    room_number: i32,
    max_occupants: i32,
    occupants: i32,
    links: Vec<LinkJson>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LinkJson {
    href: String,
    rel: String,
    method: rocket::http::Method,
}

pub fn routes() -> Vec<Route> {
    routes![put_hold_room, put_reserve_room]
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