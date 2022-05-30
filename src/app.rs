use std::sync::Arc;

use crate::{admin, files, redirect_handler, users};
use axum::{
    routing::{delete, get, patch, post, put, Router},
    Extension,
};
use dashmap::DashMap;

pub fn makeapp(tokens: Arc<DashMap<String, String>>, state: crate::State) -> Router {
    Router::new()
        .route("/", get(move || redirect_handler::root(state.clone())))
        .route(
            "/:path",
            get({Arc::clone(&state);move |path| redirect_handler::redirect(path, state)}),
        )
        .route("/simpleshortener/api", get(files::doc))
        .route("/simpleshortener/api/", get(files::doc))
        .route(
            "/simpleshortener/api/edit/:id",
            patch(move |auth, path, json| admin::edit(auth, path, json, state)),
        )
        .route(
            "/simpleshortener/api/delete/:id",
            delete(move |auth, path| admin::delete(auth, path, state)),
        )
        .route(
            "/simpleshortener/api/add",
            put(move |auth, json| admin::add(auth, json, state)),
        )
        .route(
            "/simpleshortener/api/list",
            get(move |auth| admin::list(auth, state)),
        )
        .route("/simpleshortener/api/qr", post(move |json| admin::generate_qr(json)))
        .route(
            "/simpleshortener/api/token",
            post(move |headers| users::token(headers, state)),
        )
        .route(
            "/simpleshortener/api/create",
            post(move |json| users::setup(json, state)),
        )
        .route("/simpleshortener", get(move || files::panel(state.is_init)))
        .route(
            "/simpleshortener/",
            get(move || files::panel(state.is_init)),
        )
        .route("/simpleshortener/static/link.png", get(files::logo))
        .route("/favicon.ico", get(files::favicon))
        .layer(Extension(tokens))
}
