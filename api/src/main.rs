mod controllers;
mod db;
mod models;
mod schema;

use crate::controllers::{
	auth_middleware::auth_middleware,
	nights::{
		__path_create_night, __path_delete_night, __path_edit_night, __path_get_all_nights,
		__path_get_all_nights_with_user, __path_get_one_night, create_night, delete_night,
		edit_night, get_all_nights, get_all_nights_with_user, get_one_night,
	},
	user::{__path_login_user, __path_register_user, login_user, register_user},
};
use axum::{
	middleware,
	routing::{delete, get, patch, post},
	Router,
};
use db::establish_connection;
use diesel::{
	r2d2::{ConnectionManager, Pool},
	PgConnection,
};
use models::{night::Night, user::User};
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
            login_user,
            create_night,
            get_all_nights,
            get_all_nights_with_user,
            get_one_night,
            delete_night,
            edit_night
		),
		components(
			schemas(
				User,
				api::models::user::responses::LoginResponse,
                api::models::user::responses::CreateResponse,
                api::models::user::UserJSONRequest,
                Night,
                api::models::night::responses::CreateResponse,
                api::models::night::responses::ResponseNights,
                api::models::night::responses::ResponseNightsWithUser,
                api::models::night::responses::ResponseNight,
                api::models::night::responses::DeleteResponse,
                api::models::night::responses::EditResponse,
                api::models::night::NightJSONRequest,
                api::models::night::NightWithUser,
                api::models::night::Drunkness,
	    	)
        ),
		tags(
			(name = "imok", description = "")
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
		.route("/:id", patch(edit_night))
		.layer(middleware::from_fn(auth_middleware));

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
