mod admin;
mod files;
mod redirect_handler;
mod structs;

use axum::{routing::get, routing::post, Router};
use std::net::SocketAddr;

// OnceCell init
static CONFIG: once_cell::sync::OnceCell<structs::Config> = once_cell::sync::OnceCell::new();
static URLS: once_cell::sync::OnceCell<dashmap::DashMap<String, String>> =
    once_cell::sync::OnceCell::new();
static DB: once_cell::sync::OnceCell<sqlx::Pool<sqlx::Postgres>> = once_cell::sync::OnceCell::new();
static DISALLOWED_SHORTENINGS: once_cell::sync::OnceCell<std::collections::HashSet<String>> =
    once_cell::sync::OnceCell::new();

#[tokio::main]
async fn main() {
    DISALLOWED_SHORTENINGS
        .set(std::collections::HashSet::from([
            String::from(""),
            String::from("simpleshortener_admin_api"),
            String::from("simpleshortener_static_files"),
            String::from("favicon.ico"),
            String::from("simpleshortener_admin_panel"),
        ]))
        .expect("Failed to set disallowed shortenings");
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
        Err(err) => {
            eprintln!("Failed to read config: {:#?}", err);
            std::process::exit(1);
        }
    };
    // get config
    tracing::log::info!("Parsing config {}", &args[1]);
    let mut config = match toml::from_str::<structs::Config>(&config_string) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Failed to parse config: {:#?}", err);
            std::process::exit(2);
        }
    };
    // This looks scary, but it simply looks through the config for the user's hashed passwords and lowercases them.
    config
        .users
        .iter_mut()
        .map(|(_, x)| *x = x.to_lowercase())
        .for_each(drop);
    CONFIG
        .set(config.clone())
        .expect("Failed to write to config OnceCell");

    let pool = match sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(config.database.as_str())
        .await
    {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Failed to connect to database: {:#?}", err);
            std::process::exit(3);
        }
    };
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    let urls_vec = sqlx::query!("SELECT * FROM links")
        .fetch_all(&pool)
        .await
        .expect("Failed to select links from database");
    let urls = dashmap::DashMap::with_capacity(urls_vec.len());
    for url in urls_vec {
        urls.insert(url.link, url.destination);
    }
    URLS.set(urls).expect("Failed to set URLS OnceCell");
    DB.set(pool).expect("Failed to set database OnceCell");
    // build our application with a route
    let app = Router::new()
        .route("/", get(files::root))
        .route("/:path", get(redirect_handler::redirect))
        .route("/simpleshortener_admin_api", get(files::doc))
        .route("/simpleshortener_admin_api/", get(files::doc))
        .route("/simpleshortener_admin_api/edit", post(admin::edit))
        .route("/simpleshortener_admin_api/delete", post(admin::delete))
        .route("/simpleshortener_admin_api/add", post(admin::add))
        .route("/simpleshortener_admin_api/list", get(admin::list))
        .route("/simpleshortener_admin_panel", get(files::panelhtml))
        .route("/simpleshortener_admin_panel/", get(files::panelhtml))
        .route("/simpleshortener_admin_panel/panel.js", get(files::paneljs))
        .route("/simpleshortener_static_files/link.png", get(files::logo))
        .route("/simpleshortener_static_files/font.woff", get(files::fontw))
        .route("/simpleshortener_static_files/font.woff2", get(files::fontw2))
        .route("/favicon.ico", get(files::favicon));

    // Checks for a PORT environment variable
    let port = match std::env::var("PORT") {
        Ok(port_string) => match port_string.parse::<u16>() {
            Ok(port) => port,
            Err(_) => config.port,
        },
        Err(_) => config.port,
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
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
