use crate::{
	models::night::{Night, NightJSONRequest},
	State,
};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use mongodb::bson::Bson;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct CreateResponse {
	msg: String,
	data: Option<Bson>,
}

pub async fn create_night(
	Json(payload): Json<NightJSONRequest>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<CreateResponse>) {
	let mut resp = CreateResponse::default();
	let mut code = StatusCode::OK;

	tracing::info!("{:?}", payload);
	match Night::create_night(&mut state.db_connection.get().unwrap(), payload) {
		Ok(index) => {
			resp.msg = "Created".to_string();
			resp.data = Some(Bson::from(index.to_string()));
		},
		Err(err) => {
			resp.msg = err.to_string();
			tracing::info!("{:?}", resp.msg);
			resp.data = Some(Bson::default());
			code = StatusCode::BAD_REQUEST;
		},
	}

	(code, Json(resp))
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct ResponseNights {
	msg: String,
	data: Option<Vec<Night>>,
}

pub async fn get_all_nights(
	Extension(state): Extension<State>,
) -> (StatusCode, Json<ResponseNights>) {
	let mut resp = ResponseNights::default();
	let mut code = StatusCode::OK;

	match Night::get_all_nights(&mut state.db_connection.get().unwrap()) {
		Ok(mut index) => {
			resp.msg = "Created".to_string();
			index.sort_by(|a, b| a.created_at.cmp(&b.created_at));
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
			} else {
				resp.msg = err.to_string();
				resp.data = None;
				code = StatusCode::BAD_REQUEST;
			}
		},
	};

	// resp.msg = "Success".to_string();
	// v.sort_by(|a, b| a.craziness.date.cmp(&b.craziness.date));
	// resp.data = Some(v);
	(code, Json(resp))
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct ResponseNight {
	msg: String,
	data: Option<Night>,
}

pub async fn get_one_night(
	Path(item_id): Path<i32>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<ResponseNight>) {
	let mut resp = ResponseNight::default();
	let mut code = StatusCode::OK;

	match Night::get_night(&mut state.db_connection.get().unwrap(), item_id) {
		Ok(night) => {
			resp.msg = "Found".to_string();
			resp.data = Some(night);
		},
		Err(err) =>
			if let diesel::result::Error::NotFound = err {
				resp.msg = format!("Night with id: {} not found ", item_id);
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

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DeleteResponse {
	msg: String,
	data: Option<usize>,
}

pub async fn delete_night(
	Path(item_id): Path<i32>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<DeleteResponse>) {
	let mut resp = DeleteResponse::default();
	let mut code = StatusCode::OK;

	match Night::delete_night(&mut state.db_connection.get().unwrap(), item_id) {
		Ok(count) =>
			if count.eq(&1) {
				resp.msg = format!("Deleted Night with id: {}", item_id);
				resp.data = Some(count);
			} else {
				resp.msg = format!("Night with id: {} not found ", item_id);
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

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct EditResponse {
	msg: String,
	data: Option<usize>,
}

pub async fn edit_night(
	Path(item_id): Path<i32>,
	Json(payload): Json<NightJSONRequest>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<EditResponse>) {
	let mut resp = EditResponse::default();
	let mut code = StatusCode::OK;

	match Night::edit_night(&mut state.db_connection.get().unwrap(), item_id, payload) {
		Ok(count) =>
			if count.eq(&1) {
				resp.msg = format!("Updated Night with id: {}", item_id);
				resp.data = Some(count);
			} else {
				resp.msg = format!("Night with id: {} not found ", item_id);
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
