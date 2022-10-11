use api_types::users::{
	self,
	model::{User, UserJSONRequest},
};

use crate::State;
use axum::{http::StatusCode, Extension, Json};
use mongodb::bson::Bson;

pub async fn register_user(
	Json(payload): Json<UserJSONRequest>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<users::response::CreateResponse>) {
	let mut resp = users::response::CreateResponse::default();
	let mut code = StatusCode::OK;

	match User::create_user(
		&mut state.db_connection.get().unwrap(),
		UserJSONRequest { username: payload.username, password: payload.password },
	) {
		Ok(index) => {
			resp.msg = "Created".to_string();
			resp.data = Some(Bson::from(index.to_string()));
		},
		Err(err) => {
			if let diesel::result::Error::DatabaseError(
				diesel::result::DatabaseErrorKind::UniqueViolation,
				_,
			) = err
			{
				resp.msg = "User already exists".to_string();
				resp.data = Some(Bson::default());
				code = StatusCode::BAD_REQUEST;
			}
		},
	}

	(code, Json(resp))
}

pub async fn login_user(
	Json(payload): Json<UserJSONRequest>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<users::response::LoginResponse>) {
	let mut resp = users::response::LoginResponse::default();
	let mut code = StatusCode::OK;

	match User::get_user(&mut state.db_connection.get().unwrap(), payload) {
		Ok(user) => {
			resp.msg = "Logged in succesfully".to_string();
			resp.data = Some(user);
		},
		Err(err) =>
			if let diesel::result::Error::NotFound = err {
				resp.msg = "User not found or wrong credentials".to_string();
				resp.data = None;
				code = StatusCode::NOT_FOUND;
			} else {
				resp.msg = err.to_string();
				resp.data = None;
				code = StatusCode::NOT_FOUND;
			},
	}

	(code, Json(resp))
}
