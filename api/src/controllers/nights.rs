use std::str::FromStr;

use crate::{
	models::{craziness::Craziness, night::Night},
	State,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use futures::stream::TryStreamExt;
use mongodb::bson::{oid::ObjectId, Bson};

use super::{DeletedId, Response, ResponseNights};

pub async fn get_one_night(
	Path(params): Path<String>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<Response>) {
	let mut resp = Response::default();
	match ObjectId::from_str(&params) {
		Ok(oid) => {
			let db_req = Night::get_night(state.night_collection, oid).await;

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

pub async fn get_all_nights(
	Extension(state): Extension<State>,
) -> (StatusCode, Json<ResponseNights>) {
	let mut resp = ResponseNights::default();
	let cursor = Night::get_all_nights(state.night_collection.clone()).await.unwrap();
	let mut v: Vec<Night> = cursor.try_collect().await.unwrap();

	resp.msg = "Sucess".to_string();
	v.sort_by(|a, b| a.craziness.date.cmp(&b.craziness.date));
	resp.data = Some(v);
	(StatusCode::CREATED, Json(resp))
}

pub async fn create_night(
	Json(payload): Json<Craziness>,
	Extension(state): Extension<State>,
) -> impl IntoResponse {
	let db_req =
		Night::create_night(state.night_collection, Night { id: None, craziness: payload }).await;

	match db_req {
		Ok(res) => (StatusCode::CREATED, Json(res.inserted_id)),
		Err(err) => (StatusCode::BAD_REQUEST, Json(Bson::String(err.to_string()))),
	}
}

pub async fn delete_night(
	Path(params): Path<String>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<DeletedId>) {
	let mut resp = DeletedId::default();
	match ObjectId::from_str(&params) {
		Ok(oid) => {
			let db_req = Night::delete_night(state.night_collection, oid).await;

			match db_req {
				Ok(res) =>
					if let DeleteResult = res {
						resp.data = Some(oid);
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

pub async fn edit_night(
	Path(params): Path<String>,
	Json(payload): Json<Craziness>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<Response>) {
	let mut resp = Response::default();
	match ObjectId::from_str(&params) {
		Ok(oid) => {
			let db_req = Night::edit_night(state.night_collection, oid, payload).await;

			match db_req {
				Ok(_) => (StatusCode::CREATED, Json(Bson::String("Success".to_string()))),
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
