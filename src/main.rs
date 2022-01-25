mod admin;
mod structs;
mod redirect_handlers;


use axum::{routing::get, Router};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::net::SocketAddr;

// OnceCell init
static CONFIG: OnceCell<structs::Config> = OnceCell::new();
static URLS: OnceCell<HashMap<String, String>> = OnceCell::new();
static DB: OnceCell<sqlx::Pool<sqlx::Postgres>> = OnceCell::new();

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_env("LOG"))
        .init();
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        args.push(String::from("./config.toml"))
    };
    tracing::log::info!("Reading config {}", &args[1]);
    let config_string = match std::fs::read_to_string(&args[1]) {
        Ok(config_string) => config_string,
        Err(e) => panic!("{}", e),
    };
    // get config
    tracing::log::info!("Parsing config {}", &args[1]);
    let config = match toml::from_str::<structs::Config>(config_string.as_str()) {
        Ok(config) => config,
        Err(e) => panic!("{}", e),
    };
    CONFIG.set(config.clone()).unwrap();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(config.database.as_str()).await?;
    sqlx::query("CREATE TABLE IF NOT EXISTS links (link text NOT NULL, destination text NOT NULL)").execute(&pool).await?;
    sqlx::query("SELECT * FROM links");
    DB.set(pool).unwrap();
    // build our application with a route
    let app = Router::new()
        .route("/", get(redirect_handlers::root))
        .route("/:path", get(redirect_handlers::redirect))
        .route("/admin_api", get(admin::admin).post(admin::admin))
        .route("/admin_api/list", get(admin::list_redirects))
        .route("/admin_api/edit/:id", get(admin::edit).post(admin::edit));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    tracing::log::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen for ctrl+c");
        })
        .await
        .unwrap();
}
