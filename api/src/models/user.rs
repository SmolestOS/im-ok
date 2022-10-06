use mongodb::{
	bson::{self, oid::ObjectId},
	error::Error,
	options::FindOneOptions,
	results::InsertOneResult,
};
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

	pub async fn get_user(
		collection: mongodb::Collection<User>,
		user: User,
	) -> std::result::Result<Option<User>, Error> {
		let find_option = FindOneOptions::builder().build();
		collection
			.find_one(
				bson::doc! {"username": user.username, "password": user.password},
				find_option,
			)
			.await
	}
}
