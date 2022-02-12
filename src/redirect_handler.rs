use crate::structs::WebServerError;
use axum::extract::Path;

pub async fn redirect(
    Path(path): Path<String>,
) -> Result<(axum::http::StatusCode, axum::http::HeaderMap), WebServerError> {
    let destination_url = crate::URLS
        .get()
        .ok_or(WebServerError::UrlsNotFound)?
        .get(&path)
        .ok_or(WebServerError::NotFound)?
        .value();
    tracing::trace!("Path: {}, Destination: {}", path, destination_url);
    let mut headers = axum::http::HeaderMap::new();
    let destination = match axum::http::HeaderValue::from_str(destination_url) {
        Ok(dest) => dest,
        Err(_) => return Err(WebServerError::InvalidRedirectUri),
    };
    headers.insert(axum::http::header::LOCATION, destination);
    Ok((axum::http::StatusCode::PERMANENT_REDIRECT, headers))
}
