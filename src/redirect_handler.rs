use crate::error::WebServerError;
use crate::State;
use axum::extract::Path;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, HeaderValue, StatusCode};

#[allow(clippy::unused_async)]
pub async fn redirect(
    Path(mut path): Path<String>,
    state: State,
) -> Result<(StatusCode, HeaderMap, &'static str), WebServerError> {
    path = path.to_lowercase();
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );
    let destination_url = match state.urls.get(&path) {
        Some(dest) => dest,
        None => return Ok((StatusCode::OK, headers, include_str!("resources/404.html"))),
    };
    debug!("Path: {}, Destination: {}", path, destination_url.as_str());
    let destination = match HeaderValue::from_str(destination_url.as_str()) {
        Ok(dest) => dest,
        Err(_) => return Err(WebServerError::InvalidRedirectUri),
    };
    headers.insert(axum::http::header::LOCATION, destination);
    Ok((StatusCode::PERMANENT_REDIRECT, headers, ""))
}

// Checks if a specific config var exists or serves the default root
#[allow(clippy::unused_async)]
pub async fn root(state: State) -> Result<(StatusCode, HeaderMap, &'static str), WebServerError> {
    debug!("Handling root request");
    let mut headers = HeaderMap::new();
    match state.urls.get("/") {
        None => {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
            Ok((StatusCode::OK, headers, include_str!("resources/root.html")))
        }
        Some(root) => {
            let destination = match HeaderValue::from_str(root.as_str()) {
                Ok(dest) => dest,
                Err(_) => return Err(WebServerError::InvalidRedirectUri),
            };
            headers.insert(axum::http::header::LOCATION, destination);
            Ok((StatusCode::PERMANENT_REDIRECT, headers, ""))
        }
    }
}
