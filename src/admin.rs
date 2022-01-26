pub async fn list_redirects(_: crate::structs::Authorization) -> impl axum::response::IntoResponse {
    let links = match crate::URLS.get() {
        None => {
            return Err(crate::structs::AdminErrors::InternalError);
        }
        Some(links) => links,
    };
    let json_response = match serde_json::to_string(&crate::structs::List {
        links: links.clone(),
    }) {
        Ok(json) => json,
        Err(_) => {
            return Err(crate::structs::AdminErrors::InternalError);
        }
    };
    Ok(json_response)
}

pub async fn edit(
    axum::extract::Json(_payload): axum::extract::Json<crate::structs::Edit>,
) -> impl axum::response::IntoResponse {
}

pub async fn add(
    _: crate::structs::Authorization,
    axum::extract::Json(payload): axum::extract::Json<crate::structs::Add>,
) -> Result<(axum::http::StatusCode, &'static str), crate::structs::AdminErrors> {
    let db = match crate::DB.get() {
        None => return Err(crate::structs::AdminErrors::InternalError),
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
        return Err(crate::structs::AdminErrors::InternalError);
    }
    let links = match crate::URLS.get() {
        None => return Err(crate::structs::AdminErrors::InternalError),
        Some(links) => links,
    };
    links.insert(payload.link, payload.destination);

    Ok((axum::http::status::StatusCode::CREATED, "Link added!"))
}
