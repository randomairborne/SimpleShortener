use crate::structs::WebServerError;
use axum::extract::Path;
use axum::response::Redirect;

pub async fn redirect(Path(path): Path<String>) -> Result<Redirect, WebServerError> {
    let destination_url = crate::URLS
        .get()
        .ok_or(WebServerError::UrlsNotFound)?
        .get(&path)
        .ok_or(WebServerError::NotFound)?
        .value();
    tracing::trace!("Path: {}, Destination: {}", path, destination_url);
    Ok(Redirect::permanent(destination_url.parse()?))
}
