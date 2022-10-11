use api_types::users::model::UserJSONRequest;

pub fn register(item: UserJSONRequest) -> std::result::Result<ureq::Response, ureq::Error> {
	ureq::post("http://localhost:3000/users/register")
		.set("Content-Type", "application/json")
		.send_json(item)
}

pub fn login(item: UserJSONRequest) -> std::result::Result<ureq::Response, ureq::Error> {
	ureq::get("http://localhost:3000/users/login")
		.set("Content-Type", "application/json")
		.send_json(item)
}
