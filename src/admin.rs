use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn list_redirects<'a>() -> (StatusCode, std::borrow::Cow<'a, str>) {
    let links = match crate::URLS.get() {
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was a serious internal error with the OnceCell".into(),
            )
        }
        Some(links) => links,
    };
    let json_response = match serde_json::to_string(&crate::structs::List {
        links: links.clone(),
    }) {
        Ok(json) => json,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was a serious internal error".into(),
            )
        }
    };
    (StatusCode::OK, json_response.into())
}

pub async fn edit(
    axum::extract::Json(payload): axum::extract::Json<crate::structs::Edit>,
) -> impl IntoResponse {
}

pub async fn add<'a>(
    axum::extract::Json(payload): axum::extract::Json<crate::structs::Add>,
) -> (StatusCode, std::borrow::Cow<'a, str>) {
    let db = match crate::DB.get() {
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was a serious internal error with the OnceCell".into(),
            )
        }
        Some(db) => db,
    };
    match sqlx::query!(
        "INSERT INTO links VALUES ($1,$2)",
        payload.link,
        payload.destination
    )
    .execute(db)
    .await
    {
        Ok(_) => {}
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert to database: ".into(),
            )
        }
    }
    (StatusCode::CREATED, "Added successfully!".into())
}
