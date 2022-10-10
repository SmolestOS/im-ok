use super::model::Night;
use bson::Bson;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct CreateResponse {
	pub msg: String,
	pub data: Option<Bson>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct ResponseMany {
	pub msg: String,
	pub data: Option<Vec<Night>>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct ResponseOne {
	pub msg: String,
	pub data: Option<Night>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DeleteResponse {
	pub msg: String,
	pub data: Option<usize>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct EditResponse {
	pub msg: String,
	pub data: Option<usize>,
}
