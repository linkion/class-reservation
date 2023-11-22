# Rust Class Reservation Interface/Server
CS128H Final Project

Group: Milo & Patrick

Members:
* Patrick Orave (porav3)
* Milosz Brodziak (mbrod6)

## Project Introduction
Seeing the slowness of registering for classes, we figured, hey, what if we were to write it in Rust? And that's this project. We're not expecting this to be used in any capacity by the school, this'll be a proof of concept of using WASM and the power of Rust to make a performant class registration system.

**Due to time constraints**, we're switching from a class reservation to a dorm room reservation. Since a class reservation has a lot more features that are to be expected, and without those features the project would feel incomplete.

## Technical Information
**Frontend**

written with the rust Yew webframework using WASM

**Backend**

Rust Rocket server with REST
using server-sent events to update all clients of changes to registrations

using [diesel](https://diesel.rs) to connect server with postgres SQL database

## Possible Challenges
* learning the web framework Yew
* Working in a group and contributing to the same project
* Using Git for version control
* Setting up a development server for testing the frontend with the backend with example data

## References
* the full rust stack is based off of this [rocket-yew-starter-pack](https://github.com/dontlaugh/rocket-yew-starter-pack) and [rust-web-starter](https://github.com/ghotiphud/rust-web-starter)

## Building and running
Using `Docker`

Run 

```docker compose up -d --build```

connect to development site frontend

```http://localhost:8080```