use mongodb::bson::oid::ObjectId;
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	pub username: String,
	pub password: String,
}

impl User {
    pub async fn create_user(
	collection: mongodb::Collection<User>,
	item: User,
    ) -> std::result::Result<InsertOneResult, Error> {
	collection.insert_one(item, None).await
    }

    pub async fn get_user(
	collection: mongodb::Collection<User>,
	item_id: ObjectId,
    ) -> std::result::Result<Option<Night>, Error> {
	let find_option = FindOneOptions::builder().build();
	collection.find_one(bson::doc! {"_id": item_id}, find_option).await
    }

    pub async fn delete_user(
	collection: mongodb::Collection<User>,
	item_id: ObjectId,
    ) -> std::result::Result<DeleteResult, Error> {
	collection.delete_one(bson::doc! {"_id": item_id }, None).await
    }

    pub async fn edit_user(
	collection: mongodb::Collection<User>,
	item_id: ObjectId,
        user: User,
    ) -> std::result::Result<UpdateResult, Error> {
	let query = bson::doc! { "_id": item_id };
	let doc = bson::to_document(&user.password).unwrap();
	let update = bson::doc! {"$set": { "password": doc }};
	collection.update_one(query, update, None).await
   }
}
