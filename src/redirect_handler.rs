use crate::structs::WebServerError;
use axum::extract::Path;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, HeaderValue, StatusCode};

pub async fn redirect(
    Path(mut path): Path<String>,
) -> Result<(StatusCode, HeaderMap), WebServerError> {
    path = path.to_lowercase();
    let destination_url = crate::URLS
        .get()
        .ok_or(WebServerError::UrlsNotFound)?
        .get(&path)
        .ok_or(WebServerError::NotFound)?;
    tracing::debug!("Path: {}, Destination: {}", path, destination_url.as_str());
    let mut headers = HeaderMap::new();
    let destination = match HeaderValue::from_str(destination_url.as_str()) {
        Ok(dest) => dest,
        Err(_) => return Err(WebServerError::InvalidRedirectUri),
    };
    headers.insert(axum::http::header::LOCATION, destination);
    Ok((StatusCode::PERMANENT_REDIRECT, headers))
}

// Checks if a specific config var exists or serves the default root
pub async fn root() -> Result<(StatusCode, HeaderMap, &'static str), WebServerError> {
    tracing::debug!("Handling root request");
    let config = match crate::CONFIG.get() {
        None => return Err(WebServerError::ConfigNotFound),
        Some(config) => config,
    };
    let mut headers = HeaderMap::new();
    match &config.root {
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
