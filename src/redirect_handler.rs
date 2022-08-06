use crate::error::WebServerError;
use crate::State;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;

#[allow(clippy::unused_async)]
pub async fn redirect(
    req: Request<axum::body::Body>,
    state: State,
) -> Result<impl IntoResponse, WebServerError> {
    let path = req.uri().to_string().to_lowercase();
    let destination_url = match state.urls.get(&path) {
        Some(dest) => dest,
        None => {
            return Ok((
                StatusCode::OK,
                [("Content-Type", "text/html".to_string())],
                include_str!("resources/404.html"),
            ))
        }
    };
    debug!("Path: {}, Destination: {}", path, *destination_url);
    Ok((
        StatusCode::PERMANENT_REDIRECT,
        [("Location", destination_url.to_string())],
        "",
    ))
}

// Checks if a specific config var exists or ADVERTISEEEEEE
#[allow(clippy::unused_async)]
pub async fn root(state: State) -> Result<impl IntoResponse, WebServerError> {
    debug!("Handling root request");
    let root = match state.urls.get("") {
        Some(url) => url.to_string(),
        None => "https://github.com/randomairborne/simpleshortener".to_string(),
    };
    Ok((StatusCode::PERMANENT_REDIRECT, [("Location", root)], ""))
}
