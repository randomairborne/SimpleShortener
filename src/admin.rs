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
    Ok(json_response)
}

pub async fn edit(
    _: crate::structs::Authorization,
    axum::extract::Json(payload): axum::extract::Json<crate::structs::Edit>,
) -> impl axum::response::IntoResponse {
    let db = match crate::DB.get() {
        None => return Err(crate::structs::Errors::InternalError),
        Some(db) => db,
    };
    let sqlx_result = match sqlx::query!(
        "UPDATE links SET link = $1 WHERE destination = $2 ",
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
    let links = match crate::URLS.get() {
        None => return Err(crate::structs::Errors::InternalError),
        Some(links) => links,
    };
    links.remove(payload.link.as_str());

    Ok(r#"{"message":"Link edited!"}"#)
}

pub async fn delete(
    _: crate::structs::Authorization,
    axum::extract::Json(payload): axum::extract::Json<crate::structs::Delete>,
) -> impl axum::response::IntoResponse {
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
    let links = match crate::URLS.get() {
        None => return Err(crate::structs::Errors::InternalError),
        Some(links) => links,
    };
    links.remove(payload.link.as_str());

    Ok(r#"{"message":"Link removed!"}"#)
}

pub async fn add(
    _: crate::structs::Authorization,
    axum::extract::Json(payload): axum::extract::Json<crate::structs::Add>,
) -> Result<(axum::http::StatusCode, &'static str), crate::structs::Errors> {
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
    let links = match crate::URLS.get() {
        None => return Err(crate::structs::Errors::InternalError),
        Some(links) => links,
    };
    links.insert(payload.link, payload.destination);

    Ok((axum::http::status::StatusCode::CREATED, r#"{"message":"Link added!"}"#))
}
