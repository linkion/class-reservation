-- Your SQL goes here
CREATE TABLE teachers (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  middle_name VARCHAR,
  email VARCHAR
);