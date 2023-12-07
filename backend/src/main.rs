use diesel::{RunQueryDsl, SelectableHelper, connection};
use rocket::{fs::NamedFile, http::Status};

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

#[post("/reset/data")]
fn reset_data() -> Status {
    use backend::models::*;
    use backend::schema::dormitories::dsl::*;
    use backend::schema::rooms::dsl::*;
    use backend::schema::dormitories_rooms::dsl::*;
    use backend::schema::rooms_students::dsl::*;
    use backend::schema::rooms_students_holds::dsl::*;
    use backend::schema::rooms_students_reservations::dsl::*;


    let connection = &mut backend::establish_connection();

    diesel::delete(dormitories).execute(connection).expect("failed to delete dorms table");
    diesel::delete(backend::schema::students::table).execute(connection).expect("failed to delete student table");
    diesel::delete(backend::schema::rooms::table).execute(connection).expect("failed to delete rooms table");
    diesel::delete(dormitories_rooms).execute(connection).expect("failed to delete dormitories_rooms table");
    diesel::delete(rooms_students).execute(connection).expect("failed to delete rooms_students table");
    diesel::delete(rooms_students_holds).execute(connection).expect("failed to delete rooms_students_holds table");
    diesel::delete(rooms_students_reservations).execute(connection).expect("failed to delete rooms_students_reservations table");


    use csv::{ReaderBuilder, StringRecord};

    let mut rdr = ReaderBuilder::new().from_path("src/dormitories.csv").expect("dormitories.csv not found");

    diesel::insert_into(backend::schema::students::table).values(NewStudent{ first_name: "Hello, ", last_name: "World!" }).execute(connection).expect("failed to insert student");

    let records = rdr
        .records()
        .collect::<Result<Vec<StringRecord>, csv::Error>>().expect("failure to collect csv rows");

    for item in records.iter() {
        let new_dorm_name: String = item[0].to_string();
        let new_dorm_group: String = item[1].trim().to_string();
        let new_dorm_pic: String = item[2].trim().to_string();
        let new_dorm = NewDorm { dorm_name: &new_dorm_name, dorm_group: &new_dorm_group, dorm_pic: &new_dorm_pic };
        let result_dorm: Dorm = diesel::insert_into(dormitories).values(new_dorm).returning(Dorm::as_returning()).get_result(connection).expect("failed to insert new dorm");
        
        let new_dorm_id = result_dorm.id;
        let example_room_nums: Vec<i32> = vec![101,102,103,200,202];
        for room_num in example_room_nums.iter() {
          use crate::routes_rooms::{post_room_json, RoomInput};
          post_room_json(rocket::serde::json::Json(RoomInput { room_number: *room_num, max_occupants: 2, dorm_id: new_dorm_id }));
        }
    }

    Status::Ok
}

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS, PUT"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, reset_data])
        .mount("/", routes_rooms::routes())
        .mount("/", routes_students::routes())
        .mount("/", routes_dorms::routes())
        .mount("/", routes_rooms_reservation::routes())
}

