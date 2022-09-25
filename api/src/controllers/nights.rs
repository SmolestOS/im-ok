use std::str::FromStr;

use crate::{
	models::{craziness::Craziness, night::Night},
	State,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use futures::stream::TryStreamExt;
use mongodb::bson::{oid::ObjectId, Bson};

pub async fn get_all_nights(Extension(state): Extension<State>) -> impl IntoResponse {
	let cursor = Night::get_all_nights(state.night_collection.clone()).await.unwrap();

	let v: Vec<Night> = cursor.try_collect().await.unwrap();

	(StatusCode::CREATED, Json(v))
}

pub async fn create_night(
	Json(payload): Json<Craziness>,
	Extension(state): Extension<State>,
) -> impl IntoResponse {
	let res =
		Night::create_night(state.night_collection, Night { id: None, craziness: payload }).await;

	match res {
		Ok(id) => (StatusCode::CREATED, Json(id.inserted_id)),
		Err(err) => (StatusCode::BAD_REQUEST, Json(Bson::String(err.to_string()))),
	}
}

pub async fn delete_night(
	Path(params): Path<String>,
	Extension(state): Extension<State>,
) -> impl IntoResponse {
	let req =
		Night::delete_night(state.night_collection, ObjectId::from_str(&params).unwrap()).await;

	match req {
		Ok(_res) => (StatusCode::CREATED, Json(Bson::String("Success".to_string()))),
		Err(err) => (StatusCode::BAD_REQUEST, Json(Bson::String(err.to_string()))),
	}
}
