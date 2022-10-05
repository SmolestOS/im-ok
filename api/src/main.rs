mod controllers;
mod db;
mod models;

use crate::controllers::nights::{
	create_night, delete_night, edit_night, get_all_nights, get_one_night,
};
use axum::{
	routing::{delete, get, patch, post},
	Router,
};
use controllers::user::create_user;
use db::establish_connection;
use mongodb::Database;
use std::net::SocketAddr;
use tower_http::add_extension::AddExtensionLayer;

#[derive(Clone)]
pub struct State {
	db_connection: Database,
}

impl State {
	fn new(db_connection: Database) -> Self {
		Self { db_connection }
	}
}

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();
	dotenv::dotenv().ok();

	let database = establish_connection().await;

	let users_routes = Router::new().route("/new", post(create_user));
	let night_routes = Router::new()
		.route("/", get(get_all_nights))
		.route("/new", post(create_night))
		.route("/:id", get(get_one_night))
		.route("/:id", patch(edit_night))
		.route("/:id", delete(delete_night));

	let app = Router::new()
		// NOTE: Nesting allow us to have endpoints with below
		// the same endpoint - @charmitro
		.nest("/users", users_routes)
		.nest("/nights", night_routes)
		.layer(AddExtensionLayer::new(State::new(database)));

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	tracing::debug!("Listening on {}", addr);
	axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
