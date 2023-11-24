use diesel::prelude::*;
use backend::{schema::{rooms_students_holds, rooms_students_reservations}, models::{RoomsStudentsHolds, Room, Student, RoomsStudentsReservations}, establish_connection};
use rocket::{serde::{json::Json, Serialize, Deserialize}, Route, http::Status};

#[put("/rooms/hold", data="<form_input>")]
fn put_hold_room(form_input: Json<RoomStateInput>) -> Status {
  use backend::schema::rooms::dsl::*;
  use backend::schema::students::dsl::*;

  let room_id = form_input.room_id;
  let student_id = form_input.student_id;

  let connection = &mut establish_connection();

  let room: Room = match rooms.find(room_id)
                              .select(Room::as_select())
                              .first(connection)
                              .optional().expect("failed to access rooms table") {
    Some(v) => v,
    None => return Status::BadRequest,
  };

  let holders: Vec<Student> = RoomsStudentsHolds::belonging_to(&room).inner_join(students).select(Student::as_select()).load(connection).expect("failed to load students holding room");

  let reservers: Vec<Student> = RoomsStudentsReservations::belonging_to(&room).inner_join(students).select(Student::as_select()).load(connection).expect("failed to load students holding room");

  if holders.is_empty() && reservers.is_empty() {
      diesel::insert_into(rooms_students_holds::table).values(RoomsStudentsHolds {room_id, student_id}).execute(connection).expect("failed to create new hold");
  } else {
    return Status::BadRequest;
  }

  Status::Ok
}

#[put("/rooms/reserve", data="<form_input>")]
fn put_reserve_room(form_input: Json<RoomStateInput>) -> Status {
  use backend::schema::rooms::dsl::*;
  use backend::schema::students::dsl::*;

  let room_id = form_input.room_id;
  let student_id = form_input.student_id;

  let connection = &mut establish_connection();

  let room: Room = match rooms.find(room_id).select(Room::as_select()).first(connection).optional().expect("failed to access rooms table") {
    Some(v) => v,
    None => return Status::BadRequest,
  };

  let student: Student = match students.find(student_id).select(Student::as_select()).first(connection).optional().expect("failed to access students table") {
    Some(v) => v,
    None => return Status::BadRequest,
  };

  let holders: Vec<Student> = RoomsStudentsHolds::belonging_to(&room)
    .inner_join(students)
    .select(Student::as_select())
    .load(connection).expect("failed to load students holding room");

  let reservers: Vec<Student> = RoomsStudentsReservations::belonging_to(&room)
    .inner_join(students)
    .select(Student::as_select())
    .load(connection).expect("failed to load students holding room");

  if reservers.len() >= room.max_occupants as usize || room.occupants >= room.max_occupants {
    return Status::BadRequest;
  }

  if holders.is_empty() || holders.contains(&student) {
    diesel::insert_into(rooms_students_reservations::table).values(RoomsStudentsReservations {room_id, student_id}).execute(connection).expect("failed to create new reservation");
    diesel::update(&room).set(occupants.eq(occupants + 1)).execute(connection).expect("failed to increment occupant num");
  } else {
    return Status::BadRequest;
  }

  Status::Ok
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
struct RoomStateInput {
  room_id: i32,
  student_id: i32,
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