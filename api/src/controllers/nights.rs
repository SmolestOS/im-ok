use crate::{
	models::night::{Night, NightJSONRequest},
	State,
};
use axum::{http::StatusCode, Extension, Json};
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

	match Night::create_night(&mut state.db_connection.get().unwrap(), payload) {
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
			} else {
				resp.msg = err.to_string();
				resp.data = Some(Bson::default());
				code = StatusCode::BAD_REQUEST;
			}
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

// #[derive(serde::Serialize, serde::Deserialize, Default)]
// pub struct ResponseNight {
// 	msg: String,
// 	data: Option<Night>,
// }

// pub async fn get_one_night(
// 	Path(params): Path<String>,
// 	Extension(state): Extension<State>,
// ) -> (StatusCode, Json<ResponseNight>) {
// 	let mut resp = ResponseNight::default();
// 	match ObjectId::from_str(&params) {
// 		Ok(oid) => {
// 			let db_req =
// 				Night::get_night(state.db_connection.collection::<Night>("nights"), oid).await;

// 			match db_req {
// 				Ok(res) => {
// 					if let Some(night) = res {
// 						resp.data = Some(night);
// 						resp.msg = "Success".to_string();
// 						(StatusCode::OK, Json(resp))
// 					} else {
// 						resp.msg = "Not found".to_string();
// 						(StatusCode::BAD_REQUEST, Json(resp))
// 					}
// 				},
// 				Err(err) => {
// 					resp.msg = err.to_string();
// 					(StatusCode::BAD_REQUEST, Json(resp))
// 				},
// 			}
// 		},
// 		Err(err) => {
// 			resp.msg = err.to_string();
// 			(StatusCode::BAD_REQUEST, Json(resp))
// 		},
// 	}
// }

// #[derive(serde::Serialize, serde::Deserialize, Default)]
// pub struct DeleteResponse {
// 	msg: String,
// 	data: Option<ObjectId>,
// }

// pub async fn delete_night(
// 	Path(params): Path<String>,
// 	Extension(state): Extension<State>,
// ) -> (StatusCode, Json<DeleteResponse>) {
// 	let mut resp = DeleteResponse::default();
// 	match ObjectId::from_str(&params) {
// 		Ok(oid) => {
// 			let db_req =
// 				Night::delete_night(state.db_connection.collection::<Night>("nights"), oid).await;

// 			match db_req {
// 				Ok(res) => {
// 					resp.data = Some(oid);
// 					resp.msg = format!("Successfully deleted {} objects", res.deleted_count);
// 					(StatusCode::OK, Json(resp))
// 				},
// 				Err(err) => {
// 					resp.msg = err.to_string();
// 					(StatusCode::BAD_REQUEST, Json(resp))
// 				},
// 			}
// 		},
// 		Err(err) => {
// 			resp.msg = err.to_string();
// 			(StatusCode::BAD_REQUEST, Json(resp))
// 		},
// 	}
// }

// #[derive(serde::Serialize, serde::Deserialize, Default)]
// pub struct EditResponse {
// 	msg: String,
// 	data: Option<Bson>,
// }

// pub async fn edit_night(
// 	Path(params): Path<String>,
// 	Json(payload): Json<Craziness>,
// 	Extension(state): Extension<State>,
// ) -> (StatusCode, Json<EditResponse>) {
// 	let mut resp = EditResponse::default();
// 	match ObjectId::from_str(&params) {
// 		Ok(oid) => {
// 			let db_req =
// 				Night::edit_night(state.db_connection.collection::<Night>("nights"), oid, payload)
// 					.await;

// 			match db_req {
// 				Ok(res) => {
// 					resp.data = res.upserted_id;
// 					resp.msg = "Success".to_string();
// 					(StatusCode::OK, Json(resp))
// 				},

// 				Err(err) => {
// 					resp.msg = err.to_string();
// 					(StatusCode::BAD_REQUEST, Json(resp))
// 				},
// 			}
// 		},
// 		Err(err) => {
// 			resp.msg = err.to_string();
// 			(StatusCode::BAD_REQUEST, Json(resp))
// 		},
// 	}
// }
