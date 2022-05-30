use crate::error::WebServerError;
use crate::structs::{Add, Edit, Qr};
use crate::users::Authorization;
use crate::UrlMap;
use axum::extract::{Extension, Path};
use axum::http::header::{HeaderMap, HeaderValue};
use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;
use sqlx::PgPool;

static DISALLOWED_SHORTENINGS: [&str; 3] = ["", "favicon.ico", "simpleshortener"];

#[allow(clippy::unused_async)]
pub async fn list(_: Authorization, Extension(links): Extension<UrlMap>) -> Json<Value> {
    Json(json!({ "links": links }))
}

pub async fn edit(
    _: Authorization,
    Path(link): Path<String>,
    Json(Edit { destination }): Json<Edit>,
    Extension(links): Extension<UrlMap>,
    Extension(db): Extension<&PgPool>,
) -> Result<Json<Value>, WebServerError> {
    links
        .contains_key(&link)
        .then(|| ())
        .ok_or(WebServerError::NotFound)?;
    query!(
        "UPDATE urls SET destination = $1 WHERE link = $2",
        destination,
        link
    )
    .execute(db)
    .await?;
    links.insert(link, destination);
    Ok(Json(json!({"message":"Link edited!"})))
}

pub async fn delete(
    _: Authorization,
    Path(link): Path<String>,
    Extension(links): Extension<UrlMap>,
    Extension(db): Extension<&PgPool>,
) -> Result<Json<Value>, WebServerError> {
    links
        .contains_key(&link)
        .then(|| ())
        .ok_or(WebServerError::NotFound)?;
    query!("DELETE FROM urls WHERE link = $1", link)
        .execute(db)
        .await?;
    links.remove(&link);
    Ok(Json(json!({"message":"Link removed!"})))
}

pub async fn add(
    _: Authorization,
    Json(Add {
        mut link,
        destination,
    }): Json<Add>,
    Extension(links): Extension<UrlMap>,
    Extension(db): Extension<&PgPool>,
) -> Result<Json<Value>, WebServerError> {
    link = link.to_lowercase();
    (!links.contains_key(&link))
        .then(|| ())
        .ok_or(WebServerError::UrlConflict)?;

    (!DISALLOWED_SHORTENINGS.contains(&link.as_str()))
        .then(|| ())
        .ok_or(WebServerError::UrlDisallowed)?;
    query!("INSERT INTO urls VALUES ($1, $2)", link, destination)
        .execute(db)
        .await?;

    links.insert(link, destination);
    Ok(Json(json!({"message":"Link added!"})))
}

#[allow(clippy::unused_async)]
pub async fn qr(
    Json(Qr { destination }): Json<Qr>,
) -> Result<(StatusCode, HeaderMap, Vec<u8>), WebServerError> {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_static("image/bmp"),
    );
    tracing::debug!("Handling qr code reqeust: {}", destination);
    let qr_code = qr_code::QrCode::new(destination.as_bytes())?;
    let bmp = qr_code.to_bmp();
    let mut bmp_vec: Vec<u8> = Vec::new();
    bmp.write(&mut bmp_vec)?;
    Ok((StatusCode::OK, headers, bmp_vec))
}
