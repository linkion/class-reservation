-- Your SQL goes here
CREATE TABLE students (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  middle_name VARCHAR,
  uin integer NOT NULL,
  net_id VARCHAR
);