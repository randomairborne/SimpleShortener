use axum::extract::Path;
use axum::http::{HeaderMap, HeaderValue, StatusCode};

// basic handler that responds with a static string
pub async fn root() -> (StatusCode, HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::try_from("text/html").unwrap());
    (StatusCode::OK, headers, include_str!("root.html"))
}

pub async fn redirect(Path(path): Path<String>) -> (StatusCode, HeaderMap, &'static str) {
    let urls = match crate::URLS.get().clone() {
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                HeaderMap::default(),
                "There was a severe internal server error.",
            )
        }
        Some(config) => config,
    };
    for (shortening, destination_url) in urls.iter() {
        tracing::log::debug!(
            "Shortening: {}, Path: {}, Destination: {}",
            shortening,
            path,
            destination_url
        );
        if shortening == &path {
            let mut headers = HeaderMap::new();
            headers.insert("Location", HeaderValue::try_from(destination_url).unwrap());
            return (StatusCode::FOUND, headers, "Redirecting...");
        }
    }
    (
        StatusCode::NOT_FOUND,
        HeaderMap::default(),
        "404 redirect not found, ask the sender if they made a typo",
    )
}
