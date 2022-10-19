mod controllers;
mod db;
mod models;
mod schema;

use crate::controllers::{
	nights::{
		create_night, delete_night, edit_night, get_all_nights, get_all_nights_with_user,
		get_one_night,
	},
	user::{__path_register_user, login_user},
};
use axum::{
	routing::{delete, get, patch, post},
	Router,
};
use controllers::user::register_user;
use db::establish_connection;
use diesel::{
	r2d2::{ConnectionManager, Pool},
	PgConnection,
};
use models::user::User;
use std::net::SocketAddr;
use tower_http::{add_extension::AddExtensionLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct State {
	db_connection: Pool<ConnectionManager<PgConnection>>,
}

impl State {
	fn new(db_connection: Pool<ConnectionManager<PgConnection>>) -> Self {
		Self { db_connection }
	}
}

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();
	dotenvy::dotenv().ok();

	#[derive(OpenApi)]
	#[openapi(
		paths(
			register_user,
		),
		components(
			schemas(User,
					api::models::user::responses::CreateResponse,
api::models::user::UserJSONRequest,
			)
),
		tags(
			(name = "todo", description = "Todo items")
		)
	)]
	struct ApiDoc;

	let database = establish_connection().await;

	let users_routes = Router::new()
		.route("/register", post(register_user))
		.route("/login", post(login_user));

	let night_routes = Router::new()
		.route("/", get(get_all_nights))
		.route("/with_users", get(get_all_nights_with_user))
		.route("/new", post(create_night))
		.route("/:id", get(get_one_night))
		.route("/:id", delete(delete_night))
		.route("/:id", patch(edit_night));

	let app = Router::new()
		// NOTE: Nesting allow us to have endpoints with below
		// the same endpoint - @charmitro
		.merge(SwaggerUi::new("/swagger-ui/*tail").url("/api-doc/openapi.json", ApiDoc::openapi()))
		.nest("/users", users_routes)
		.nest("/nights", night_routes)
		.layer(TraceLayer::new_for_http())
		.layer(AddExtensionLayer::new(State::new(database)));

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	tracing::debug!("Listening on {}", addr);
	axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
