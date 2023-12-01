// @generated automatically by Diesel CLI.

diesel::table! {
    dormitories (id) {
        id -> Int4,
        dorm_name -> Varchar,
        dorm_group -> Varchar,
        dorm_pic -> Varchar,
        rooms -> Int4,
        rooms_available -> Int4,
    }
}

diesel::table! {
    dormitories_rooms (dorm_id, room_id) {
        dorm_id -> Int4,
        room_id -> Int4,
    }
}

diesel::table! {
    rooms (id) {
        id -> Int4,
        room_number -> Int4,
        max_occupants -> Int4,
        occupants -> Int4,
    }
}

diesel::table! {
    rooms_students (room_id, student_id) {
        room_id -> Int4,
        student_id -> Int4,
    }
}

diesel::table! {
    rooms_students_holds (room_id, student_id) {
        room_id -> Int4,
        student_id -> Int4,
    }
}

diesel::table! {
    rooms_students_reservations (room_id, student_id) {
        room_id -> Int4,
        student_id -> Int4,
    }
}

diesel::table! {
    students (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        middle_name -> Nullable<Varchar>,
    }
}

diesel::joinable!(dormitories_rooms -> dormitories (dorm_id));
diesel::joinable!(dormitories_rooms -> rooms (room_id));
diesel::joinable!(rooms_students -> rooms (room_id));
diesel::joinable!(rooms_students -> students (student_id));
diesel::joinable!(rooms_students_holds -> rooms (room_id));
diesel::joinable!(rooms_students_holds -> students (student_id));
diesel::joinable!(rooms_students_reservations -> rooms (room_id));
diesel::joinable!(rooms_students_reservations -> students (student_id));

diesel::allow_tables_to_appear_in_same_query!(
    dormitories,
    dormitories_rooms,
    rooms,
    rooms_students,
    rooms_students_holds,
    rooms_students_reservations,
    students,
);
