-- Your SQL goes here
CREATE TABLE rooms_students (
  room_id INTEGER REFERENCES rooms(id),
  student_id INTEGER REFERENCES students(id),
  PRIMARY KEY (room_id, student_id)
);