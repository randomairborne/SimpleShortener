pub async fn list_redirects(_: crate::structs::Authorization) -> impl axum::response::IntoResponse {
    let links = match crate::URLS.get() {
        None => {
            return Err(crate::structs::Errors::InternalError);
        }
        Some(links) => links,
    };
    let json_response = match serde_json::to_string(&crate::structs::List {
        links: links.clone(),
    }) {
        Ok(json) => json,
        Err(_) => {
            return Err(crate::structs::Errors::InternalError);
        }
    };
    Ok(json_response + "\n")
}

pub async fn edit(
    _: crate::structs::Authorization,
    axum::extract::Json(payload): axum::extract::Json<crate::structs::Edit>,
) -> impl axum::response::IntoResponse {
    let links = match crate::URLS.get() {
        None => return Err(crate::structs::Errors::InternalError),
        Some(links) => links,
    };
    match links.get(&payload.link) {
        None => {
            tracing::trace!("Could not edit {}, not found", payload.link);
            return Err(crate::structs::Errors::NotFound);
        }
        Some(_) => {}
    };
    let disallowed_shortenings = match crate::DISALLOWED_SHORTENINGS.get() {
        None => return Err(crate::structs::Errors::InternalError),
        Some(ds) => ds,
    };
    if disallowed_shortenings.contains(payload.link.as_str()) {
        return Err(crate::structs::Errors::UrlConflict);
    }
    let db = match crate::DB.get() {
        None => return Err(crate::structs::Errors::InternalError),
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
        Err(_) => return Err(crate::structs::Errors::InternalError),
    };
    if sqlx_result != 1 {
        return Err(crate::structs::Errors::NotFoundJson);
    }
    links.remove(payload.link.as_str());
    links.insert(payload.link, payload.destination);

    Ok(r#"{"message":"Link edited!"}\n"#)
}

pub async fn delete(
    _: crate::structs::Authorization,
    axum::extract::Json(payload): axum::extract::Json<crate::structs::Delete>,
) -> impl axum::response::IntoResponse {
    let links = match crate::URLS.get() {
        None => return Err(crate::structs::Errors::InternalError),
        Some(links) => links,
    };
    match links.get(&payload.link) {
        None => {
            tracing::trace!("Could not delete {}, not found", payload.link);
            return Err(crate::structs::Errors::NotFound);
        }
        Some(_) => {}
    };
    let disallowed_shortenings = match crate::DISALLOWED_SHORTENINGS.get() {
        None => return Err(crate::structs::Errors::InternalError),
        Some(ds) => ds,
    };
    if disallowed_shortenings.contains(payload.link.as_str()) {
        return Err(crate::structs::Errors::UrlConflict);
    }
    let db = match crate::DB.get() {
        None => return Err(crate::structs::Errors::InternalError),
        Some(db) => db,
    };
    let sqlx_result = match sqlx::query!("DELETE FROM links WHERE link = $1", payload.link)
        .execute(db)
        .await
    {
        Ok(result) => result.rows_affected(),
        Err(_) => return Err(crate::structs::Errors::InternalError),
    };
    if sqlx_result != 1 {
        return Err(crate::structs::Errors::NotFoundJson);
    }
    links.remove(payload.link.as_str());

    Ok(r#"{"message":"Link removed!"}\n"#)
}

pub async fn add(
    _: crate::structs::Authorization,
    axum::extract::Json(payload): axum::extract::Json<crate::structs::Add>,
) -> Result<(axum::http::StatusCode, &'static str), crate::structs::Errors> {
    let links = match crate::URLS.get() {
        None => return Err(crate::structs::Errors::InternalError),
        Some(links) => links,
    };
    match links.get(&payload.link) {
        None => {}
        Some(_) => {
            return Err(crate::structs::Errors::UrlConflict);
        }
    };
    let disallowed_shortenings = match crate::DISALLOWED_SHORTENINGS.get() {
        None => return Err(crate::structs::Errors::InternalError),
        Some(ds) => ds,
    };
    if disallowed_shortenings.contains(payload.link.as_str()) {
        return Err(crate::structs::Errors::UrlConflict);
    }
    let db = match crate::DB.get() {
        None => return Err(crate::structs::Errors::InternalError),
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
        return Err(crate::structs::Errors::InternalError);
    }
    links.insert(payload.link, payload.destination);

    Ok((
        axum::http::status::StatusCode::CREATED,
        r#"{"message":"Link added!"}\n"#,
    ))
}
