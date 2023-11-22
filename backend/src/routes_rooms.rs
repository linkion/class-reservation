use backend::*;
use backend::models::{NewRoom, Room, DormGroups, DormitoriesRooms};
use rocket::Route;
use rocket::form::Form;
use rocket::serde::{json::Json, Serialize, Deserialize};
use diesel::prelude::*;

// CREATE new room at given dormitory (dorm_id)
#[post("/rooms", data="<form_input>", rank=0)]
fn post_room(form_input: Form<RoomInput>) -> Json<RoomJsonRet> {
    use backend::schema::rooms;
    use backend::schema::dormitories_rooms;

    let new_room = NewRoom { room_number: &form_input.room_number, max_occupants: &form_input.max_occupants, occupants: &0 };

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

    let new_room = NewRoom { room_number: &form_input.room_number, max_occupants: &form_input.max_occupants, occupants: &0 };

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

/*#[get("/classes/<id>")]
fn get_class(id: i32) -> Json<ClassJsonRet> {
  use backend::schema::classes::dsl::classes;

  let connection = &mut establish_connection();

  let class: Class = classes.find(id).select(Class::as_select()).first(connection).expect("class not found");

  let class_name = class.class_name;
  let max_students = class.max_students;
  let registered_students = class.registered_students;
  let subject_code = class.subject_code;
  let course_number = class.course_number;
  let teacher_name = class.teacher_name;
  let links = vec![
    LinkJson { href: format!("{id}/students"), rel: "students".to_string(), method: rocket::http::Method::Get }, 
    LinkJson { href: format!("{id}/teachers"), rel: "teachers".to_string(), method: rocket::http::Method::Get }
  ];

  Json(ClassJsonRet { id, class_name, max_students, registered_students, subject_code, course_number, teacher_name, links })
}

#[get("/classes")]
fn get_classes() -> Json<ClassesJson> {
    use backend::schema::classes::dsl::classes;

    let connection = &mut establish_connection();

    let results = classes.select(Class::as_select()).load(connection).expect("Failed to load classes");
    Json(ClassesJson { classes: results })
}

#[post("/classes", data="<form_input>", rank=1)]
async fn post_class_json(form_input: Json<ClassInput>, queue: &State<Sender<Class>>) -> Json<Class> {
    use backend::schema::classes;

    let connection = &mut establish_connection();

    let new_class = NewClass {class_name: &form_input.name,max_students: &form_input.max_students, subject_code: &form_input.subject_code.to_ascii_uppercase(), course_number: &form_input.course_number };
    
    let result = diesel::insert_into(classes::table).values(new_class).returning(Class::as_returning()).get_result(connection).expect("failed to insert class");
    let _res = queue.send(result.clone());
    Json(result)
}

#[post("/classes", data="<form_input>", rank=0)]
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
struct ClassInput<> {
    name: String,
    max_students: i32,
    subject_code: String,
    course_number: i32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ClassesJson {
    classes: Vec<Class>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ClassJsonRet {
    pub id: i32,
    pub class_name: String,
    pub max_students: i32,
    pub registered_students: i32,
    pub subject_code: String,
    pub course_number: i32,
    pub teacher_name: Option<String>,
    pub links: Vec<LinkJson>,
}
*/
pub fn routes() -> Vec<Route> {
  routes![post_room, post_room_json, get_room, delete_room]
}