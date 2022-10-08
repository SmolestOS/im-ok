use crate::schema::nights;
use chrono::{Local, NaiveDate};
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

#[derive(Serialize, Deserialize, AsChangeset)]
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

impl Night {
	pub fn create_night(conn: &mut PgConnection, item: NightJSONRequest) -> QueryResult<usize> {
		use crate::schema::nights::dsl;
		let night = NewNightDB {
			user_id: item.user_id,
			drunkness: item.drunkness,
			coitus: item.coitus,
			drive: item.drive,
			talked_2x: item.talked_2x,
			location: item.location,
			description: item.description,
			created_at: Local::now().date_naive(),
		};

		diesel::insert_into(dsl::nights).values::<NewNightDB>(night).execute(conn)
	}

	pub fn get_all_nights(conn: &mut PgConnection) -> Result<Vec<Night>, diesel::result::Error> {
		use crate::schema::nights::dsl;

		dsl::nights.load::<Night>(conn)
	}

	pub fn get_night(
		conn: &mut PgConnection,
		item_id: i32,
	) -> Result<Night, diesel::result::Error> {
		use crate::schema::nights::dsl;
		dsl::nights.filter(dsl::id.eq(item_id)).first::<Night>(conn)
	}

	pub fn delete_night(
		conn: &mut PgConnection,
		item_id: i32,
	) -> Result<usize, diesel::result::Error> {
		use crate::schema::nights::dsl;
		diesel::delete(dsl::nights.filter(dsl::id.eq(item_id))).execute(conn)
	}

	pub fn edit_night(
		conn: &mut PgConnection,
		item_id: i32,
		updated_night: NightJSONRequest,
	) -> Result<usize, diesel::result::Error> {
		use crate::schema::nights::dsl;
		diesel::update(dsl::nights)
			.filter(dsl::id.eq(item_id))
			.set::<NightJSONRequest>(updated_night)
			.execute(conn)
	}
}
