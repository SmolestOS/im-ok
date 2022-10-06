use crate::{models::user::User, State};
use axum::{http::StatusCode, Extension, Json};
use mongodb::bson::Bson;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct CreateResponse {
	msg: String,
	data: Option<Bson>,
}
pub async fn create_user(
	Json(payload): Json<User>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<CreateResponse>) {
	let mut resp = CreateResponse::default();
	let db_req = User::create_night(
		state.db_connection.collection::<User>("users"),
		User { id: None, username: payload.username, password: payload.password },
	)
	.await;

	match db_req {
		Ok(res) => {
			resp.msg = "Success".to_string();
			resp.data = Some(res.inserted_id);
			(StatusCode::OK, Json(resp))
		},
		Err(err) => {
			resp.msg = err.to_string();
			(StatusCode::BAD_REQUEST, Json(resp))
		},
	}
}
