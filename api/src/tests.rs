#[allow(clippy::module_inception)]
#[cfg(test)]
mod tests {
	use crate::router;
	use api::models::{
		night::{
			responses::{
				CreateNightResponse, DeleteNightResponse, EditResponse, ResponseNight,
				ResponseNights, ResponseNightsWithUser,
			},
			Drunkness, NightJSONRequest,
		},
		user::{
			responses::{CreateUserResponse, DeleteUserResponse, LoginResponse},
			UserJSONRequest,
		},
	};
	use axum_test_helper::TestClient;

	#[tokio::test]
	async fn users_login_should_work() {
		dotenvy::dotenv().ok().unwrap();

		let router = router().await;
		let client = TestClient::new(router);
		let reg_res: CreateUserResponse = client
			.post("/users/register")
			.json(&UserJSONRequest {
				username: "apitest".to_string(),
				password: "apitest".to_string(),
			})
			.send()
			.await
			.json()
			.await;

		let login_res: LoginResponse = client
			.post("/users/login")
			.json(&UserJSONRequest {
				username: "apitest".to_string(),
				password: "apitest".to_string(),
			})
			.send()
			.await
			.json()
			.await;

		let login_data = login_res.data.as_ref().unwrap();

		assert_eq!(reg_res.data.unwrap(), login_data.user.id);
		assert_eq!("apitest".to_string(), login_data.user.username);
		let _users_res: DeleteUserResponse = client
			.delete(format!("/users/{}", reg_res.data.unwrap()).as_str())
			.send()
			.await
			.json()
			.await;
	}

	#[tokio::test]
	async fn users_register_should_work() {
		dotenvy::dotenv().ok().unwrap();

		let router = router().await;
		let client = TestClient::new(router);
		let res: CreateUserResponse = client
			.post("/users/register")
			.json(&UserJSONRequest {
				username: "apitest".to_string(),
				password: "apitest".to_string(),
			})
			.send()
			.await
			.json()
			.await;

		let data = res.data.as_ref().unwrap();
		let _users_res: DeleteUserResponse = client
			.delete(format!("/users/{}", res.data.unwrap()).as_str())
			.send()
			.await
			.json()
			.await;

		assert_ne!(0, *data);
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
		let nights_create_res: CreateNightResponse = client
			.post("/nights/new")
			.json(&NightJSONRequest {
				user_id: 1,
				drunkness: Drunkness::Ant,
				coitus: true,
				drive: true,
				talked_2x: true,
				location: "apitest".to_string(),
				description: "apitest".to_string(),
			})
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;
		let nights_res: ResponseNights = client
			.get("/nights")
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;

		// assert_ne: Not Equal, therefore nights are not 0, as intented
		assert_ne!(0, nights_res.data.unwrap().len());
		let _nights_delete_res: DeleteNightResponse = client
			.delete(format!("/nights/{}", nights_create_res.data.unwrap()).as_str())
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;
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
		let nights_create_res: CreateNightResponse = client
			.post("/nights/new")
			.json(&NightJSONRequest {
				user_id: 1,
				drunkness: Drunkness::Ant,
				coitus: true,
				drive: true,
				talked_2x: true,
				location: "apitest".to_string(),
				description: "apitest".to_string(),
			})
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;
		let nights_res: ResponseNight = client
			.get("/nights/41")
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;

		assert_eq!(41, nights_res.data.unwrap().id);
		let _nights_delete_res: DeleteNightResponse = client
			.delete(format!("/nights/{}", nights_create_res.data.unwrap()).as_str())
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;
	}
	#[tokio::test]
	async fn get_all_nights_with_user_should_work() {
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
		let nights_create_res: CreateNightResponse = client
			.post("/nights/new")
			.json(&NightJSONRequest {
				user_id: 1,
				drunkness: Drunkness::Ant,
				coitus: true,
				drive: true,
				talked_2x: true,
				location: "apitest".to_string(),
				description: "apitest".to_string(),
			})
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;

		let nights_res: ResponseNightsWithUser = client
			.get("/nights/with_users")
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;

		assert_ne!(0, nights_res.data.unwrap().len());
		let _nights_delete_res: DeleteNightResponse = client
			.delete(format!("/nights/{}", nights_create_res.data.unwrap()).as_str())
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;
	}
	#[tokio::test]
	async fn create_night_should_work() {
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
		let nights_res: CreateNightResponse = client
			.post("/nights/new")
			.json(&NightJSONRequest {
				user_id: 1,
				drunkness: Drunkness::Ant,
				coitus: true,
				drive: true,
				talked_2x: true,
				location: "manasou".to_string(),
				description: "paterasou".to_string(),
			})
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;

		assert_ne!(0, nights_res.data.unwrap());
	}

	#[tokio::test]
	async fn delete_night_should_work() {
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
		let nights_create_res: CreateNightResponse = client
			.post("/nights/new")
			.json(&NightJSONRequest {
				user_id: 1,
				drunkness: Drunkness::Ant,
				coitus: true,
				drive: true,
				talked_2x: true,
				location: "apitest".to_string(),
				description: "apitest".to_string(),
			})
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;
		let nights_res: DeleteNightResponse = client
			.delete(format!("/nights/{}", nights_create_res.data.unwrap()).as_str())
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;

		assert_eq!(1, nights_res.data.unwrap());
	}
	#[tokio::test]
	async fn edit_night_should_work() {
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
		let nights_create_res: CreateNightResponse = client
			.post("/nights/new")
			.json(&NightJSONRequest {
				user_id: 1,
				drunkness: Drunkness::Ant,
				coitus: true,
				drive: true,
				talked_2x: true,
				location: "apitest".to_string(),
				description: "apitest".to_string(),
			})
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;

		let nights_res: EditResponse = client
			.patch(format!("/nights/{}", nights_create_res.data.unwrap()).as_str())
			.json(&NightJSONRequest {
				user_id: 1,
				drunkness: Drunkness::Ant,
				coitus: true,
				drive: true,
				talked_2x: true,
				location: "apitestedit".to_string(),
				description: "apitestedit".to_string(),
			})
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;

		assert_ne!(0, nights_res.data.unwrap());
		let _nights_delete_res: DeleteNightResponse = client
			.delete(format!("/nights/{}", nights_create_res.data.unwrap()).as_str())
			.header("Authorization", format!("Bearer {}", token).to_string())
			.send()
			.await
			.json()
			.await;
	}
}
