use crate::{admin, files, redirect_handler, users};
use axum::routing::{delete, get, patch, post, put, Router};

pub fn makeapp(state: crate::State) -> Router {
    Router::new()
        .route(
            "/",
            get({
                let state = state.clone();
                move || redirect_handler::root(state)
            }),
        )
        .route(
            "/:path",
            get({
                let state = state.clone();
                move |path| redirect_handler::redirect(path, state)
            }),
        )
        .route("/simpleshortener/api", get(files::doc))
        .route("/simpleshortener/api/", get(files::doc))
        .route(
            "/simpleshortener/api/edit/:id",
            patch({
                let state = state.clone();
                move |path, json, auth| admin::edit(path, json, auth, state)
            }),
        )
        .route(
            "/simpleshortener/api/delete/:id",
            delete({
                let state = state.clone();
                move |path, auth| admin::delete(path, auth, state)
            }),
        )
        .route(
            "/simpleshortener/api/add",
            put({
                let state = state.clone();
                move |json, auth| admin::add(json, auth, state)
            }),
        )
        .route(
            "/simpleshortener/api/list",
            get({
                let state = state.clone();
                move |auth| admin::list(auth, state)
            }),
        )
        .route(
            "/simpleshortener/api/qr",
            post({
                let state = state.clone();
                move |json, auth| admin::generate_qr(json, auth, state)
            }),
        )
        .route(
            "/simpleshortener/api/token",
            post({
                let state = state.clone();
                move |headers| users::token(headers, state)
            }),
        )
        .route(
            "/simpleshortener/api/create",
            post({
                let state = state.clone();
                move |json| users::setup(json, state)
            }),
        )
        .route(
            "/simpleshortener",
            get({
                let state = state.clone();
                move || files::panel(state)
            }),
        )
        .route("/simpleshortener/", get(move || files::panel(state)))
        .route("/simpleshortener/static/link.png", get(files::logo))
        .route("/favicon.ico", get(files::favicon))
}
