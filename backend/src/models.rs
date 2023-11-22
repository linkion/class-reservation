use diesel::prelude::*;

use crate::schema::*;

use rocket::{serde::Serialize, form::FromForm};

pub enum DormGroups {
    urbana_north,
    urbana_south,
    ikenberry_commons_north,
    ikenberry_commons_south,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = dormitories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Dormitory {
    pub id: i32,
    pub dorm_name: String,
    pub dorm_group: String,
}

/*
CREATE TYPE locations AS ENUM ('Urbana North', 'Urbana South', 'Ikenberry Commons North', 'Ikenberry Commons South');
CREATE TABLE dormitories (
  id SERIAL PRIMARY KEY,
  dorm_name VARCHAR NOT NULL,
  dorm_group VARCHAR NOT NULL
); 
*/

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Room {
    pub id: i32,
    pub room_number: i32,
    pub max_occupants: i32,
    pub occupants: i32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = students)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Student {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Room))]
#[diesel(belongs_to(Student))]
#[diesel(table_name = rooms_students)]
#[derive(Insertable)]
#[diesel(primary_key(room_id, student_id))]
pub struct RoomsStudent {
    pub room_id: i32,
    pub student_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = dormitories)]
pub struct NewDorm<'a> {
    pub dorm_name: &'a str,
    pub dorm_group: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = rooms)]
pub struct NewRoom<'a> {
    pub room_number: &'a i32,
    pub max_occupants: &'a i32,
    pub occupants: &'a i32,
}

#[derive(Insertable)]
#[diesel(table_name = students)]
pub struct NewStudent<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub middle_name: &'a str,
}