use crate::structs::{Add, Authorization, Edit, Qr, WebServerError};
use crate::UrlMap;
use axum::extract::{Extension, Path};
use axum::http::header::{HeaderMap, HeaderValue};
use axum::http::StatusCode;
use axum::Json;
use rand::Rng;
use serde_json::Value;
use sha2::Digest;
use sqlx::PgPool;
use std::ops::Not;

static DISALLOWED_SHORTENINGS: [&str; 3] = ["", "favicon.ico", "simpleshortener"];

#[allow(clippy::unused_async)]
pub async fn list(
    _: crate::structs::Authorization,
    Extension(links): Extension<UrlMap>,
) -> Json<Value> {
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

    DISALLOWED_SHORTENINGS
        .contains(&link.as_str())
        .not()
        .then(|| ())
        .ok_or(WebServerError::UrlDisallowed)?;
    query!("INSERT INTO urls VALUES ($1, $2)", link, destination)
        .execute(db)
        .await?;

    links.insert(link, destination);
    Ok(Json(json!({"message":"Link added!"})))
}

#[allow(clippy::unused_async)]
pub async fn token(
    headers: HeaderMap,
    Extension(db): Extension<PgPool>,
    Extension(tokens): Extension<UrlMap>,
) -> Result<Json<Value>, WebServerError> {
    let username = String::from_utf8(
        headers
            .get("username")
            .ok_or(WebServerError::BadRequest)?
            .as_bytes()
            .into(),
    )
    .map_err(|_| WebServerError::BadRequest)?;
    let password = String::from_utf8(
        headers
            .get("password")
            .ok_or(WebServerError::BadRequest)?
            .as_bytes()
            .into(),
    )
    .map_err(|_| WebServerError::BadRequest)?;
    let correct = match query!(
        "SELECT password FROM accounts WHERE username = $1",
        &username
    )
    .fetch_one(&db)
    .await
    {
        Ok(pw) => pw,
        Err(sqlx::Error::RowNotFound) => {
            return Ok(Json(json!({ "error": "Username or password incorrect!" })))
        }
        Err(e) => return Err(WebServerError::Db(e)),
    };
    let provided_password = sha2::Sha512::digest(&password)
        .into_iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();
    if correct.password == provided_password {
        return Ok(Json(json!({ "error": "Username or password incorrect!" })));
    };
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz."
        .chars()
        .collect();
    let mut token = String::with_capacity(64);
    let mut rng = rand::thread_rng();
    for _ in 0..64 {
        token.push(
            *chars
                .get(rng.gen_range(0..chars.len()))
                .ok_or(WebServerError::NotFound)?,
        );
    }
    tokens.insert(username, password);
    Ok(Json(json!({ "token": token })))
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
