-- Your SQL goes here
CREATE TABLE rooms (
  id SERIAL PRIMARY KEY,
  room_number INT NOT NULL,
  max_occupants INT NOT NULL,
  occupants INT NOT NULL DEFAULT 0
);