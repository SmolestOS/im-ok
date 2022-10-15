use axum::{
	http::{Request, StatusCode},
	middleware::Next,
	response::Response,
};

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
	if auth_token.eq("TOKEN") {
		true
	} else {
		false
	}
}
