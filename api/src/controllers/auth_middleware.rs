use axum::{
	http::{Request, StatusCode},
	middleware::Next,
	response::Response,
};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

pub async fn auth_middleware<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
	let auth_header = req
		.headers()
		.get(axum::http::header::AUTHORIZATION)
		.and_then(|header| header.to_str().ok());

	let auth_header = if let Some(auth_header) = auth_header {
		auth_header
	} else {
		return Err(StatusCode::UNAUTHORIZED)
	};

	if authorize_user(auth_header).await {
		Ok(next.run(req).await)
	} else {
		Err(StatusCode::UNAUTHORIZED)
	}
}

async fn authorize_user(auth_token: &str) -> bool {
	auth_token.eq("TOKEN")
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Claims {
	sub: String,
	company: String,
	exp: usize,
}

pub async fn token_gen() -> jsonwebtoken::errors::Result<String> {
	let my_claims = Claims {
		sub: "b@b.com".to_owned(),
		company: std::env::var("COMPANY").to_owned().unwrap(),
		exp: 100,
	};
	let key = std::env::var("KEY").unwrap();

	let header =
		Header { kid: Some("signing_key".to_owned()), alg: Algorithm::HS512, ..Default::default() };

	let token = encode(&header, &my_claims, &EncodingKey::from_secret(key.as_bytes()));
	token
}
