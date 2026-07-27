use axum::{Router, routing::get};

pub fn app() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> &'static str {
    "ok"
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use super::app;

    #[tokio::test]
    async fn health_endpoint_returns_ok() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/health")
                    .body(Body::empty())
                    .expect("the test request should be valid"),
            )
            .await
            .expect("the router should handle the request");

        assert_eq!(response.status(), StatusCode::OK);
    }
}
