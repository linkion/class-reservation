-- Your SQL goes here
CREATE TABLE classes_students (
  class_id INTEGER REFERENCES classes(id),
  student_id INTEGER REFERENCES students(id),
  PRIMARY KEY (class_id, student_id)
);