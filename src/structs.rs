use axum::body::{boxed, Full};
use axum::http::status::StatusCode;
use axum::http::HeaderValue;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TlsConfig {
    pub certfile: String,
    pub keyfile: String,
    pub port: Option<u16>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub port: Option<u16>,
    pub root: Option<String>,
    pub database: Option<String>,
    pub users: std::collections::HashMap<String, String>,
    pub tls: Option<TlsConfig>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Add {
    pub link: String,
    pub destination: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Edit {
    pub destination: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct List {
    pub links: &'static dashmap::DashMap<String, String>,
}

#[derive(Debug)]
pub enum WebServerError {
    IncorrectAuth,
    BadRequest,
    NotFound,
    NotFoundJson,
    UrlConflict,
    UrlDisallowed,

    DbError(bincode::Error),
    InvalidUri(axum::http::uri::InvalidUri),

    UrlsNotFound,
    DisallowedNotFound,
    ConfigNotFound,
    InvalidRedirectUri,
    MissingHeaders,
}

impl From<bincode::Error> for WebServerError {
    fn from(e: bincode::Error) -> Self {
        Self::DbError(e)
    }
}

impl From<axum::http::uri::InvalidUri> for WebServerError {
    fn from(e: axum::http::uri::InvalidUri) -> Self {
        Self::InvalidUri(e)
    }
}

impl axum::response::IntoResponse for WebServerError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("handling error: {:#?}", self);
        let (body, status, content_type): (Cow<str>, StatusCode, &'static str) = match self {
            WebServerError::IncorrectAuth => (
                r#"{"error":"Authentication failed"}"#.into(),
                StatusCode::UNAUTHORIZED,
                "application/json",
            ),
            WebServerError::BadRequest => (
                r#"{"error":"Missing header or malformed json"}"#.into(),
                StatusCode::BAD_REQUEST,
                "application/json",
            ),
            WebServerError::NotFound => (
                include_str!("resources/404.html").into(),
                StatusCode::NOT_FOUND,
                "text/html",
            ),
            WebServerError::NotFoundJson => (
                r#"{"error":"Link not found"}"#.into(),
                StatusCode::NOT_FOUND,
                "application/json",
            ),
            WebServerError::UrlConflict => (
                r#"{"error":"Short URL conflicts with already-existing url, try editing instead"}"#
                    .into(),
                StatusCode::CONFLICT,
                "application/json",
            ),
            WebServerError::DbError(e) => (
                format!(r#"{{"error":"Database returned an error: {:?}"}}"#, e).into(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "application/json",
            ),
            WebServerError::InvalidUri(e) => (
                format!(r#"{{"error":"The redirect URI is invalid: {}"}}"#, e).into(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "application/json",
            ),
            WebServerError::UrlsNotFound => (
                r#"{"error":"Internal URL mapping list not found"}"#.into(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "application/json",
            ),
            WebServerError::DisallowedNotFound => (
                r#"{"error":"Internal disallowed URL list not found"}"#.into(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "application/json",
            ),
            WebServerError::ConfigNotFound => (
                r#"{"error":"Internal config not found"}"#.into(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "application/json",
            ),
            WebServerError::MissingHeaders => (
                r#"{"error":"another extractor took headers"}"#.into(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "application/json",
            ),
            WebServerError::InvalidRedirectUri => (
                r#"{"error":"database returned invalid header"}"#.into(),
                StatusCode::INTERNAL_SERVER_ERROR,
                "application/json",
            ),

            WebServerError::UrlDisallowed => (
                r#"{"error":"URL empty or used by the system"}"#.into(),
                StatusCode::CONFLICT,
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
impl<T: Send> axum::extract::FromRequest<T> for Authorization {
    type Rejection = WebServerError;
    async fn from_request(
        req: &mut axum::extract::RequestParts<T>,
    ) -> Result<Self, Self::Rejection> {
        let headers = req.headers().ok_or(WebServerError::MissingHeaders)?;

        let username = String::from_utf8(
            headers
                .get("username")
                .ok_or(WebServerError::BadRequest)?
                .as_bytes()
                .into(),
        )
        .map_err(|_| WebServerError::BadRequest)?;
        let password = String::from_utf8(
            headers
                .get("password")
                .ok_or(WebServerError::BadRequest)?
                .as_bytes()
                .into(),
        )
        .map_err(|_| WebServerError::BadRequest)?;
        let config = crate::CONFIG.get().ok_or(WebServerError::ConfigNotFound)?;
        let result = sha2::Sha256::digest(password)
            .into_iter()
            .map(|x| format!("{:02x}", x))
            .collect::<String>();
        let existing_hash = config.users.get(&username).map(String::as_str);
        tracing::debug!("Attempting to log in user {}", username);
        tracing::trace!(
            "supplied password hash is {}, correct password hash is {}",
            result,
            existing_hash.unwrap_or("(failed to get proper password hash)")
        );

        if existing_hash.map_or(false, |user| user == result) {
            Ok(Self)
        } else {
            Err(WebServerError::IncorrectAuth)
        }
    }
}
