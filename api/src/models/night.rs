use crate::schema::nights;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// An enum to track the level of Drunkness (0 - 5)
#[derive(
	Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, diesel_derive_enum::DbEnum, ToSchema,
)]
pub enum Drunkness {
	Cool,
	LittleHead,
	Bream,
	Gnat,
	Ant,
	ImOk,
}

impl Default for Drunkness {
	fn default() -> Self {
		Self::Cool
	}
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
pub struct Night {
	pub id: i32,
	pub user_id: i32,
	pub drunkness: Drunkness,
	pub coitus: bool,
	pub drive: bool,
	pub talked_2x: bool,
	pub location: String,
	pub description: String,
	#[schema(value_type = Date)]
	pub created_at: NaiveDate,
}

#[derive(Serialize, Deserialize, AsChangeset, Debug, ToSchema)]
#[diesel(table_name = nights)]
pub struct NightJSONRequest {
	pub user_id: i32,
	pub drunkness: Drunkness,
	pub coitus: bool,
	pub drive: bool,
	pub talked_2x: bool,
	pub location: String,
	pub description: String,
}

#[derive(Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = nights)]
pub struct NewNightDB {
	pub user_id: i32,
	pub drunkness: Drunkness,
	pub coitus: bool,
	pub drive: bool,
	pub talked_2x: bool,
	pub location: String,
	pub description: String,
	#[schema(value_type = Date)]
	pub created_at: NaiveDate,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, Default, ToSchema)]
pub struct NightWithUser {
	pub id: i32,
	pub user_id: i32,
	pub username: String,
	pub drunkness: Drunkness,
	pub coitus: bool,
	pub drive: bool,
	pub talked_2x: bool,
	pub location: String,
	pub description: String,
	#[schema(value_type = Date)]
	pub created_at: NaiveDate,
}

pub mod responses {
	use super::*;

	#[derive(serde::Serialize, serde::Deserialize, Default, ToSchema)]
	pub struct CreateNightResponse {
		pub msg: String,
		pub data: Option<i32>,
	}

	#[derive(serde::Serialize, serde::Deserialize, Default, ToSchema)]
	pub struct ResponseNights {
		pub msg: String,
		pub data: Option<Vec<Night>>,
	}

	#[derive(serde::Serialize, serde::Deserialize, Default, ToSchema)]
	pub struct ResponseNightsWithUser {
		pub msg: String,
		pub data: Option<Vec<NightWithUser>>,
	}

	#[derive(serde::Serialize, serde::Deserialize, Default, ToSchema)]
	pub struct ResponseNight {
		pub msg: String,
		pub data: Option<Night>,
	}

	#[derive(serde::Serialize, serde::Deserialize, Default, ToSchema)]
	pub struct DeleteResponse {
		pub msg: String,
		pub data: Option<usize>,
	}

	#[derive(serde::Serialize, serde::Deserialize, Default, ToSchema)]
	pub struct EditResponse {
		pub msg: String,
		pub data: Option<usize>,
	}
}
