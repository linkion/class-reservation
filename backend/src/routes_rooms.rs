use backend::*;
use backend::models::*;
use rocket::Route;
use rocket::form::Form;
use rocket::serde::{json::Json, Serialize, Deserialize};
use diesel::prelude::*;

// CREATE new room at given dormitory (dorm_id)
#[post("/rooms", data="<form_input>", rank=0)]
fn post_room(form_input: Form<RoomInput>) -> Json<RoomJsonRet> {
    use backend::schema::dormitories::dsl::*;
    use backend::schema::dormitories_rooms::dsl::*;

    let new_room = NewRoom { room_number: &form_input.room_number, max_occupants: &form_input.max_occupants };

    let connection = &mut establish_connection();

    let new_room_ret: Room = diesel::insert_into(backend::schema::rooms::table).values(new_room).returning(Room::as_returning()).get_result(connection).expect("failed to insert room");
    
    let dorm_room_link = DormitoriesRooms { dorm_id: form_input.dorm_id, room_id: new_room_ret.id };
    let dorm: Dorm = dormitories.find(form_input.dorm_id).select(Dorm::as_select()).first(connection).expect("failed to find dorm");

    diesel::insert_into(dormitories_rooms).values(dorm_room_link).execute(connection).expect("failed to link dorm with new room");

    diesel::update(&dorm).set(rooms_available.eq(rooms_available + 1)).execute(connection).expect("failed to update dorm with new room");
    diesel::update(&dorm).set(rooms.eq(rooms + 1)).execute(connection).expect("failed to update dorm with new room");

    Json(RoomJsonRet { 
        id: new_room_ret.id, 
        room_number: new_room_ret.room_number, 
        max_occupants: new_room_ret.max_occupants, 
        occupants: new_room_ret.occupants, 
        links: vec![],
    })
}

#[post("/rooms", data="<form_input>", rank=1)]
fn post_room_json(form_input: Json<RoomInput>) -> Json<RoomJsonRet> {
    use backend::schema::dormitories::dsl::*;
    use backend::schema::dormitories_rooms::dsl::*;

    let new_room = NewRoom { room_number: &form_input.room_number, max_occupants: &form_input.max_occupants };

    let connection = &mut establish_connection();

    let new_room_ret: Room = diesel::insert_into(backend::schema::rooms::table).values(new_room).returning(Room::as_returning()).get_result(connection).expect("failed to insert room");
    
    let dorm_room_link = DormitoriesRooms { dorm_id: form_input.dorm_id, room_id: new_room_ret.id };
    let dorm: Dorm = dormitories.find(form_input.dorm_id).select(Dorm::as_select()).first(connection).expect("failed to find dorm");

    diesel::insert_into(dormitories_rooms).values(dorm_room_link).execute(connection).expect("failed to link dorm with new room");

    diesel::update(&dorm).set(rooms_available.eq(rooms_available + 1)).execute(connection).expect("failed to update dorm with new room");
    diesel::update(&dorm).set(rooms.eq(rooms + 1)).execute(connection).expect("failed to update dorm with new room");

    Json(RoomJsonRet { 
        id: new_room_ret.id, 
        room_number: new_room_ret.room_number, 
        max_occupants: new_room_ret.max_occupants, 
        occupants: new_room_ret.occupants, 
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

// RETURN all rooms within dorm
#[get("/dorms/<dorm_id>/rooms")]
fn get_dorm_rooms(dorm_id: i32) -> Json<Vec<RoomJsonRet>> {


    let connection = &mut establish_connection();

    let selected_dorm: Dorm = schema::dormitories::table.find(dorm_id).select(Dorm::as_select()).first(connection).expect("failed to find dorm");

    let rooms: Vec<Room> = DormitoriesRooms::belonging_to(&selected_dorm).inner_join(backend::schema::rooms::table).select(Room::as_select()).load(connection).expect("failed to load rooms associated with dorm");
    let mut return_rooms: Vec<RoomJsonRet> = vec![];

    for iter_room in rooms {
        let id = iter_room.id;
        let room_number = iter_room.room_number;
        let max_occupants = iter_room.max_occupants;
        let occupants = iter_room.occupants;
        let links: Vec<LinkJson> = vec![];
        return_rooms.push(RoomJsonRet { id, room_number, max_occupants, occupants, links });
    }

    Json(return_rooms)
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
  routes![post_room, post_room_json, get_room, get_all_rooms, get_dorm_rooms, delete_room]
}