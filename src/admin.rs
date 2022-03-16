use crate::db::flush_urls;
use crate::structs::{Add, Authorization, Edit, List, Qr, WebServerError};
use crate::utils::qrgen;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use hyper::header::{HeaderMap, HeaderValue};
use std::ops::Not;

static DISALLOWED_SHORTENINGS: [&'static str; 3] = ["", "favicon.ico", "simpleshortener"];

pub async fn list(_: crate::structs::Authorization) -> Result<Json<List>, WebServerError> {
    Ok(Json(List {
        links: crate::URLS.get().ok_or(WebServerError::UrlsNotFound)?,
    }))
}

pub async fn edit(
    _: Authorization,
    Path(link): Path<String>,
    Json(Edit { destination }): Json<Edit>,
) -> Result<&'static str, WebServerError> {
    let links = crate::URLS.get().ok_or(WebServerError::UrlsNotFound)?;
    links
        .contains_key(&link)
        .then(|| ())
        .ok_or(WebServerError::NotFoundJson)?;

    links.insert(link, destination);
    flush_urls()?;
    Ok(r#"{"message":"Link edited!"}\n"#)
}

pub async fn delete(
    _: Authorization,
    Path(link): Path<String>,
) -> Result<&'static str, WebServerError> {
    let links = crate::URLS.get().ok_or(WebServerError::UrlsNotFound)?;
    links
        .contains_key(&link)
        .then(|| ())
        .ok_or(WebServerError::NotFoundJson)?;

    links.remove(&link);
    flush_urls()?;
    Ok(r#"{"message":"Link removed!"}"#)
}

pub async fn add(
    _: Authorization,
    Json(Add {
        mut link,
        destination,
    }): Json<Add>,
) -> Result<(StatusCode, &'static str), WebServerError> {
    link = link.to_lowercase();
    let links = crate::URLS.get().ok_or(WebServerError::UrlsNotFound)?;
    (!links.contains_key(&link))
        .then(|| ())
        .ok_or(WebServerError::UrlConflict)?;

    DISALLOWED_SHORTENINGS
        .contains(&link.as_str())
        .not()
        .then(|| ())
        .ok_or(WebServerError::UrlDisallowed)?;

    links.insert(link, destination);
    flush_urls()?;
    Ok((StatusCode::CREATED, r#"{"message":"Link added!"}"#))
}

// basic handler that responds with a dynamic QR code
pub async fn qr(
    Json(Qr { destination }): Json<Qr>,
) -> Result<(StatusCode, HeaderMap, Vec<u8>), WebServerError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        hyper::header::CONTENT_TYPE,
        HeaderValue::from_static("image/bmp"),
    );
    tracing::debug!("Handling qr code reqeust: {}", destination);
    let qr_bmp = qrgen(destination)?;
    Ok((StatusCode::OK, headers, qr_bmp))
}
