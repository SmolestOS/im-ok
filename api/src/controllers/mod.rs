pub mod nights;
pub mod user;
use crate::models::night::Night;
use mongodb::bson::{oid::ObjectId, Bson};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Response {
	pub msg: String,
	pub data: Option<Night>,
}
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct ResponseNights {
	pub msg: String,
	pub data: Option<Vec<Night>>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DeletedResponse {
	pub msg: String,
	pub data: Option<ObjectId>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct CreatedResponse {
	pub msg: String,
	pub data: Option<Bson>,
}
#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct EditedResponse {
	pub msg: String,
	pub data: Option<ObjectId>,
}
