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
    let docs = std::fs::read_to_string("src/resources/doc.html").expect("Program is in debug mode and the documentation file was not found!");
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
    let panel = std::fs::read_to_string("src/resources/panel.html").expect("Program is in debug mode and the panel file was not found!");
    (
        StatusCode::OK,
        headers,
        panel,
    )
}

// basic handler that responds with a static font file
pub async fn font2() -> (StatusCode, HeaderMap, &'static [u8]) {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("font/woff2"));
    debug!("Handling woff2 font request");
    (
        StatusCode::OK,
        headers,
        include_bytes!("resources/font.woff2").as_ref(),
    )
}

// basic handler that responds with a static font file
pub async fn font() -> (StatusCode, HeaderMap, &'static [u8]) {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("font/woff"));
    debug!("Handling woff font request");
    (
        StatusCode::OK,
        headers,
        include_bytes!("resources/font.woff").as_ref(),
    )
}

// basic handler that responds with a static icon file
pub async fn favicon() -> (StatusCode, HeaderMap, &'static [u8]) {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/ico"));
    debug!("Handling favicon request");
    (
        StatusCode::OK,
        headers,
        include_bytes!("resources/favicon.ico").as_ref(),
    )
}

// basic handler that responds with a static png file
pub async fn logo() -> (StatusCode, HeaderMap, &'static [u8]) {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));
    debug!("Handling logo request");
    (
        StatusCode::OK,
        headers,
        include_bytes!("resources/link.png").as_ref(),
    )
}
