use crate::structs::WebServerError;
use crate::UrlMap;
use axum::extract::{Extension, Path};
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, HeaderValue, StatusCode};

#[allow(clippy::unused_async)]
pub async fn redirect(
    Path(mut path): Path<String>,
    Extension(links): Extension<UrlMap>,
) -> Result<(StatusCode, HeaderMap, &'static str), WebServerError> {
    path = path.to_lowercase();
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );
    let destination_url = match links.get(&path) {
        Some(dest) => dest,
        None => return Ok((StatusCode::OK, headers, include_str!("resources/404.html"))),
    };
    tracing::debug!("Path: {}, Destination: {}", path, destination_url.as_str());
    let destination = match HeaderValue::from_str(destination_url.as_str()) {
        Ok(dest) => dest,
        Err(_) => return Err(WebServerError::InvalidRedirectUri),
    };
    headers.insert(axum::http::header::LOCATION, destination);
    Ok((StatusCode::PERMANENT_REDIRECT, headers, ""))
}

// Checks if a specific config var exists or serves the default root
#[allow(clippy::unused_async)]
pub async fn root(
    Extension(links): Extension<UrlMap>,
) -> Result<(StatusCode, HeaderMap, &'static str), WebServerError> {
    tracing::debug!("Handling root request");
    let mut headers = HeaderMap::new();
    match links.get("/") {
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
