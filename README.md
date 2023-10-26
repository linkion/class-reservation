# Rust Class Reservation Interface/Server
CS128H Final Project

Group: Milo & Patrick
Members:
* Patrick Orave
* Milo Brodziak

## Project Introduction
Seeing the slowness of registering for classes, we figured, hey, what if we were to write it in Rust? And that's this project. We're not expecting this to be used in any capacity by the school, this'll be a proof of concept of using WASM and the power of Rust to make a performant class registration system.

## Technical Information
Frontend
written with the rust Yew webframework using WASM

Backend
Rust Rocket server with REST
using websockets to update all clients of changes to registrations
using sqlx to connect server with SQL database

## Possible Challenges
* learning the web framework Yew
* Working in a group and contributing to the same project
* Using Git for version control
* Setting up a development server for testing the frontend with the backend with example data

## References
* the full rust stack is based off of this [template](https://github.com/dontlaugh/rocket-yew-starter-pack)

## Building and running
Install `trunk`

```cargo install trunk```

Add wasm build target

```rustup target add wasm32-unknown-unknown```

Start Development client server

```trunk serve --open```
