use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Clone, Serialize, Deserialize, Eq, PartialEq, Debug, Default, ToSchema)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub password: String,
	pub created_on: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserJSONRequest {
	pub username: String,
	pub password: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUserDB {
	pub username: String,
	pub password: String,
	pub created_on: chrono::NaiveDateTime,
}

pub mod responses {
	use super::*;
	use mongodb::bson::Bson;

	#[derive(serde::Serialize, serde::Deserialize, Default, ToSchema)]
	pub struct CreateResponse {
		pub msg: String,
		pub data: Option<Bson>,
	}

	#[derive(serde::Serialize, serde::Deserialize, Default)]
	pub struct LoginResponse {
		pub msg: String,
		pub data: Option<User>,
	}
}
