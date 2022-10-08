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

#[derive(Serialize, Deserialize)]
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
		use crate::schema::nights::dsl::*;
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

		diesel::insert_into(nights).values(&night).execute(conn)
	}

	pub fn get_all_nights(conn: &mut PgConnection) -> Result<Vec<Night>, diesel::result::Error> {
		use crate::schema::nights::dsl;

		dsl::nights.load::<Night>(conn)
	}

	// 	pub async fn get_night(
	// 		collection: mongodb::Collection<Night>,
	// 		item_id: ObjectId,
	// 	) -> std::result::Result<Option<Night>, Error> {
	// 		let find_option = FindOneOptions::builder().build();
	// 		collection.find_one(bson::doc! {"_id": item_id}, find_option).await
	// 	}

	// 	pub async fn delete_night(
	// 		collection: mongodb::Collection<Night>,
	// 		item_id: ObjectId,
	// 	) -> std::result::Result<DeleteResult, Error> {
	// 		collection.delete_one(bson::doc! {"_id": item_id }, None).await
	// 	}

	// 	pub async fn edit_night(
	// 		collection: mongodb::Collection<Night>,
	// 		item_id: ObjectId,
	// 		craziness: Craziness,
	// 	) -> std::result::Result<UpdateResult, Error> {
	// 		let query = bson::doc! { "_id": item_id };
	// 		let doc = bson::to_document(&craziness).unwrap();
	// 		let update = bson::doc! {"$set": { "craziness": doc }};
	// 		collection.update_one(query, update, None).await
	// 	}
}
