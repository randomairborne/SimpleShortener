use crate::{admin, redirect_handler, users};
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
        .nest("/simpleshortener/", makeapi(&state))
        .fallback(get(move |req| redirect_handler::redirect(req, state)))
}
fn makeapi(state: &crate::State) -> Router {
    Router::new()
        .route(
            "/",
            get({
                let state = state.clone();
                move || admin::panel(state)
            }),
        )
        .route(
            "/api/edit/:id",
            patch({
                let state = state.clone();
                move |path, json, auth| admin::edit(path, json, auth, state)
            }),
        )
        .route(
            "/api/delete/:id",
            delete({
                let state = state.clone();
                move |path, auth| admin::delete(path, auth, state)
            }),
        )
        .route(
            "/api/add",
            put({
                let state = state.clone();
                move |json, auth| admin::add(json, auth, state)
            }),
        )
        .route(
            "/api/list",
            get({
                let state = state.clone();
                move |auth| admin::list(auth, state)
            }),
        )
        .route(
            "/api/qr",
            post({
                let state = state.clone();
                move |json, auth| admin::generate_qr(json, auth, state)
            }),
        )
        .route(
            "/api/token",
            post({
                let state = state.clone();
                move |headers| users::login(headers, state)
            }),
        )
        .route(
            "/api/token/invalidate/:token",
            post({
                let state = state.clone();
                move |path| users::invalidate(path, state)
            }),
        )
        .route(
            "/api/create",
            post({
                let state = state.clone();
                move |json| users::setup(json, state)
            }),
        )
        .route(
            "/static/panel.js",
            get(|| async {
                (
                    [("Content-Type", "text/html")],
                    include_str!("resources/panel.js"),
                )
            }),
        )
}
