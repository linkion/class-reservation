use super::rocket;
use super::main;
use rocket::local::blocking::Client;
use rocket::http::Status;

#[test]
fn test_index() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let mut response = client.get("/classes").dispatch();

    assert_eq!(response.status(), Status::Ok);
}