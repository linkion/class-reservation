use diesel::prelude::*;

use crate::schema::{classes, students, teachers, classes_students, classes_teachers};

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = classes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Class {
    pub id: i32,
    pub class_name: String,
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
pub struct Teacher {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Class))]
#[diesel(belongs_to(Student))]
#[diesel(table_name = classes_students)]
#[diesel(primary_key(class_id, student_id))]
pub struct ClassesStudent {
    pub class_id: i32,
    pub student_id: i32,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Class))]
#[diesel(belongs_to(Teacher))]
#[diesel(table_name = classes_teachers)]
#[diesel(primary_key(class_id, teacher_id))]
pub struct ClassesTeacher {
    pub class_id: i32,
    pub teacher_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = classes)]
pub struct NewClass<'a> {
    pub class_name: &'a str,
    pub max_students: &'a i32,
}

#[derive(Insertable)]
#[diesel(table_name = students)]
pub struct NewStudent<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub middle_name: &'a str,
    pub uin: &'a i32,
}

#[derive(Insertable)]
#[diesel(table_name = teachers)]
pub struct NewTeacher<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub middle_name: &'a str,
    pub email: &'a str,
}