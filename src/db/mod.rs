use mongodb::{
	bson::oid::ObjectId,
	error::Error,
	results::{DeleteResult, InsertOneResult},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Night {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub location: String,
	pub date: bson::DateTime,
}

impl Night {
	#[allow(dead_code)]
	pub fn create_night(
		collection: &mut mongodb::sync::Collection<Night>,
		item: Night,
	) -> std::result::Result<InsertOneResult, Error> {
		// Convert `captain_marvel` to a Bson instance:
		collection.insert_one(item, None)
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
