// Checks if a specific config var exists or serves the default root
pub async fn root() -> Result<
    (axum::http::StatusCode, axum::http::HeaderMap, &'static str),
    crate::structs::WebServerError,
> {
    tracing::trace!("Handling root request");
    let config = match crate::CONFIG.get() {
        None => return Err(crate::structs::WebServerError::ConfigNotFound),
        Some(config) => config.clone(),
    };
    let mut headers = axum::http::HeaderMap::new();
    match config.root {
        None => {
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static("text/html"),
            );
            Ok((
                axum::http::StatusCode::OK,
                headers,
                include_str!("resources/root.html"),
            ))
        }
        Some(root) => {
            let destination = match axum::http::HeaderValue::from_str(root.as_str()) {
                Ok(dest) => dest,
                Err(_) => return Err(crate::structs::WebServerError::InvalidRedirectUri),
            };
            headers.insert(axum::http::header::LOCATION, destination);
            Ok((axum::http::StatusCode::PERMANENT_REDIRECT, headers, ""))
        }
    }
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
    tracing::trace!("Handling html request");
    (
        axum::http::StatusCode::OK,
        headers,
        include_str!("resources/panel.html"),
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
