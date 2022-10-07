use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn register(item: User) -> std::result::Result<ureq::Response, ureq::Error> {
	ureq::post("http://localhost:3000/users/register")
	    .set("Content-Type", "application/json")
	    .send_json(item)
    }
}
