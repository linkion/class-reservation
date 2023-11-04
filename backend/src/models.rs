use diesel::prelude::*;

use crate::schema::{classes, students, classes_students};

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = classes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Class {
    pub id: i32,
    pub class_name: String,
    pub teacher: Option<String>,
    pub max_students: i32,
    pub registered_students: i32,
    pub published: bool,
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = students)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Student {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub uin: i32,
    pub net_id: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = teachers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClassesStudent {
    pub class_id: i32,
    pub student_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = classes)]
pub struct NewClass<'a> {
    pub class_name: &'a str,
    pub teacher: &'a str,
    pub max_students: &'a i32,
}