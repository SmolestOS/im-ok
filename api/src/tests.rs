#[allow(clippy::module_inception)]
#[cfg(test)]
mod tests {
	use crate::router;
	use api::models::{
		night::responses::{ResponseNight, ResponseNights},
		user::{responses::LoginResponse, UserJSONRequest},
	};
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

	#[tokio::test]
	async fn get_all_nights_should_work() {
		dotenvy::dotenv().ok().unwrap();

		let router = router().await;
		let client = TestClient::new(router);
		let login_res: LoginResponse = client
			.post("/users/login")
			.json(&UserJSONRequest {
				username: "username".to_string(),
				password: "password".to_string(),
			})
			.send()
			.await
			.json()
			.await;

		let token = login_res.data.unwrap().token;
		let nights_res: ResponseNights = client
			.get("/nights")
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;

		// assert_ne: Not Equal, therefore nights are not 0, as intented
		assert_ne!(0, nights_res.data.unwrap().len());
	}

	#[tokio::test]
	async fn get_one_night_should_work() {
		dotenvy::dotenv().ok().unwrap();

		let router = router().await;
		let client = TestClient::new(router);
		let login_res: LoginResponse = client
			.post("/users/login")
			.json(&UserJSONRequest {
				username: "username".to_string(),
				password: "password".to_string(),
			})
			.send()
			.await
			.json()
			.await;

		let token = login_res.data.unwrap().token;
		let nights_res: ResponseNight = client
			.get("/nights/51")
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;

		assert_eq!(51, nights_res.data.unwrap().id);
	}
}
