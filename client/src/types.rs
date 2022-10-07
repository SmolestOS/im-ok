use super::models::night::Night;
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum AppState {
	LoginRegister,
	Editing,
	Viewing,
	Submit,
}

impl Default for AppState {
	fn default() -> Self {
		Self::LoginRegister
	}
}

impl AppState {
	pub fn set_app_state(&mut self, state: AppState) {
		*self = state;
	}
}
