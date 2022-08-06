use axum::extract::Path;
use axum::headers::authorization::Bearer;
use axum::Json;
use serde_json::Value;
use sha2::Digest;

use crate::error::WebServerError;
use crate::structs::LogIn;
use crate::State;
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

pub async fn login(
    Json(LogIn { username, password }): Json<LogIn>,
    state: State,
) -> Result<Json<Value>, WebServerError> {
    let correct_with_salt = match query!(
        "SELECT password FROM accounts WHERE username = $1",
        username
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
    Ok(Json(json!({ "token": token_gen(&state) })))
}

fn token_gen(state: &State) -> String {
    let token = crate::randstr();
    state.tokens.insert(token.clone());
    token
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
    let salty = format!("{}|{}", &salt, password_hash);
    query!("INSERT INTO accounts VALUES (?, ?)", username, salty)
        .execute(&state.db)
        .await?;
    state.is_init.store(true, Ordering::Relaxed);
    Ok(Json(
        json!({"message":"Account added!", "token": token_gen(&state)}),
    ))
}

#[allow(clippy::unused_async)]
pub async fn invalidate(Path(path): Path<String>, state: crate::State) -> Json<Value> {
    state.tokens.remove(&path);
    Json(json!({"message": "Token invalidated!"}))
}
