use crate::models::Craziness;
use mongodb::{
	bson::oid::ObjectId, error::Error, options::FindOptions, results::InsertOneResult, sync::Cursor,
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

	pub fn delete_night(item_id: ObjectId) -> std::result::Result<ureq::Response, ureq::Error> {
		ureq::delete(&format!("http://localhost:3000/night/{}", item_id)).call()
	}

	pub fn edit_night(
		item_id: ObjectId,
		craziness: Craziness,
	) -> std::result::Result<ureq::Response, ureq::Error> {
		ureq::patch(&format!("http://localhost:3000/night/{}", item_id))
			.set("Content-Type", "application/json")
			.send_json(ureq::json!(craziness))
	}
}
