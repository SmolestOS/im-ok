use mongodb::{bson::oid::ObjectId, error::Error, results::InsertOneResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub username: String,
	pub password: String,
}

impl User {
	pub async fn create_user(
		collection: mongodb::Collection<User>,
		item: User,
	) -> std::result::Result<InsertOneResult, Error> {
		collection.insert_one(item, None).await
	}
}
