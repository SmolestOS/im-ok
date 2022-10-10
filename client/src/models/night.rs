use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Drunkness {
	Cool,
	LittleHead,
	Bream,
	Gnat,
	Ant,
	ImOk,
}

impl Default for Drunkness {
	fn default() -> Self {
		Self::Cool
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Night {
	pub id: i32,
	pub user_id: i32,
	pub drunkness: Drunkness,
	pub coitus: bool,
	pub drive: bool,
	pub talked_2x: bool,
	pub location: String,
	pub description: String,
	pub created_at: NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct NightJSONRequest {
	pub user_id: i32,
	pub drunkness: Drunkness,
	pub coitus: bool,
	pub drive: bool,
	pub talked_2x: bool,
	pub location: String,
	pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewNightDB {
	pub user_id: i32,
	pub drunkness: Drunkness,
	pub coitus: bool,
	pub drive: bool,
	pub talked_2x: bool,
	pub location: String,
	pub description: String,
	pub created_at: NaiveDate,
}

impl Night {
	pub fn create_night(
		item: NightJSONRequest,
	) -> std::result::Result<ureq::Response, ureq::Error> {
		ureq::post("http://localhost:3000/nights/new")
			.set("Content-Type", "application/json")
			.send_json(item)
	}

	pub fn delete_night(item_id: i32) -> std::result::Result<ureq::Response, ureq::Error> {
		ureq::delete(&format!("http://localhost:3000/nights/{}", item_id)).call()
	}

	pub fn edit_night(night: Night) -> std::result::Result<ureq::Response, ureq::Error> {
		ureq::patch(&format!("http://localhost:3000/nights/{}", night.id))
			.set("Content-Type", "application/json")
			.send_json(ureq::json!(night))
	}

	pub fn get_all_nights() -> std::result::Result<ureq::Response, ureq::Error> {
		ureq::get("http://localhost:3000/nights").call()
	}
}
