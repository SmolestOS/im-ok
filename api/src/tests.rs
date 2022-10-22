#[cfg(test)]
pub mod tests {
    use axum::http::StatusCode;
    use axum_test_helper::TestClient;
    use axum::{
	routing::{delete, get, patch, post},
	Router,
    };
    use crate::controllers::nights::*;


    #[tokio::test]
    async fn test_main_router() {
        let router = night_routes;
        let client = TestClient::new(router);
        let res = client.get("/").send().await;
        assert_eq!(res.status(), StatusCode::OK);
    }
}
