use crate::structs::{Authorization, Delete, Edit, Errors, List};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn list(_: crate::structs::Authorization) -> Result<Json<List>, Errors> {
    Ok(Json(List {
        links: crate::URLS.get().ok_or(Errors::UrlsNotFound)?.clone(),
    }))
}

pub async fn edit(_: Authorization, Json(payload): Json<Edit>) -> impl IntoResponse {
    let links = match crate::URLS.get() {
        None => return Err(Errors::InternalError),
        Some(links) => links,
    };
    match links.get(&payload.link) {
        None => {
            tracing::trace!("Could not edit {}, not found", payload.link);
            return Err(Errors::NotFoundJson);
        }
        Some(_) => {}
    };
    let disallowed_shortenings = match crate::DISALLOWED_SHORTENINGS.get() {
        None => return Err(Errors::InternalError),
        Some(ds) => ds,
    };
    if disallowed_shortenings.contains(payload.link.as_str()) {
        return Err(Errors::UrlConflict);
    }
    let db = match crate::DB.get() {
        None => return Err(Errors::InternalError),
        Some(db) => db,
    };
    let sqlx_result = match sqlx::query!(
        "UPDATE links SET destination = $1 WHERE link = $2 ",
        payload.destination,
        payload.link
    )
    .execute(db)
    .await
    {
        Ok(result) => result.rows_affected(),
        Err(_) => return Err(Errors::InternalError),
    };
    if sqlx_result != 1 {
        return Err(Errors::NotFoundJson);
    }
    links.remove(payload.link.as_str());
    links.insert(payload.link, payload.destination);

    Ok(r#"{"message":"Link edited!"}\n"#)
}

pub async fn delete(_: Authorization, Json(payload): Json<Delete>) -> impl IntoResponse {
    let links = match crate::URLS.get() {
        None => return Err(Errors::InternalError),
        Some(links) => links,
    };
    match links.get(&payload.link) {
        None => {
            tracing::trace!("Could not delete {}, not found", payload.link);
            return Err(Errors::NotFoundJson);
        }
        Some(_) => {}
    };
    let disallowed_shortenings = match crate::DISALLOWED_SHORTENINGS.get() {
        None => return Err(Errors::InternalError),
        Some(ds) => ds,
    };
    if disallowed_shortenings.contains(payload.link.as_str()) {
        return Err(Errors::UrlConflict);
    }
    let db = match crate::DB.get() {
        None => return Err(Errors::InternalError),
        Some(db) => db,
    };
    let sqlx_result = match sqlx::query!("DELETE FROM links WHERE link = $1", payload.link)
        .execute(db)
        .await
    {
        Ok(result) => result.rows_affected(),
        Err(_) => return Err(Errors::InternalError),
    };
    if sqlx_result != 1 {
        return Err(Errors::NotFoundJson);
    }
    links.remove(payload.link.as_str());

    Ok(r#"{"message":"Link removed!"}\n"#)
}

pub async fn add(
    _: Authorization,
    Json(payload): Json<crate::structs::Add>,
) -> Result<(StatusCode, &'static str), Errors> {
    let links = match crate::URLS.get() {
        None => return Err(Errors::InternalError),
        Some(links) => links,
    };
    match links.get(&payload.link) {
        None => {}
        Some(_) => {
            return Err(Errors::UrlConflict);
        }
    };
    let disallowed_shortenings = match crate::DISALLOWED_SHORTENINGS.get() {
        None => return Err(Errors::InternalError),
        Some(ds) => ds,
    };
    if disallowed_shortenings.contains(payload.link.as_str()) {
        return Err(Errors::UrlConflict);
    }
    let db = match crate::DB.get() {
        None => return Err(Errors::InternalError),
        Some(db) => db,
    };
    if let Err(_) = sqlx::query!(
        "INSERT INTO links VALUES ($1,$2)",
        payload.link,
        payload.destination
    )
    .execute(db)
    .await
    {
        return Err(Errors::InternalError);
    }
    links.insert(payload.link, payload.destination);

    Ok((StatusCode::CREATED, r#"{"message":"Link added!"}\n"#))
}
