#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
mod admin;
mod app;
mod error;
mod files;
mod redirect_handler;
mod structs;
mod users;
use std::sync::{atomic::AtomicBool, Arc};

use dashmap::DashMap;
use rand::Rng;

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
    let db = sqlx::PgPool::connect(
        &std::env::var("DATABASE_URI")
            .unwrap_or_else(|_| std::env::var("DATABASE_URL").expect("No database URI")),
    )
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
    let is_init: bool = query!("SELECT username FROM accounts")
        .fetch_one(&db)
        .await
        .is_ok();
    println!("[SimpleShortener] Listening on http://broadcasthost:8080");
    axum::Server::bind(&([0, 0, 0, 0], 8080).into())
        .serve(
            app::makeapp(
                Arc::new(db),
                Arc::new(urls),
                Arc::new(AtomicBool::from(is_init)),
            )
            .into_make_service(),
        )
        .await
        .expect("Failed to await main HTTP process");
}

pub type UrlMap = DashMap<String, String>;
pub type TokenMap = DashMap<String, String>;
pub type IsInit = Arc<AtomicBool>;
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
