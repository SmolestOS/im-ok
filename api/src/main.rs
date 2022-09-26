mod controllers;
mod db;
mod models;

use crate::controllers::nights::{create_night, delete_night, edit_night, get_all_nights};
use axum::{
	routing::{delete, get, patch, post},
	Router,
};
use controllers::user::create_user;
use db::establish_connection;
use models::night::Night;
use mongodb::Collection;
use std::net::SocketAddr;
use tower_http::add_extension::AddExtensionLayer;

#[derive(Clone)]
pub struct State {
	night_collection: Collection<Night>,
}

impl State {
	fn new(night_collection: Collection<Night>) -> Self {
		Self { night_collection }
	}
}

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();
	dotenv::dotenv().ok();

	let collection = establish_connection().await;

	let app = Router::new()
		.route("/users", post(create_user))
		.route("/nights", get(get_all_nights))
		// TODO(@panosfol): get_one_night;
		// .route("/night/:id", get(get_night))
		.route("/night", post(create_night))
		.route("/night/:id", delete(delete_night))
		.route("/night/:id", patch(edit_night))
		.layer(AddExtensionLayer::new(State::new(collection)));

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	tracing::debug!("Listening on {}", addr);
	axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
