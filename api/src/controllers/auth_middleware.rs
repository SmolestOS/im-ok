use axum::{
	http::{Request, StatusCode},
	middleware::Next,
	response::Response,
};
use jsonwebtoken::{
	decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

pub async fn auth_middleware<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
	let auth_header = req
		.headers()
		.get(axum::http::header::AUTHORIZATION)
		.and_then(|header| header.to_str().ok());

	let auth_header = if let Some(auth_header) = auth_header {
		auth_header
	} else {
		return Err(StatusCode::BAD_GATEWAY)
	};

	if authorize_user(&auth_header[7..auth_header.len()]).await {
		Ok(next.run(req).await)
	} else {
		Err(StatusCode::UNAUTHORIZED)
	}
}

async fn authorize_user(auth_token: &str) -> bool {
	let mut validation = Validation::new(Algorithm::HS256);
	validation.sub = Some("b@b.com".to_string());
	validation.set_audience(&["me"]);
	let key = b"secret";
	tracing::debug!("{}", auth_token);

	match decode::<Claims>(auth_token, &DecodingKey::from_secret(key), &validation) {
		Ok(c) => {
			tracing::debug!("{:?}", c);
			true
		},
		Err(err) => match *err.kind() {
			ErrorKind::InvalidToken => {
				tracing::debug!("{:?}", err);
				false
			}, // Example on how to handle a specific error
			ErrorKind::InvalidIssuer => {
				tracing::debug!("{:?}", err);
				false
			},
			_ => false,
		},
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
	aud: String,
	sub: String,
	company: String,
	exp: usize,
}

pub async fn token_gen() -> jsonwebtoken::errors::Result<String> {
	let key = b"secret";
	let my_claims = Claims {
		aud: "me".to_owned(),
		sub: "b@b.com".to_owned(),
		company: "ACME".to_owned(),
		exp: 10000000000,
	};
	encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key))
}
