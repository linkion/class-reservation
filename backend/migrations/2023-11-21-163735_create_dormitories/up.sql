-- Your SQL goes here
CREATE TABLE dormitories (
  id SERIAL PRIMARY KEY,
  dorm_name VARCHAR NOT NULL,
  dorm_group VARCHAR NOT NULL,
  dorm_pic VARCHAR NOT NULL,
  rooms INT NOT NULL DEFAULT 0,
  rooms_available INT NOT NULL DEFAULT 0
);