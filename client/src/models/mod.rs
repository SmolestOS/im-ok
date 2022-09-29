use bson::oid::ObjectId;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
/// An enum to track the level of Drunkness (0 - 5)
pub enum User {
	Lostsaka,
	Gkasma,
}

impl Default for User {
	fn default() -> Self {
		Self::Lostsaka
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
/// An enum to track the level of Drunkness (0 - 5)
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
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum AppState {
	Editing,
	Viewing,
	Submit,
}

impl Default for AppState {
	fn default() -> Self {
		Self::Submit
	}
}

/// A struct to track the result of the night
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Craziness {
	pub user: User,
	pub drunkness: Drunkness,
	pub coitus: bool,
	pub drive: bool,
	pub talked_2x: bool,
	pub location: String,
	pub description: String,
	pub date: DateTime<Local>,
}

impl Default for Craziness {
	fn default() -> Self {
		Self {
			user: User::default(),
			drunkness: Drunkness::default(),
			coitus: false,
			drive: false,
			talked_2x: false,
			location: "Athens".to_string(),
			description: "Kala htan".to_string(),
			date: DateTime::<Local>::default(),
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Night {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub craziness: Craziness,
}

impl Night {
	pub fn create_night(item: Night) -> std::result::Result<ureq::Response, ureq::Error> {
		ureq::post("http://localhost:3000/nights/new")
			.set("Content-Type", "application/json")
			.send_json(item.craziness)
	}

	pub fn delete_night(item_id: ObjectId) -> std::result::Result<ureq::Response, ureq::Error> {
		ureq::delete(&format!("http://localhost:3000/nights/{}", item_id)).call()
	}

	pub fn edit_night(
		item_id: ObjectId,
		craziness: Craziness,
	) -> std::result::Result<ureq::Response, ureq::Error> {
		ureq::patch(&format!("http://localhost:3000/nights/{}", item_id))
			.set("Content-Type", "application/json")
			.send_json(ureq::json!(craziness))
	}

	pub fn get_all_nights() -> std::result::Result<ureq::Response, ureq::Error> {
		ureq::get("http://localhost:3000/nights").call()
	}
}
