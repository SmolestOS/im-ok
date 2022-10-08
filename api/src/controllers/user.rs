use crate::{
	models::user::{User, UserJSONRequest},
	State,
};
use axum::{http::StatusCode, Extension, Json};
use mongodb::bson::Bson;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct CreateResponse {
	msg: String,
	data: Option<Bson>,
}

pub async fn register_user(
	Json(payload): Json<UserJSONRequest>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<CreateResponse>) {
	let mut resp = CreateResponse::default();
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

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct LoginResponse {
	msg: String,
	data: Option<User>,
}

pub async fn login_user(
	Json(payload): Json<UserJSONRequest>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<LoginResponse>) {
	let mut resp = LoginResponse::default();
	let mut code = StatusCode::OK;

	match User::get_user(
		&mut state.db_connection.get().unwrap(),
		payload,
	) {
		Ok(index) => {
			resp.msg = "Logged in succesfully".to_string();
			resp.data = Some(index);
			code = StatusCode::OK;
		},
		Err(err) =>
			if let diesel::result::Error::NotFound = err {
				resp.msg = "User not or wrong credentials".to_string();
				resp.data = None;
				code = StatusCode::BAD_REQUEST;
			},
	}

	(code, Json(resp))
}
