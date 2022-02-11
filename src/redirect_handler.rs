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
    headers.insert(
        axum::http::header::LOCATION,
        axum::http::HeaderValue::from_str(destination_url)
            .unwrap_or_else(|_| return Err(WebServerError::InvalidRedirectUri)),
    );
    Ok((axum::http::StatusCode::PERMANENT_REDIRECT, headers))
}
