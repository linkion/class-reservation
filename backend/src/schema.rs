// @generated automatically by Diesel CLI.

diesel::table! {
    classes (id) {
        id -> Int4,
        class_name -> Varchar,
        max_students -> Int4,
        registered_students -> Int4,
        subject_code -> Varchar,
        course_number -> Int4,
        teacher_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    classes_students (class_id, student_id) {
        class_id -> Int4,
        student_id -> Int4,
    }
}

diesel::table! {
    classes_teachers (class_id, teacher_id) {
        class_id -> Int4,
        teacher_id -> Int4,
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

diesel::table! {
    teachers (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
    }
}

diesel::joinable!(classes_students -> classes (class_id));
diesel::joinable!(classes_students -> students (student_id));
diesel::joinable!(classes_teachers -> classes (class_id));
diesel::joinable!(classes_teachers -> teachers (teacher_id));

diesel::allow_tables_to_appear_in_same_query!(
    classes,
    classes_students,
    classes_teachers,
    students,
    teachers,
);
