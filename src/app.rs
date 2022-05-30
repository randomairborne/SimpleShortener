use crate::{admin, files, redirect_handler, users, UrlMap};
use axum::extract::Extension;
use axum::routing::{delete, get, patch, post, put, Router};
use dashmap::DashMap;
use sqlx::PgPool;
use std::sync::Arc;

pub fn makeapp(db: Arc<PgPool>, urls: Arc<UrlMap>, is_init: crate::IsInit) -> Router {
    let authtokens: UrlMap = DashMap::new();
    Router::new()
        .route("/", get(redirect_handler::root))
        .route("/:path", get(redirect_handler::redirect))
        .route("/simpleshortener/api", get(files::doc))
        .route("/simpleshortener/api/", get(files::doc))
        .route("/simpleshortener/api/edit/:id", patch(admin::edit))
        .route("/simpleshortener/api/delete/:id", delete(admin::delete))
        .route("/simpleshortener/api/add", put(admin::add))
        .route("/simpleshortener/api/list", get(admin::list))
        .route("/simpleshortener/api/qr", post(admin::qr))
        .route("/simpleshortener/api/token", post(users::token))
        .route("/simpleshortener/api/create", post(users::setup))
        .route("/simpleshortener", get(files::panel))
        .route("/simpleshortener/", get(files::panel))
        .route("/simpleshortener/static/link.png", get(files::logo))
        .route("/favicon.ico", get(files::favicon))
        .layer(Extension(urls))
        .layer(Extension(db))
        .layer(Extension(authtokens))
        .layer(Extension(is_init))
}
