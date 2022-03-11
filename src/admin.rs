use crate::db::flush_urls;
use crate::structs::{Add, Authorization, Edit, List, WebServerError};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use std::ops::Not;

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

    crate::DISALLOWED_SHORTENINGS
        .get()
        .ok_or(WebServerError::DisallowedNotFound)?
        .contains(&link)
        .not()
        .then(|| ())
        .ok_or(WebServerError::UrlDisallowed)?;

    links.insert(link, destination);
    flush_urls()?;
    Ok((StatusCode::CREATED, r#"{"message":"Link added!"}"#))
}
