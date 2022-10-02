use crate::{models::user::User, State};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUser {
	username: String,
	password: String,
}
pub async fn create_user(
	Json(payload): Json<CreateUser>,
	Extension(_state): Extension<State>,
) -> impl IntoResponse {
	let user = User { id: None, username: payload.username, password: payload.password };

	(StatusCode::CREATED, Json(user))
}
