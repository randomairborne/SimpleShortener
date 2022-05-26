#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod admin;
mod app;
mod files;
mod redirect_handler;
mod structs;
// mod tests;
use std::sync::Arc;

use dashmap::DashMap;

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
    let db = sqlx::PgPool::connect(&std::env::var("DATABASE_URI").expect("No database URI"))
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
    let urls: UrlMap = DashMap::new();
    for pair in urls_mangled {
        urls.insert(pair.link, pair.destination);
    }
    info!("listening on http://broadcasthost:8080");
    axum_server::bind(([0, 0, 0, 0], 8080).into())
        .serve(app::makeapp(Arc::new(db), Arc::new(urls)).into_make_service())
        .await
        .expect("Failed to await main HTTP process");
}

pub type UrlMap = DashMap<String, String>;
pub type TokenMap = DashMap<String, String>;
