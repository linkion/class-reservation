-- Your SQL goes here
CREATE TABLE classes_teachers (
  class_id INTEGER REFERENCES classes(id),
  teacher_id INTEGER REFERENCES teachers(id),
  PRIMARY KEY (class_id, teacher_id)
);