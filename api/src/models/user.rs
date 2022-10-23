use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Clone, Serialize, Deserialize, Eq, PartialEq, Debug, Default, ToSchema)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub password: String,
	#[schema(value_type = Date)]
	pub created_on: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserJSONRequest {
	pub username: String,
	pub password: String,
}

#[derive(Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = users)]
pub struct NewUserDB {
	pub username: String,
	pub password: String,
	#[schema(value_type = Date)]
	pub created_on: chrono::NaiveDateTime,
}

pub mod responses {
	use super::*;

	#[derive(serde::Serialize, serde::Deserialize, Default, ToSchema)]
	pub struct CreateUserResponse {
		pub msg: String,
		pub data: Option<i32>,
	}

	#[derive(serde::Serialize, serde::Deserialize, Default, ToSchema)]
	pub struct LoginData {
		pub token: String,
		pub user: User,
	}

	#[derive(serde::Serialize, serde::Deserialize, Default, ToSchema)]
	pub struct LoginResponse {
		pub msg: String,
		pub data: Option<LoginData>,
	}
}
