
// basic handler that responds with a static string
pub async fn root() -> (axum::http::StatusCode, axum::http::HeaderMap, &'static str) {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("text/html"),
    );
    tracing::trace!("Handling root request");
    (
        axum::http::StatusCode::OK,
        headers,
        include_str!("resources/root.html"),
    )
}

// basic handler that responds with a static string
pub async fn doc() -> (axum::http::StatusCode, axum::http::HeaderMap, &'static str) {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("text/html"),
    );
    tracing::trace!("Handling doc request");
    (
        axum::http::StatusCode::OK,
        headers,
        include_str!("resources/doc.html"),
    )
}

// basic handler that responds with a static string
pub async fn panelhtml() -> (axum::http::StatusCode, axum::http::HeaderMap, &'static str) {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("text/html"),
    );
    tracing::trace!("Handling doc request");
    (
        axum::http::StatusCode::OK,
        headers,
        include_str!("resources/panel/panel.html"),
    )
}

// basic handler that responds with a static string
pub async fn paneljs() -> (axum::http::StatusCode, axum::http::HeaderMap, &'static str) {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("text/javascript"),
    );
    tracing::trace!("Handling doc request");
    (
        axum::http::StatusCode::OK,
        headers,
        include_str!("resources/panel/panel.js"),
    )
}

// basic handler that responds with a static font file
pub async fn font2() -> (axum::http::StatusCode, axum::http::HeaderMap, &'static [u8]) {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("font/woff2"),
    );
    tracing::trace!("Handling woff2 font request");
    (
        axum::http::StatusCode::OK,
        headers,
        include_bytes!("resources/font.woff2").as_ref(),
    )
}

// basic handler that responds with a static font file
pub async fn font() -> (axum::http::StatusCode, axum::http::HeaderMap, &'static [u8]) {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("font/woff"),
    );
    tracing::trace!("Handling woff font request");
    (
        axum::http::StatusCode::OK,
        headers,
        include_bytes!("resources/font.woff").as_ref(),
    )
}

// basic handler that responds with a static icon file
pub async fn favicon() -> (axum::http::StatusCode, axum::http::HeaderMap, &'static [u8]) {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("image/ico"),
    );
    tracing::trace!("Handling favicon request");
    (
        axum::http::StatusCode::OK,
        headers,
        include_bytes!("resources/favicon.ico").as_ref(),
    )
}

// basic handler that responds with a static png file
pub async fn logo() -> (axum::http::StatusCode, axum::http::HeaderMap, &'static [u8]) {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("image/png"),
    );
    tracing::trace!("Handling logo request");
    (
        axum::http::StatusCode::OK,
        headers,
        include_bytes!("resources/link.png").as_ref(),
    )
}