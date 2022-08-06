#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod admin;
mod app;
mod error;
mod redirect_handler;
mod structs;
mod users;

use dashmap::{DashMap, DashSet};
use rand::Rng;
use sqlx::SqlitePool;
use std::{
    str::FromStr,
    sync::{atomic::AtomicBool, Arc},
};

#[macro_use]
extern crate sqlx;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate serde_json;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_env("LOG"))
        .init();
    let db_path =
        std::path::PathBuf::from_str(&std::env::var("DATABASE").expect("DATABASE not set!"))
            .expect("DATABASE should be a path");
    if !db_path.exists() {
        std::fs::File::create(&db_path).expect("Failed to create database");
    }
    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite://{}", db_path.display()))
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to write migrations!");
    let urls_mangled = query!("SELECT * FROM urls")
        .fetch_all(&db)
        .await
        .expect("Failed to select from URLs");
    let urls: DashMap<String, String> = DashMap::with_capacity(urls_mangled.capacity());
    for pair in urls_mangled {
        urls.insert(pair.link, pair.destination);
    }
    let is_init: bool = query!("SELECT username FROM accounts")
        .fetch_one(&db)
        .await
        .is_ok();
    let state = Arc::new(AppState {
        db,
        tokens: Arc::new(DashSet::new()),
        urls: Arc::new(urls),
        is_init: Arc::new(AtomicBool::from(is_init)),
    });
    println!("[SimpleShortener] Listening on http://127.0.0.1:8080");
    axum::Server::bind(&([0, 0, 0, 0], 8080).into())
        .serve(app::makeapp(state.clone()).into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
            println!("Cleaning up...");
            state.db.close().await;
            println!("Goodbye!");
        })
        .await
        .expect("Failed to await main HTTP process");
}

#[must_use]
pub fn randstr() -> String {
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz."
        .chars()
        .collect();
    let mut result = String::with_capacity(64);
    let mut rng = rand::thread_rng();
    for _ in 0..64 {
        result.push(*chars.get(rng.gen_range(0..chars.len())).unwrap_or(&'.'));
    }
    result
}

pub type State = Arc<AppState>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub urls: Arc<DashMap<String, String>>,
    pub tokens: Arc<DashSet<String>>,
    pub is_init: Arc<AtomicBool>,
}
