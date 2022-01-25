use axum::response::IntoResponse;

pub async fn admin() -> impl IntoResponse {
    let config = crate::CONFIG.get().expect("Json did not read correctly").clone();
}

pub async fn list_redirects() -> impl IntoResponse {

}

pub async fn edit() -> impl IntoResponse {
    sqlx::query!("INSERT INTO links VALUES ('nerd','https://randomairborne.dev')");
}

pub async fn add() -> impl IntoResponse {

}