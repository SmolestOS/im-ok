use crate::{
	db,
	models::night::{responses::*, NightJSONRequest},
	State,
};
use axum::{extract::Path, http::StatusCode, Extension, Json};

#[utoipa::path(
    post,
    path = "/nights/new",
    request_body = NightJSONRequest,
    responses(
	(status = 200, description = "Creates a new night entry for the current user", body = [CreateResponse])
    )
)]
pub async fn create_night(
	Json(payload): Json<NightJSONRequest>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<CreateResponse>) {
	let mut resp = CreateResponse::default();
	let mut code = StatusCode::OK;

	match db::nights::create_night(&mut state.db_connection.get().unwrap(), payload) {
		Ok(index) => {
			resp.msg = "Created".to_string();
			resp.data = Some(index);
		},
		Err(err) => {
			resp.msg = err.to_string();
			tracing::info!("{:?}", resp.msg);
			resp.data = None;
			code = StatusCode::BAD_REQUEST;
		},
	}

	(code, Json(resp))
}
#[utoipa::path(
    get,
    path = "/nights/",
    responses(
	(status = 200, description = "Get all night entries", body = [ResponseNights])
    )
)]
pub async fn get_all_nights(
	Extension(state): Extension<State>,
) -> (StatusCode, Json<ResponseNights>) {
	let mut resp = ResponseNights::default();
	let mut code = StatusCode::OK;

	match db::nights::get_all_nights(&mut state.db_connection.get().unwrap()) {
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

	(code, Json(resp))
}
#[utoipa::path(
    get,
    path = "/nights/with_users",
    responses(
	(status = 200, description = "Get all night entries but with the user attached to them", body = [ResponseNightsWithUser])
    )
)]
pub async fn get_all_nights_with_user(
	Extension(state): Extension<State>,
) -> (StatusCode, Json<ResponseNightsWithUser>) {
	let mut resp = ResponseNightsWithUser::default();
	let mut code = StatusCode::OK;

	match db::nights::get_all_nights_with_user(&mut state.db_connection.get().unwrap()) {
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
	(code, Json(resp))
}
#[utoipa::path(
    get,
    path = "/nights/{id}",
    responses(
	(status = 200, description = "Getting one night using the corresponding id", body = [ResponseNight])
    ),
    params(
        ("id" = i32,),
        )
)]

pub async fn get_one_night(
	Path(item_id): Path<i32>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<ResponseNight>) {
	let mut resp = ResponseNight::default();
	let mut code = StatusCode::OK;

	match db::nights::get_night(&mut state.db_connection.get().unwrap(), item_id) {
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

#[utoipa::path(
    delete,
    path = "/nights/{id}",
    responses(
	(status = 200, description = "Delete a specific night using the corresponding id", body = [DeleteResponse])
    ),
    params(
        ("id" = i32,),
    )
)]

pub async fn delete_night(
	Path(item_id): Path<i32>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<DeleteResponse>) {
	let mut resp = DeleteResponse::default();
	let mut code = StatusCode::OK;

	match db::nights::delete_night(&mut state.db_connection.get().unwrap(), item_id) {
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
#[utoipa::path(
    patch,
    path = "/nights/{id}",
    request_body = NightJSONRequest,
    responses(
	(status = 200, description = "Edit a night using the corresponding id", body = [EditResponse])
    ),
    params(
        ("id" = i32,),
    )
)]

pub async fn edit_night(
	Path(item_id): Path<i32>,
	Json(payload): Json<NightJSONRequest>,
	Extension(state): Extension<State>,
) -> (StatusCode, Json<EditResponse>) {
	let mut resp = EditResponse::default();
	let mut code = StatusCode::OK;

	match db::nights::edit_night(&mut state.db_connection.get().unwrap(), item_id, payload) {
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
