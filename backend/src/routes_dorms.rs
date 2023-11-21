use backend::*;
use rocket::{State, route, Route};
use rocket::form::Form;
use rocket::serde::{json::Json, Serialize, Deserialize};
use diesel::{prelude::*, result};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;

pub fn routes() -> Vec<Route> {
  routes![post_room, get_room, delete_room]
}