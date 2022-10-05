use mongodb::bson::oid::ObjectId;
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub username: String,
	pub password: String,
}
