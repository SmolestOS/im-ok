use crate::models::night::Night;
use mongodb::{Client, Collection};

pub async fn establish_connection() -> Collection<Night> {
	let c = Client::with_uri_str(
		std::env::var("MONGO_URI").expect("MONGO_URI environment variable not set."),
	)
	.await
	.unwrap();

	c.database("im_ok").collection("nights")
}
