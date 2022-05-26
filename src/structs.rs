use axum::body::{boxed, Full};
use axum::http::status::StatusCode;
use axum::http::uri::InvalidUri;
use axum::http::HeaderValue;
use qr_code::bmp_monochrome::BmpError;
use qr_code::types::QrError;
use serde::Deserialize;
use std::borrow::Cow;
use std::sync::Arc;

use crate::TokenMap;

#[derive(Deserialize, Clone, Debug)]
pub struct Add {
    pub link: String,
    pub destination: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Edit {
    pub destination: String,
}

#[derive(Deserialize, Clone)]
pub struct Qr {
    pub destination: String,
}

#[derive(Debug)]
pub enum WebServerError {
    IncorrectAuth,
    BadRequest,
    NotFound,
    UrlConflict,
    UrlDisallowed,

    Db(sqlx::Error),
    InvalidUri(InvalidUri),
    Bmp(BmpError),
    Qr(QrError),

    InvalidRedirectUri,
    MissingHeaders,
    MissingExtensions,
}

impl From<sqlx::Error> for WebServerError {
    fn from(e: sqlx::Error) -> Self {
        Self::Db(e)
    }
}

impl From<axum::http::uri::InvalidUri> for WebServerError {
    fn from(e: axum::http::uri::InvalidUri) -> Self {
        Self::InvalidUri(e)
    }
}

impl From<BmpError> for WebServerError {
    fn from(e: BmpError) -> Self {
        Self::Bmp(e)
    }
}

impl From<QrError> for WebServerError {
    fn from(e: QrError) -> Self {
        Self::Qr(e)
    }
}

impl axum::response::IntoResponse for WebServerError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("handling error: {:#?}", self);
        let (error, status): (Cow<str>, StatusCode) = match self {
            WebServerError::IncorrectAuth => {
                ("Authentication failed".into(), StatusCode::UNAUTHORIZED)
            }
            WebServerError::BadRequest => (
                "Missing header or malformed json".into(),
                StatusCode::BAD_REQUEST,
            ),
            WebServerError::NotFound => ("Link not found".into(), StatusCode::NOT_FOUND),
            WebServerError::UrlConflict => (
                "Short URL conflicts with already-existing url, try editing instead".into(),
                StatusCode::CONFLICT,
            ),

            WebServerError::MissingHeaders => (
                "another extractor took headers".into(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            WebServerError::InvalidRedirectUri => (
                "database returned invalid header".into(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),

            WebServerError::UrlDisallowed => (
                "URL empty or used by the system".into(),
                StatusCode::CONFLICT,
            ),
            WebServerError::MissingExtensions => (
                "Missing internal request extension(s)".into(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            WebServerError::Db(e) => (
                format!("Database returned an error: {:?}", e).into(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            WebServerError::InvalidUri(e) => (
                format!("The redirect URI is invalid: {:?}", e).into(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            WebServerError::Bmp(e) => (
                format!("BMP conversion error: {:?}", e).into(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            WebServerError::Qr(e) => (
                format!("QR creation error: {:?}", e).into(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        };
        let body = json!({ "error": error }).to_string();
        axum::response::Response::builder()
            .header(
                axum::http::header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
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
        let provided_token = String::from_utf8(
            headers
                .get("Authorization")
                .ok_or(WebServerError::BadRequest)?
                .as_bytes()
                .into(),
        )
        .map_err(|_| WebServerError::BadRequest)?;
        let tokens = req
            .extensions()
            .ok_or(WebServerError::MissingExtensions)?
            .get::<Arc<TokenMap>>()
            .ok_or(WebServerError::MissingExtensions)?;
        if tokens.get(&provided_token).is_some() {
            Ok(Self)
        } else {
            Err(WebServerError::IncorrectAuth)
        }
    }
}
