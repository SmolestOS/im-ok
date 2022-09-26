use std::str::FromStr;

use crate::{
	models::{craziness::Craziness, night::Night},
	State,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use futures::stream::TryStreamExt;
use mongodb::bson::{oid::ObjectId, Bson};

// TODO(@panosfol): get_night function;

pub async fn get_all_nights(Extension(state): Extension<State>) -> impl IntoResponse {
	let cursor = Night::get_all_nights(state.night_collection.clone()).await.unwrap();
	let v: Vec<Night> = cursor.try_collect().await.unwrap();

	// TODO(@panosfol): sort by date eg. `foo_items.sort_by(|a, b| a.date.cmp(&b.date));`

	(StatusCode::CREATED, Json(v))
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
) -> impl IntoResponse {
	let db_req =
		Night::delete_night(state.night_collection, ObjectId::from_str(&params).unwrap()).await;

	match db_req {
		Ok(_) => (StatusCode::CREATED, Json(Bson::String("Success".to_string()))),
		Err(err) => (StatusCode::BAD_REQUEST, Json(Bson::String(err.to_string()))),
	}
}

pub async fn edit_night(
	Path(params): Path<String>,
	Json(payload): Json<Craziness>,
	Extension(state): Extension<State>,
) -> impl IntoResponse {
	let db_req =
		Night::edit_night(state.night_collection, ObjectId::from_str(&params).unwrap(), payload)
			.await;

	match db_req {
		Ok(_) => (StatusCode::CREATED, Json(Bson::String("Success".to_string()))),
		Err(err) => (StatusCode::BAD_REQUEST, Json(Bson::String(err.to_string()))),
	}
}
