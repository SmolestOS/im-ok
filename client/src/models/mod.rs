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
