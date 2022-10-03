use std::str::FromStr;

use crate::{
    models:: user::User,State,
};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;

use super::{CreateUserResponse, DeleteUserResponse, EditUserResponse, ResponseNight };

pub async fn create_user(
    Json(payload): Json<User>,
    Extension(state): Extension<State>,
) -> (StatusCode, Json<CreateUserResponse>) {
    let mut resp = CreateUserResponse::default();
    let db_req = User::create_user(state.user_collection, User { id: None, username: payload.username, password: payload.password}).await;

    match db_req {
        Ok(res) => {
            resp.msg = "Success".to_string();
            resp.data = Some(res.inserted_id);
            (StatusCode::OK, Json(resp))
        },
        Err(err) => {
            resp.msg = err.to_string();
            (StatusCode::BAD_REQUEST, Json(resp))
        }
    }
}

pub async fn get_one_user(
    Path(params): Path<String>,
    Extension(state): Extension<State>,
) -> (StatusCode, Json<ResponseUser>) {
    let mut resp = ResponseUser::default();
    match ObjectId::from_str(&params) {
	Ok(oid) => {
	    let db_req = User::get_user(state.user_collection, oid).await;

	    match db_req {
		Ok(res) =>
		    if let Some(user) = res {
			resp.data = Some(user);
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

pub async fn delete_user(
    Path(params): Path<String>,
    Extension(state): Extension<State>,
) -> (StatusCode, Json<DeleteUserResponse>) {
    let mut resp = DeleteUserResponse::default();
    match ObjectId::from_str(&params) {
	Ok(oid) => {
	    let db_req = Night::delete_user(state.user_collection, oid).await;

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

pub async fn edit_user(
    Path(params): Path<String>,
    Json(payload): Json<User>,
    Extension(state): Extension<State>,
) -> (StatusCode, Json<EditUserResponse>) {
    let mut resp = EditUserResponse::default();
    match ObjectId::from_str(&params) {
	Ok(oid) => {
	    let db_req = Night::edit_user(state.user_collection, oid, payload).await;

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
