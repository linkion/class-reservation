## How to get started

### Using Docker

run:

`docker compose up -d --build`

download the diesel CLI:

follow directions here: [diesel cli](https://github.com/diesel-rs/diesel/tree/master/diesel_cli)

If running on windows you'd need to download a [postgres server](https://www.postgresql.org/download/windows/)

If on macos, you can download [libpq](https://formulae.brew.sh/formula/libpq) from homebrew

If you are getting issues even with libpq installed. try running `cargo clean`

Then, in the root of the backend directory: run 

`diesel migration run`

To see the website populated, goto the [backend website](http://localhost:8081/)
And press the Reset Database button

If, you get a 500: Internal Error, there most likely is an issue with connecting to the database,
make sure to check docker to see if `db` is running. If so, then make sure that in the backend, `.env`, DATABASE_URL should equal the url to the postgres server

View the frontend by going to [frontend website](http://localhost:8080/)

### Running Locally

You'll still need Docker to run the postgres database

can be started with `docker compose up -d --build db`

To run the backend, you'll need to download [diesel cli](https://github.com/diesel-rs/diesel/tree/master/diesel_cli), see above for more info

Navigate to `/backend`

Then run `diesel migration run`

Then start the server with `cargo run`

To then start the frontend, navigate to `/frontend`

run: `trunk serve`

To see the website populated, goto the [backend website](http://localhost:8081/)
And press the Reset Database button

View the frontend by going to [frontend website](http://localhost:8080/)
