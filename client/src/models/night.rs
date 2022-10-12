use api::models::night::{Night, NightJSONRequest};

pub fn create_night(item: NightJSONRequest) -> std::result::Result<ureq::Response, ureq::Error> {
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
