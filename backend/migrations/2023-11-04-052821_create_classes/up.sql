-- Your SQL goes here
CREATE TABLE classes (
  id SERIAL PRIMARY KEY,
  class_name VARCHAR NOT NULL,
  max_students INTEGER NOT NULL,
  registered_students INTEGER NOT NULL DEFAULT 0,
  published BOOLEAN NOT NULL DEFAULT TRUE
);