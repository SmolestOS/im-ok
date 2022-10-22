#[allow(clippy::module_inception)]
#[cfg(test)]
mod tests {
	use crate::router;
	use api::models::user::{responses::LoginResponse, UserJSONRequest};
	use axum_test_helper::TestClient;

	#[tokio::test]
	async fn users_login_should_work() {
		dotenvy::dotenv().ok().unwrap();

		let router = router().await;
		let client = TestClient::new(router);
		let res: LoginResponse = client
			.post("/users/login")
			.json(&UserJSONRequest {
				username: "username".to_string(),
				password: "password".to_string(),
			})
			.send()
			.await
			.json()
			.await;

		let data = res.data.as_ref().unwrap();

		assert_eq!(1, data.user.id);
		assert_eq!("username".to_string(), data.user.username);
	}
}
