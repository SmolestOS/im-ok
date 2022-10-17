use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Serialize, Deserialize, Eq, PartialEq, Debug, Default)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub password: String,
	pub created_on: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
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

    #[derive(serde::Serialize, serde::Deserialize, Default)]
    pub struct CreateResponse {
	pub msg: String,
	pub data: Option<Bson>,
    }

    #[derive(serde::Serialize, serde::Deserialize, Default)]
    pub struct LoginData {
        pub token: String,
        pub user: User,
    }

    #[derive(serde::Serialize, serde::Deserialize, Default)]
    pub struct LoginResponse {
	pub msg: String,
	pub data: Option<LoginData>,
    }
}
