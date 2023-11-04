// @generated automatically by Diesel CLI.

diesel::table! {
    classes (id) {
        id -> Int4,
        class_name -> Varchar,
        teacher -> Nullable<Varchar>,
        max_students -> Int4,
        registered_students -> Int4,
        published -> Bool,
    }
}

diesel::table! {
    classes_students (class_id, student_id) {
        class_id -> Int4,
        student_id -> Int4,
    }
}

diesel::table! {
    students (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        uin -> Int4,
        net_id -> Nullable<Varchar>,
    }
}

diesel::joinable!(classes_students -> classes (class_id));
diesel::joinable!(classes_students -> students (student_id));

diesel::allow_tables_to_appear_in_same_query!(
    classes,
    classes_students,
    students,
);
