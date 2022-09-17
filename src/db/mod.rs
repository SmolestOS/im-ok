use crate::models::Craziness;
use mongodb::{
	bson::oid::ObjectId,
	error::Error,
	options::FindOptions,
	results::{DeleteResult, InsertOneResult},
	sync::Cursor,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Night {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub craziness: Craziness,
}

impl Night {
	pub fn create_night(
		collection: &mut mongodb::sync::Collection<Night>,
		item: Night,
	) -> std::result::Result<InsertOneResult, Error> {
		// Convert `captain_marvel` to a Bson instance:
		collection.insert_one(item, None)
	}

	pub fn get_all_nights(
		collection: &mut mongodb::sync::Collection<Night>,
	) -> std::result::Result<Cursor<Night>, Error> {
		let find_options = FindOptions::builder().limit(None).build();
		collection.find(None, find_options)
	}

	#[allow(dead_code)]
	pub fn delete_night(
		collection: &mut mongodb::sync::Collection<Night>,
		item_id: ObjectId,
	) -> std::result::Result<DeleteResult, Error> {
		// Convert `captain_marvel` to a Bson instance:
		collection.delete_one(bson::doc! {"id": item_id }, None)
	}
}
