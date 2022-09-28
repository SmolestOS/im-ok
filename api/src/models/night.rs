use mongodb::{
	bson::{self, oid::ObjectId},
	error::Error,
	options::{FindOneOptions, FindOptions},
	results::{DeleteResult, InsertOneResult, UpdateResult},
	Cursor,
};
use serde::{Deserialize, Serialize};

use super::craziness::Craziness;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Night {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub craziness: Craziness,
}

impl Night {
	pub async fn create_night(
		collection: mongodb::Collection<Night>,
		item: Night,
	) -> std::result::Result<InsertOneResult, Error> {
		collection.insert_one(item, None).await
	}

	pub async fn get_all_nights(
		collection: mongodb::Collection<Night>,
	) -> std::result::Result<Cursor<Night>, Error> {
		let find_options = FindOptions::builder().limit(None).build();
		collection.find(None, find_options).await
	}

	pub async fn get_night(
		collection: mongodb::Collection<Night>,
		item_id: ObjectId,
	) -> std::result::Result<Option<Night>, Error> {
		let find_option = FindOneOptions::builder().build();
		collection.find_one(bson::doc! {"_id": item_id}, find_option).await
	}

	pub async fn delete_night(
		collection: mongodb::Collection<Night>,
		item_id: ObjectId,
	) -> std::result::Result<DeleteResult, Error> {
		collection.delete_one(bson::doc! {"_id": item_id }, None).await
	}

	pub async fn edit_night(
		collection: mongodb::Collection<Night>,
		item_id: ObjectId,
		craziness: Craziness,
	) -> std::result::Result<UpdateResult, Error> {
		let query = bson::doc! { "_id": item_id };
		let doc = bson::to_document(&craziness).unwrap();
		let update = bson::doc! {"$set": { "craziness": doc }};
		collection.update_one(query, update, None).await
	}
}
