use axum::body::{boxed, Full};
use axum::http::status::StatusCode;
use axum::http::uri::InvalidUri;
use axum::http::HeaderValue;
use qr_code::bmp_monochrome::BmpError;
use qr_code::types::QrError;
use std::borrow::Cow;
use std::string::FromUtf8Error;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum WebServerError {
    // Errors a user could cause
    IncorrectAuth,
    NotFound,
    UrlConflict,
    UrlDisallowed,
    InvalidUsernameOrPassword,

    // Internal server errors
    Db(sqlx::Error),
    InvalidUri(InvalidUri),
    Bmp(BmpError),
    Qr(QrError),
    FromUtf8(FromUtf8Error),

    InvalidRedirectUri,
    NoSalt,
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

impl From<FromUtf8Error> for WebServerError {
    fn from(e: FromUtf8Error) -> Self {
        Self::FromUtf8(e)
    }
}

impl From<QrError> for WebServerError {
    fn from(e: QrError) -> Self {
        Self::Qr(e)
    }
}

impl axum::response::IntoResponse for WebServerError {
    fn into_response(self) -> axum::response::Response {
        let (error, status): (Cow<str>, StatusCode) = match self {
            Self::NotFound => ("Link not found".into(), StatusCode::NOT_FOUND),
            Self::IncorrectAuth => ("Authentication failed".into(), StatusCode::UNAUTHORIZED),
            Self::InvalidUsernameOrPassword => (
                "Username or password incorrect!".into(),
                StatusCode::UNAUTHORIZED,
            ),
            Self::UrlConflict => (
                "Short URL conflicts with already-existing url, try editing instead".into(),
                StatusCode::CONFLICT,
            ),
            Self::InvalidRedirectUri => (
                "Database returned invalid header".into(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Self::UrlDisallowed => (
                "URL empty or used by the system".into(),
                StatusCode::CONFLICT,
            ),
            Self::NoSalt => (
                "Internal salt was not found".into(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Self::Db(e) => (
                format!("Database returned an error: {:?}", e).into(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Self::FromUtf8(e) => (
                format!("Error converting to UTF-8: {:?}", e).into(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Self::InvalidUri(e) => (
                format!("The redirect URI is invalid: {:?}", e).into(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Self::Bmp(e) => (
                format!("BMP conversion error: {:?}", e).into(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Self::Qr(e) => (
                format!("QR creation error: {:?}", e).into(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        };
        if status == StatusCode::INTERNAL_SERVER_ERROR {
            error!("Error processing request: {}", error);
        } else if status == StatusCode::NOT_FOUND || status == StatusCode::CONFLICT {
            debug!("User error: {}", error);
        } else {
            warn!("Possible problem: {}", error);
        }
        axum::response::Response::builder()
            .header(
                axum::http::header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            )
            .status(status)
            .body(boxed(Full::from(json!({ "error": error }).to_string())))
            .unwrap()
    }
}
