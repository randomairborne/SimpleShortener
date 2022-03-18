use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use tracing::debug;

// basic handler that responds with a static string
pub async fn doc() -> (StatusCode, HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
    debug!("Handling doc request");
    #[cfg(not(debug_assertions))]
    let docs = include_str!("resources/doc.html").to_string();
    #[cfg(debug_assertions)]
    let docs = std::fs::read_to_string("src/resources/doc.html")
        .expect("Program is in debug mode and the documentation file was not found!");
    (StatusCode::OK, headers, docs)
}

// basic handler that responds with a static string
pub async fn panel_html() -> (StatusCode, HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
    debug!("Handling html request");
    #[cfg(not(debug_assertions))]
    let panel = include_str!("resources/panel.html").to_string();
    #[cfg(debug_assertions)]
    let panel = std::fs::read_to_string("src/resources/panel.html")
        .expect("Program is in debug mode and the panel file was not found!");
    (StatusCode::OK, headers, panel)
}

// basic handler that responds with a static icon file
pub async fn favicon() -> (StatusCode, HeaderMap, Vec<u8>) {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/ico"));
    debug!("Handling favicon request");
    (
        StatusCode::OK,
        headers,
        include_bytes!("resources/favicon.ico").to_vec(),
    )
}

// basic handler that responds with a static png file
pub async fn logo() -> (StatusCode, HeaderMap, Vec<u8>) {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));
    debug!("Handling logo request");
    (
        StatusCode::OK,
        headers,
        include_bytes!("resources/link.png").to_vec(),
    )
}
