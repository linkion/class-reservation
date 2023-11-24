use backend::*;
use backend::models::{NewRoom, Room, DormitoriesRooms};
use rocket::Route;
use rocket::form::Form;
use rocket::serde::{json::Json, Serialize, Deserialize};
use diesel::prelude::*;

// CREATE new room at given dormitory (dorm_id)
#[post("/rooms", data="<form_input>", rank=0)]
fn post_room(form_input: Form<RoomInput>) -> Json<RoomJsonRet> {
    use backend::schema::rooms;
    use backend::schema::dormitories_rooms;

    let new_room = NewRoom { room_number: &form_input.room_number, max_occupants: &form_input.max_occupants };

    let connection = &mut establish_connection();

    let result: Room = diesel::insert_into(rooms::table).values(new_room).returning(Room::as_returning()).get_result(connection).expect("failed to insert room");
    
    let dorm_room_link = DormitoriesRooms { dorm_id: form_input.dorm_id, room_id: result.id };

    let _ = diesel::insert_into(dormitories_rooms::table).values(dorm_room_link).execute(connection).expect("failed to link dorm with new room");

    Json(RoomJsonRet { 
        id: result.id, 
        room_number: result.room_number, 
        max_occupants: result.max_occupants, 
        occupants: result.occupants, 
        links: vec![],
    })
}

#[post("/rooms", data="<form_input>", rank=1)]
fn post_room_json(form_input: Json<RoomInput>) -> Json<RoomJsonRet> {
    use backend::schema::rooms;

    let new_room = NewRoom { room_number: &form_input.room_number, max_occupants: &form_input.max_occupants };

    let connection = &mut establish_connection();

    let result: Room = diesel::insert_into(rooms::table).values(new_room).returning(Room::as_returning()).get_result(connection).expect("failed to insert room");
    
    Json(RoomJsonRet { 
        id: result.id, 
        room_number: result.room_number, 
        max_occupants: result.max_occupants, 
        occupants: result.occupants, 
        links: vec![],
    })
}

// RETURN single room with room_id
#[get("/rooms/<room_id>", rank=0)]
fn get_room(room_id: i32) -> Json<RoomJsonRet> {
    use backend::schema::rooms::dsl::rooms;

    let connection = &mut establish_connection();

    let result: Room = rooms.find(room_id).select(Room::as_select()).first(connection).expect("room not found");

    Json(RoomJsonRet { 
        id: result.id, 
        room_number: result.room_number, 
        max_occupants: result.max_occupants, 
        occupants: result.occupants, 
        links: vec![],
    })
}

#[get("/rooms")]
fn get_all_rooms() -> Json<Vec<RoomJsonRet>> {
  use backend::schema::rooms::dsl::*;

  let connection = &mut establish_connection();

  let results = rooms.select(Room::as_select()).load(connection).expect("failed to load dorms");

  let mut results_json: Vec<RoomJsonRet> = vec![];

  for item in results.iter() {
    let new_dorm_json = RoomJsonRet { id: item.id, room_number: item.room_number, max_occupants: item.max_occupants, occupants: item.occupants, links: vec![] };
    results_json.push(new_dorm_json);
  }

  Json(results_json)
}

// UPDATE will be in routes_rooms_reservation.rs

// DELETE room given dorm_id and room_id
#[delete["/rooms/<dorm_id>/<room_number>"]]
fn delete_room(dorm_id: i32, room_number: i32) -> Json<RoomJsonRet> {
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

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(FromForm)]
struct RoomInput {
    room_number: i32,
    max_occupants: i32,
    dorm_id: i32,
}

pub fn routes() -> Vec<Route> {
  routes![post_room, post_room_json, get_room, get_all_rooms, delete_room]
}