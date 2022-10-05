use std::str::FromStr;

use crate::{
	models::{craziness::Craziness, night::Night},
	State,
};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;

use super::{CreateNightResponse, DeleteNightResponse, EditNightResponse, ResponseNight, ResponseNights};

pub async fn get_all_nights(
	Extension(state): Extension<State>,
) -> (StatusCode, Json<ResponseNights>) {
	let mut resp = ResponseNights::default();
	let cursor = Night::get_all_nights(state.db_connection.collection::<Night>("nights"))
		.await
		.unwrap();
	let mut v: Vec<Night> = cursor.try_collect().await.unwrap();

	resp.msg = "Success".to_string();
	v.sort_by(|a, b| a.craziness.date.cmp(&b.craziness.date));
	resp.data = Some(v);
	(StatusCode::CREATED, Json(resp))
}

pub async fn get_one_night(
	Path(params): Path<String>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<ResponseNight>) {
	let mut resp = ResponseNight::default();
	match ObjectId::from_str(&params) {
		Ok(oid) => {
			let db_req =
				Night::get_night(state.db_connection.collection::<Night>("nights"), oid).await;

			match db_req {
				Ok(res) =>
					if let Some(night) = res {
						resp.data = Some(night);
						resp.msg = "Success".to_string();
						(StatusCode::OK, Json(resp))
					} else {
						resp.msg = "Not found".to_string();
						(StatusCode::BAD_REQUEST, Json(resp))
					},
				Err(err) => {
					resp.msg = err.to_string();
					(StatusCode::BAD_REQUEST, Json(resp))
				},
			}
		},
		Err(err) => {
			resp.msg = err.to_string();
			(StatusCode::BAD_REQUEST, Json(resp))
		},
	}
}

pub async fn create_night(
	Json(payload): Json<Craziness>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<CreateResponse>) {
	let mut resp = CreateResponse::default();
	let db_req = Night::create_night(
		state.db_connection.collection::<Night>("nights"),
		Night { id: None, craziness: payload },
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

pub async fn delete_night(
	Path(params): Path<String>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<DeleteNightResponse>) {
	let mut resp = DeleteNightResponse::default();
	match ObjectId::from_str(&params) {
		Ok(oid) => {
			let db_req =
				Night::delete_night(state.db_connection.collection::<Night>("nights"), oid).await;

			match db_req {
				Ok(res) => {
					resp.data = Some(oid);
					resp.msg = format!("Successfully deleted {} objects", res.deleted_count);
					(StatusCode::OK, Json(resp))
				},
				Err(err) => {
					resp.msg = err.to_string();
					(StatusCode::BAD_REQUEST, Json(resp))
				},
			}
		},
		Err(err) => {
			resp.msg = err.to_string();
			(StatusCode::BAD_REQUEST, Json(resp))
		},
	}
}

pub async fn edit_night(
	Path(params): Path<String>,
	Json(payload): Json<Craziness>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<EditNightResponse>) {
	let mut resp = EditNightResponse::default();
	match ObjectId::from_str(&params) {
		Ok(oid) => {
			let db_req =
				Night::edit_night(state.db_connection.collection::<Night>("nights"), oid, payload)
					.await;

			match db_req {
				Ok(res) => {
					resp.data = res.upserted_id;
					resp.msg = "Success".to_string();
					(StatusCode::OK, Json(resp))
				},

				Err(err) => {
					resp.msg = err.to_string();
					(StatusCode::BAD_REQUEST, Json(resp))
				},
			}
		},
		Err(err) => {
			resp.msg = err.to_string();
			(StatusCode::BAD_REQUEST, Json(resp))
		},
	}
}
