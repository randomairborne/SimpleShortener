use crate::error::WebServerError;
use crate::structs::{Add, Edit, Qr};
use crate::users::authenticate;
use axum::extract::Path;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::header::{HeaderMap, HeaderValue};
use axum::http::StatusCode;
use axum::{Json, TypedHeader};
use serde_json::Value;

#[allow(clippy::unused_async)]
pub async fn list(
    TypedHeader(Authorization(auth)): TypedHeader<Authorization<Bearer>>,
    state: crate::State,
) -> Result<Json<Value>, WebServerError> {
    authenticate(&auth, &state)?;
    Ok(Json(json!({ "links": state.urls })))
}

pub async fn add(
    Json(Add {
        mut link,
        destination,
    }): Json<Add>,
    TypedHeader(Authorization(auth)): TypedHeader<Authorization<Bearer>>,
    state: crate::State,
) -> Result<Json<Value>, WebServerError> {
    authenticate(&auth, &state)?;
    link = link.to_lowercase();
    if link == "simpleshortener" {
        return Err(WebServerError::UrlDisallowed);
    }
    if !link.starts_with('/') {
        link = format!("/{}", &link);
    }
    (!state.urls.contains_key(&link))
        .then(|| ())
        .ok_or(WebServerError::UrlConflict)?;

    sqlx::query!("INSERT INTO urls VALUES ($1, $2)", link, destination)
        .execute(&state.db)
        .await?;

    state.urls.insert(link, destination);
    Ok(Json(json!({"message":"Link added!"})))
}

pub async fn edit(
    Path(link): Path<String>,
    Json(Edit { destination }): Json<Edit>,
    TypedHeader(Authorization(auth)): TypedHeader<Authorization<Bearer>>,
    state: crate::State,
) -> Result<Json<Value>, WebServerError> {
    authenticate(&auth, &state)?;
    let link = urlencoding::decode(&link)?.to_string();
    state
        .urls
        .contains_key(&link)
        .then(|| ())
        .ok_or(WebServerError::NotFound)?;
    query!(
        "UPDATE urls SET destination = $1 WHERE link = $2",
        destination,
        link
    )
    .execute(&state.db)
    .await?;
    state.urls.insert(link, destination);
    Ok(Json(json!({"message":"Link edited!"})))
}

pub async fn delete(
    Path(link): Path<String>,
    TypedHeader(Authorization(auth)): TypedHeader<Authorization<Bearer>>,
    state: crate::State,
) -> Result<Json<Value>, WebServerError> {
    authenticate(&auth, &state)?;
    let link = urlencoding::decode(&link)?.to_string();
    state
        .urls
        .contains_key(&link)
        .then(|| ())
        .ok_or(WebServerError::NotFound)?;
    query!("DELETE FROM urls WHERE link = $1", link)
        .execute(&state.db)
        .await?;
    state.urls.remove(&link);
    Ok(Json(json!({"message":"Link removed!"})))
}

#[allow(clippy::unused_async)]
pub async fn generate_qr(
    Json(Qr { destination }): Json<Qr>,
    TypedHeader(Authorization(auth)): TypedHeader<Authorization<Bearer>>,
    state: crate::State,
) -> Result<(StatusCode, HeaderMap, Vec<u8>), WebServerError> {
    authenticate(&auth, &state)?;
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_static("image/bmp"),
    );
    debug!("Handling qr code reqeust: {}", destination);
    let qr_code = qr_code::QrCode::new(destination.as_bytes())?;
    let bmp = qr_code.to_bmp();
    let mut bmp_vec: Vec<u8> = Vec::new();
    bmp.write(&mut bmp_vec)?;
    Ok((StatusCode::OK, headers, bmp_vec))
}

#[allow(clippy::unused_async)]
pub async fn panel(
    state: crate::State,
) -> (axum::http::StatusCode, axum::http::HeaderMap, &'static str) {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("text/html"),
    );
    let file = if state.is_init.load(std::sync::atomic::Ordering::Relaxed) {
        trace!("Serving standard panel");
        include_str!("resources/panel.html")
    } else {
        trace!("Serving un-init panel");
        include_str!("resources/newuser.html")
    };
    (axum::http::StatusCode::OK, headers, file)
}
