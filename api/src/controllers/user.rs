use super::auth_middleware::token_gen;
use crate::{
	db,
	models::user::{responses::*, UserJSONRequest},
	State,
};
use axum::{extract::Path, http::StatusCode, Extension, Json};

#[utoipa::path(
	post,
	path = "/users/register",
	request_body = UserJSONRequest,
	responses(
		(status = 200, description = "Creates/Registers a new user", body = [CreateUserResponse])
	)
)]
pub async fn register_user(
	Json(payload): Json<UserJSONRequest>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<CreateUserResponse>) {
	let mut resp = CreateUserResponse::default();
	let mut code = StatusCode::OK;

	match db::users::create_user(
		&mut state.db_connection.get().unwrap(),
		UserJSONRequest { username: payload.username, password: payload.password },
	) {
		Ok(index) => {
			resp.msg = "Created".to_string();
			resp.data = Some(index);
		},
		Err(err) => {
			if let diesel::result::Error::DatabaseError(
				diesel::result::DatabaseErrorKind::UniqueViolation,
				_,
			) = err
			{
				resp.msg = "User already exists".to_string();
				resp.data = None;
				code = StatusCode::BAD_REQUEST;
			}
		},
	}

	(code, Json(resp))
}

#[utoipa::path(
    post,
    path = "/users/login",
    request_body = UserJSONRequest,
    responses(
	(status = 200, description = "Login with a user", body = [LoginResponse])
    )
)]

pub async fn login_user(
	Json(payload): Json<UserJSONRequest>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<LoginResponse>) {
	let mut resp = LoginResponse::default();
	let mut code = StatusCode::OK;
	let mut login_data = LoginData::default();

	match db::users::get_user(&mut state.db_connection.get().unwrap(), payload) {
		Ok(user) => {
			resp.msg = "Success".to_string();
			login_data.user = user;
			login_data.token = token_gen().await.unwrap();
			resp.data = Some(login_data);
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

pub async fn delete_user(
	Path(item_id): Path<i32>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<DeleteUserResponse>) {
	let mut resp = DeleteUserResponse::default();
	let mut code = StatusCode::OK;

	match db::users::delete_user(&mut state.db_connection.get().unwrap(), item_id) {
		Ok(count) =>
			if count.eq(&1) {
				resp.msg = format!("Deleted User with id: {}", item_id);
				resp.data = Some(count);
			} else {
				resp.msg = format!("User with id: {} not found ", item_id);
				resp.data = None;
				code = StatusCode::NOT_FOUND;
			},
		Err(err) => {
			resp.msg = err.to_string();
			resp.data = None;
			code = StatusCode::NOT_FOUND;
		},
	}

	(code, Json(resp))
}
