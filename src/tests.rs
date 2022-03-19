#[cfg(test)]
mod tests {
    use crate::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::util::ServiceExt;
    //use serde_json::{json, Value};

    #[tokio::test]
    async fn root() {
        init();
        let app = crate::utils::build_app();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT);
    }
    #[tokio::test]
    async fn incorrect_auth() {
        init();
        let app = crate::utils::build_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/simpleshortener/api/add")
                    .header("username", "username")
                    .header("password", "password")
                    .body(Body::from(
                        r#"{"link": "shorturl","destination": "https://example.com"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        println!("{:#?}", response);
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
