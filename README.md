This is the source code of my personal website [ximidas.com](https://ximidas.com/)
.

## Structure

* `client` - frontend (written in Rust using [Yew](https://yew.rs/))
* `server` - backend (written in Rust using [Actix-Web](https://actix.rs/))

## Running

* `cd client && cp .env.example .env` - creates an environment variable file for the frontend
* `cd server && cp Config.toml.example Config.toml` - creates an environment variable file for the backend

* `cd client && trunk serve` - runs the frontend.
* `cd server && cargo run`  - runs the backend. You can also use `cargo watch -x run` to automatically restart the server when changes are made on the server side, but before you'll need to install `cargo-watch`.