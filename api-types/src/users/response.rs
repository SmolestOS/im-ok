use super::model::User;
use bson::Bson;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct CreateResponse {
    pub msg: String,
    pub data: Option<Bson>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct LoginResponse {
    pub msg: String,
    pub data: Option<User>,
}
