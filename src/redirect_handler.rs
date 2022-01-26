use std::str::FromStr;

pub async fn redirect(
    axum::extract::Path(path): axum::extract::Path<String>,
) -> impl axum::response::IntoResponse {
    let urls = match crate::URLS.get() {
        None => return Err(crate::structs::Errors::InternalError),
        Some(config) => config,
    };
    let destination_url = match urls.get(path.as_str()) {
        None => return Err(crate::structs::Errors::NotFound),
        Some(entry) => entry.key(),
    };
    tracing::trace!("Path: {}, Destination: {}", path, destination_url);
    let clean_destination_url = match axum::http::Uri::from_str(destination_url) {
        Ok(url) => url,
        Err(_) => return Err(crate::structs::Errors::InternalError),
    };
    Ok(axum::response::Redirect::permanent(clean_destination_url))
}
