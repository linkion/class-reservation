use backend::*;
use backend::models::*;
use rocket::Route;
use rocket::form::Form;
use rocket::serde::{json::Json, Serialize, Deserialize};
use diesel::prelude::*;

// CREATE
#[post("/dorms", data="<form_input>", rank=0)]
fn post_dorm(form_input: Form<DormInput>) -> Json<DormJsonRet> {
  use backend::schema::dormitories;

  let new_dorm = NewDorm { dorm_name: &form_input.dorm_name, dorm_group: &form_input.dorm_group };

  let connection = &mut establish_connection();

  let result: Dorm = diesel::insert_into(dormitories::table).values(new_dorm).returning(Dorm::as_returning()).get_result(connection).expect("failed to insert room");

  let id = result.id;
  let dorm_name = result.dorm_name;
  let dorm_group = result.dorm_group;
  let rooms = result.rooms;
  let rooms_available = result.rooms_available;
  let links: Vec<LinkJson> = vec![];
  Json(DormJsonRet {id,dorm_name,dorm_group,links, rooms, rooms_available })
}

#[post("/dorms", data="<form_input>", rank=1)]
fn post_dorm_json(form_input: Json<DormInput>) -> Json<DormJsonRet> {
  use backend::schema::dormitories;

  let new_dorm = NewDorm { dorm_name: &form_input.dorm_name, dorm_group: &form_input.dorm_group };

  let connection = &mut establish_connection();

  let result: Dorm = diesel::insert_into(dormitories::table).values(new_dorm).returning(Dorm::as_returning()).get_result(connection).expect("failed to insert room");

  let id: i32 = result.id;
  let dorm_name: String = result.dorm_name;
  let dorm_group: String = result.dorm_group;
  let rooms = result.rooms;
  let rooms_available = result.rooms_available;
  let links: Vec<LinkJson> = vec![];
  Json(DormJsonRet {id,dorm_name,dorm_group,links, rooms, rooms_available })
}

// RETURN
#[get("/dorms/<id>")]
fn get_dorm(id: i32) -> Json<DormJsonRet> {
  use backend::schema::dormitories::dsl::*;

  let connection = &mut establish_connection();

  let result: Dorm = dormitories.find(id).select(Dorm::as_select()).first(connection).expect("failed to find dorm");

  let links: Vec<LinkJson> = vec![];
  Json(DormJsonRet {id:result.id,dorm_name:result.dorm_name.clone(),dorm_group:result.dorm_group.clone(),links, rooms: result.rooms, rooms_available: result.rooms_available })
}

#[get("/dorms")]
fn get_all_dorms() -> Json<Vec<DormJsonRet>> {
  use backend::schema::dormitories::dsl::*;

  let connection = &mut establish_connection();

  let results = dormitories.select(Dorm::as_select()).load(connection).expect("failed to load dorms");

  let mut results_json: Vec<DormJsonRet> = vec![];

  for item in results.iter() {
    let new_dorm_json = DormJsonRet {id:item.id,dorm_name:item.dorm_name.clone(),dorm_group:item.dorm_group.clone(),links:vec![], rooms: item.rooms, rooms_available: item.rooms_available };
    results_json.push(new_dorm_json);
  }

  Json(results_json)
}

// UPDATE

// DELETE

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct DormJsonRet {
    id: i32,
    dorm_name: String,
    dorm_group: String,
    rooms: i32,
    rooms_available: i32,
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
struct DormInput {
    dorm_name: String,
    dorm_group: String,
}

pub fn routes() -> Vec<Route> {
  routes![post_dorm, post_dorm_json, get_dorm, get_all_dorms]
}