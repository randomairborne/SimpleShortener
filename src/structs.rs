use serde::{Deserialize, Serialize};
use sha2::Digest;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub database: String,
    pub users: std::collections::HashMap<String, String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Edit {
    pub port: u16,
    pub database: String,
    pub users: std::collections::HashMap<String, String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct List {
    pub links: dashmap::DashMap<String, String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Add {
    pub link: String,
    pub destination: String,
}

#[derive(Serialize, Clone, Debug)]
pub enum AdminErrors {
    IncorrectAuth,
    InternalError,
    BadRequest,
}

impl axum::response::IntoResponse for AdminErrors {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            AdminErrors::IncorrectAuth => axum::body::boxed(axum::body::Full::from(
                r#"{"error":"Authentication failed"}"#,
            )),
            AdminErrors::InternalError => axum::body::boxed(axum::body::Full::from(
                r#"{"error":"There was a serious internal error"}"#,
            )),
            AdminErrors::BadRequest => axum::body::boxed(axum::body::Full::from(
                r#"{"error":"Missing header or malformed json"}"#,
            )),
        };
        let status = match self {
            AdminErrors::IncorrectAuth => axum::http::status::StatusCode::UNAUTHORIZED,
            AdminErrors::InternalError => axum::http::status::StatusCode::INTERNAL_SERVER_ERROR,
            AdminErrors::BadRequest => axum::http::status::StatusCode::BAD_REQUEST,
        };
        axum::response::Response::builder()
            .header(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static("application/json"),
            )
            .status(status)
            .body(body)
            .unwrap()
    }
}

pub struct Authorization;

#[async_trait::async_trait]
impl axum::extract::FromRequest<axum::body::Body> for Authorization {
    type Rejection = AdminErrors;
    async fn from_request(
        req: &mut axum::extract::RequestParts<axum::body::Body>,
    ) -> Result<Self, Self::Rejection> {
        let headers = req.headers().ok_or_else(|| AdminErrors::InternalError)?;

        let auth_username = headers
            .get("username")
            .ok_or_else(|| AdminErrors::BadRequest)?;
        let auth_password = headers
            .get("password")
            .ok_or_else(|| AdminErrors::BadRequest)?;
        let username = String::from_utf8(auth_username.as_bytes().into())
            .map_err(|_| AdminErrors::BadRequest)?;
        let password = String::from_utf8(auth_password.as_bytes().into())
            .map_err(|_| AdminErrors::BadRequest)?;
        let config = match crate::CONFIG.get() {
            None => return Err(AdminErrors::InternalError),
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
            Err(AdminErrors::IncorrectAuth)
        }
    }
}
