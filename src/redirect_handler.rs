use crate::structs::Errors;
use axum::extract::Path;
use axum::response::Redirect;

pub async fn redirect(Path(path): Path<String>) -> Result<Redirect, Errors> {
    let destination_url = crate::URLS
        .get()
        .ok_or(Errors::UrlsNotFound)?
        .get(&path)
        .ok_or(Errors::NotFound)?
        .value();
    tracing::trace!("Path: {}, Destination: {}", path, destination_url);
    Ok(Redirect::permanent(destination_url.parse()?))
}
