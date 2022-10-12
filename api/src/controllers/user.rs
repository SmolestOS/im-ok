use crate::{
	db,
	models::user::{responses::*, UserJSONRequest},
	State,
};
use axum::{http::StatusCode, Extension, Json};
use mongodb::bson::Bson;

pub async fn register_user(
	Json(payload): Json<UserJSONRequest>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<CreateResponse>) {
	let mut resp = CreateResponse::default();
	let mut code = StatusCode::OK;

	match db::users::create_user(
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
) -> (StatusCode, Json<LoginResponse>) {
	let mut resp = LoginResponse::default();
	let mut code = StatusCode::OK;

	match db::users::get_user(&mut state.db_connection.get().unwrap(), payload) {
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
