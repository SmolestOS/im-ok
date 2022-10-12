use crate::schema::nights;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// An enum to track the level of Drunkness (0 - 5)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
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

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, Default)]
pub struct Night {
	pub id: i32,
	pub user_id: i32,
	pub drunkness: Drunkness,
	pub coitus: bool,
	pub drive: bool,
	pub talked_2x: bool,
	pub location: String,
	pub description: String,
	pub created_at: NaiveDate,
}

#[derive(Serialize, Deserialize, AsChangeset, Debug)]
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

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = nights)]
pub struct NewNightDB {
	pub user_id: i32,
	pub drunkness: Drunkness,
	pub coitus: bool,
	pub drive: bool,
	pub talked_2x: bool,
	pub location: String,
	pub description: String,
	pub created_at: NaiveDate,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, Default)]
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
	pub created_at: NaiveDate,
}
