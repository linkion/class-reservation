-- Your SQL goes here
CREATE TABLE dormitories_rooms (
  dorm_id INTEGER REFERENCES dormitories(id),
  room_id INTEGER REFERENCES rooms(id),
  PRIMARY KEY (dorm_id, room_id)
);