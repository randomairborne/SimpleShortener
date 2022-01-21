use axum::http::header::{HeaderMap, HeaderValue};
use axum::http::StatusCode;
use axum::{extract::Path, response::IntoResponse, response::Response, routing::get, Router};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;

// OnceCell init
static INSTANCE: OnceCell<Config> = OnceCell::new();

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // get config
    let config =
        serde_json::from_str::<Config>(std::fs::read_to_string("config.json").unwrap().as_str())
            .unwrap();
    INSTANCE.set(config.clone()).unwrap();
    for (_, destination_url) in config.urls.iter() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Location",
            HeaderValue::try_from(destination_url)
                .expect("There was an error parsing your config file"),
        );
    }
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/:path", get(redirect));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> impl IntoResponse {
    let config = INSTANCE.get().expect("Json did not read correctly").clone();
    Response::builder()
        .status(StatusCode::PERMANENT_REDIRECT)
        .header("Location", config.default)
        .body(axum::body::Body::empty())
        .unwrap()
}

async fn redirect(Path(path): Path<String>) -> (StatusCode, HeaderMap, &'static str) {
    let config = INSTANCE.get().expect("Json did not read correctly").clone();
    for (shortening, destination_url) in config.urls.iter() {
        println!("Shortening: {}, Path: {}", shortening, path);
        if shortening == &path {
            let mut headers = HeaderMap::new();
            headers.insert("Location", HeaderValue::try_from(destination_url).unwrap());
            return (StatusCode::PERMANENT_REDIRECT, headers, "Redirecting...");
        }
    }
    (
        StatusCode::NOT_FOUND,
        HeaderMap::default(),
        "404 redirect not found, ask the sender if they made a typo",
    )
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Config {
    port: u16,
    default: String,
    urls: HashMap<String, String>,
}
