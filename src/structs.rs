use axum::body::{boxed, Full};
use axum::http::status::StatusCode;
use axum::http::HeaderValue;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub database: String,
    pub users: std::collections::HashMap<String, String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Add {
    pub link: String,
    pub destination: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Edit {
    pub link: String,
    pub destination: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Delete {
    pub link: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct List {
    pub links: dashmap::DashMap<String, String>,
}

#[derive(Debug)]
pub enum Errors {
    IncorrectAuth,
    BadRequest,
    NotFound,
    NotFoundJson,
    UrlConflict,

    DbError(sqlx::Error),

    DbNotFound,
    UrlsNotFound,
    DisallowedNotFound,
    ConfigNotFound,

    MissingHeaders,
}

impl From<sqlx::Error> for Errors {
    fn from(e: sqlx::Error) -> Self {
        Self::DbError(e)
    }
}

impl axum::response::IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        let (body, status, content_type): (Cow<str>, StatusCode, &'static str) = match self {
            Errors::IncorrectAuth => (
                r#"{"error":"Authentication failed"}"#.into(),
                StatusCode::UNAUTHORIZED,
                "application/json",
            ),
            Errors::BadRequest => (
                r#"{"error":"Missing header or malformed json"}"#.into(),
                StatusCode::BAD_REQUEST,
                "application/json",
            ),
            Errors::NotFound => (
                include_str!("resources/404.html").into(),
                StatusCode::NOT_FOUND,
                "text/html",
            ),
            Errors::NotFoundJson => (
                r#"{"error":"Link not found"}"#.into(),
                StatusCode::NOT_FOUND,
                "application/json",
            ),
            Errors::UrlConflict => (
                r#"{"error":"Short URL conflicts with system URL, already-existing url, or is empty"}"#.into(),
                StatusCode::CONFLICT,
                "application/json",
                ),
            Errors::DbError(e) => (
                format!(r#"{{"error":"Database returned an error: {:?}"}}"#, e).into(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "application/json",
            ),
            Errors::DbNotFound => (
                r#"{"error":"Database pool not found"}"#.into(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "application/json",
            ),
            Errors::UrlsNotFound => (
                r#"{"error":"Internal URL mapping list not found"}"#.into(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "application/json",
            ),
            Errors::DisallowedNotFound => (
                r#"{"error":"Internal disallowed URL list not found"}"#.into(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "application/json",
            ),
            Errors::ConfigNotFound => (
                r#"{"error":"Internal config not found"}"#.into(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "application/json",
            ),
            Errors::MissingHeaders => (
                r#"{"error":"another extractor took headers"}"#.into(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "application/json",
            ),
        };

        axum::response::Response::builder()
            .header(
                axum::http::header::CONTENT_TYPE,
                HeaderValue::from_static(content_type),
            )
            .status(status)
            .body(boxed(Full::from(body)))
            .unwrap()
    }
}

pub struct Authorization;

#[async_trait::async_trait]
impl<T> axum::extract::FromRequest<T> for Authorization {
    type Rejection = Errors;
    async fn from_request(
        req: &mut axum::extract::RequestParts<T>,
    ) -> Result<Self, Self::Rejection> {
        let headers = req.headers().ok_or_else(|| Errors::InternalError)?;

        let auth_username = headers.get("username").ok_or_else(|| Errors::BadRequest)?;
        let auth_password = headers.get("password").ok_or_else(|| Errors::BadRequest)?;
        let username =
            String::from_utf8(auth_username.as_bytes().into()).map_err(|_| Errors::BadRequest)?;
        let password =
            String::from_utf8(auth_password.as_bytes().into()).map_err(|_| Errors::BadRequest)?;
        let config = match crate::CONFIG.get() {
            None => return Err(Errors::C),
            Some(config) => config,
        };
        let result = sha2::Sha256::digest(password)
            .into_iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>();
        tracing::trace!(
            "Attempting to log in user {}, supplied password hash is {}, correct password hash is {}",
            username,
            result,
            config
            .users
            .get(&username)
            .unwrap_or(&"(failed to get proper password hash)".to_string())

        );
        if config
            .users
            .get(&username)
            .map_or(false, |user| user == &result)
        {
            Ok(Self)
        } else {
            Err(Errors::IncorrectAuth)
        }
    }
}
