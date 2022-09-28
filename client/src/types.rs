use super::models::Night;
use bson::{oid::ObjectId, Bson};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct ResponseNight {
	pub msg: String,
	pub data: Option<Night>,
}
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct ResponseNights {
	pub msg: String,
	pub data: Option<Vec<Night>>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DeleteResponse {
	pub msg: String,
	pub data: Option<ObjectId>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct CreateResponse {
	pub msg: String,
	pub data: Option<Bson>,
}
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct EditResponse {
	pub msg: String,
	pub data: Option<Bson>,
}
