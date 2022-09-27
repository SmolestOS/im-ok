pub mod nights;
pub mod user;
use crate::models::night::Night;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Response {
	pub msg: String,
	pub data: Option<Night>,
}
