use axum::extract::Path;
use axum::headers::authorization::Bearer;
use axum::Json;
use serde_json::Value;
use sha2::Digest;

use crate::error::WebServerError;
use crate::structs::LogIn;
use std::sync::atomic::Ordering;

/// This functiion checks the state to see if the Bearer authtoken is in the token DB.
pub fn authenticate(auth: &Bearer, state: &crate::State) -> Result<(), WebServerError> {
    trace!("Tokens: {:?}", &state.tokens);
    if state.tokens.get(auth.token()).is_some() {
        Ok(())
    } else {
        Err(WebServerError::IncorrectAuth)
    }
}

#[allow(clippy::unused_async)]
pub async fn token(
    Json(LogIn { username, password }): Json<LogIn>,
    state: crate::State,
) -> Result<Json<Value>, WebServerError> {
    let correct_with_salt = match query!(
        "SELECT password FROM accounts WHERE username = $1",
        &username
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(pw) => pw,
        Err(sqlx::Error::RowNotFound) => return Err(WebServerError::InvalidUsernameOrPassword),
        Err(e) => return Err(WebServerError::Db(e)),
    };
    let correct_split = correct_with_salt.password.split('|').collect::<Vec<&str>>();
    let correct_hash = correct_split.get(1).ok_or(WebServerError::NoSalt)?;
    let salt = correct_split.get(0).ok_or(WebServerError::NoSalt)?;
    let provided_password_hash = sha2::Sha512::digest(&format!("{}|{}", salt, password))
        .into_iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();
    if *correct_hash != provided_password_hash {
        return Err(WebServerError::InvalidUsernameOrPassword);
    };
    let token = crate::randstr();
    state.tokens.insert(token.clone());
    Ok(Json(json!({ "token": token })))
}

pub async fn setup(
    Json(LogIn { username, password }): Json<LogIn>,
    state: crate::State,
) -> Result<Json<Value>, WebServerError> {
    let salt = crate::randstr();
    let password_hash = sha2::Sha512::digest(&format!("{}|{}", &salt, password))
        .into_iter()
        .map(|x| format!("{:02x}", x))
        .collect::<String>();

    debug!(
        "Creating new user with username {} and password hash `{}|{}`",
        &username, &salt, &password_hash
    );
    query!(
        "INSERT INTO accounts VALUES ($1, $2)",
        username,
        &format!("{}|{}", &salt, password_hash)
    )
    .execute(&state.db)
    .await?;
    state.is_init.store(true, Ordering::Relaxed);
    Ok(Json(json!({"message":"Account added!"})))
}

#[allow(clippy::unused_async)]
pub async fn invalidate(Path(path): Path<String>, state: crate::State) -> Json<Value> {
    state.tokens.remove(&path);
    Json(json!({"message": "Token invalidated!"}))
}
