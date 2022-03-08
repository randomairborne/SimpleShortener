mod admin;
mod files;
mod redirect_handler;
mod structs;
mod utils;

use axum::{routing::delete, routing::get, routing::patch, routing::put, Router};
use once_cell::sync::OnceCell;
use std::net::SocketAddr;

/// Configuration oncecell, holds the Config struct and can easily be pulled from
static CONFIG: OnceCell<structs::Config> = OnceCell::new();
/// URL dashmap. This can be mutated, be careful not to do so
static URLS: OnceCell<dashmap::DashMap<String, String>> = OnceCell::new();
/// database connection.
static DB: OnceCell<sqlx::Pool<sqlx::Postgres>> = OnceCell::new();
/// shortenings that are not allowed
static DISALLOWED_SHORTENINGS: OnceCell<std::collections::HashSet<String>> = OnceCell::new();

#[tokio::main]
async fn main() {
    DISALLOWED_SHORTENINGS
        .set(std::collections::HashSet::from([
            String::from(""),
            String::from("favicon.ico"),
            String::from("simpleshortener"),
        ]))
        .expect("Failed to set disallowed shortenings");
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_env("LOG"))
        .init();

    let cfg_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("./config.toml"));

    tracing::log::info!("Reading config {}", &cfg_path);
    let config_string = std::fs::read_to_string(&cfg_path).unwrap_or_else(|err| {
        eprintln!("Failed to read config: {:#?}", err);
        std::process::exit(1);
    });
    // get config
    tracing::log::info!("Parsing config {}", &cfg_path);
    let mut config = toml::from_str::<structs::Config>(&config_string).unwrap_or_else(|err| {
        eprintln!("Failed to parse config: {:#?}", err);
        std::process::exit(2);
    });
    // This looks scary, but it simply looks through the config for the user's hashed passwords and lowercases them.
    config
        .users
        // get mutable iterator over items
        .iter_mut()
        // for every item, update the stored string to be lowercase
        .map(|(_, x)| *x = x.to_lowercase())
        // consume the iterator by dropping each item in it
        .for_each(drop);

    CONFIG
        .set(config.clone())
        .expect("Failed to write to config OnceCell");

    // Checks for a PORT environment variable
    let database_uri = std::env::var("DATABASE_URI")
        .unwrap_or_else(|_| config.clone().database.expect("Database URI not set!"));
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(database_uri.as_str())
        .await
        .expect("Failed to connect to database");
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
        urls.insert(url.link.to_lowercase(), url.destination);
    }
    URLS.set(urls).expect("Failed to set URLS OnceCell");
    DB.set(pool).expect("Failed to set database OnceCell");
    // build our application with a route
    let app = Router::new()
        .route("/", get(redirect_handler::root))
        .route("/:path", get(redirect_handler::redirect))
        .route("/simpleshortener/api", get(files::doc))
        .route("/simpleshortener/api/", get(files::doc))
        .route("/simpleshortener/api/edit/:id", patch(admin::edit))
        .route("/simpleshortener/api/delete/:id", delete(admin::delete))
        .route("/simpleshortener/api/add", put(admin::add))
        .route("/simpleshortener/api/list", get(admin::list))
        .route("/simpleshortener", get(files::panelhtml))
        .route("/simpleshortener/", get(files::panelhtml))
        .route("/simpleshortener/static/link.png", get(files::logo))
        .route("/simpleshortener/static/font.woff", get(files::font))
        .route("/simpleshortener/static/font.woff2", get(files::font2))
        .route("/favicon.ico", get(files::favicon));

    // Checks for a PORT environment variable
    let port = match std::env::var("PORT").map(|x| x.parse::<u16>()) {
        Ok(Ok(port)) => port,
        Err(_) => config.port.expect("Port not set!"),
        Ok(Err(e)) => {
            eprintln!("port environment variable invalid: {:#?}", e);
            std::process::exit(3);
        }
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::log::info!("listening on {}", addr);
    if config.tls.is_some() {
        let key = utils::read_file_to_bytes(&config.clone().tls.unwrap().keyfile);
        let cert = utils::read_file_to_bytes(&config.clone().tls.unwrap().certfile);
        let tls_app = app.clone();
        let tls_port = match std::env::var("TLS_PORT").map(|x| x.parse::<u16>()) {
            Ok(Ok(port)) => port,
            Err(_) => config.clone().tls.unwrap().port,
            Ok(Err(e)) => {
                eprintln!("port environment variable invalid: {:#?}", e);
                std::process::exit(4);
            }
        };
        let server_tls = tokio::spawn(async move {
            axum_server::bind_rustls(
                SocketAddr::from(([127, 0, 0, 1], tls_port)),
                axum_server::tls_rustls::RustlsConfig::from_pem(cert, key)
                    .await
                    .expect("Bad TLS pemfiles"),
            )
            .serve(tls_app.into_make_service())
            .await
            .expect("Failed to bind to address, is something else using the port?");
        });
        server_tls.await.expect("Failed to await HTTPS process");
    }
    let server_http = tokio::spawn(async move {
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(async {
                tokio::signal::ctrl_c()
                    .await
                    .expect("failed to listen for ctrl+c");
            })
            .await
            .expect("Failed to bind to address, is something else using the port?");
    });
    server_http.await.expect("Failed to await main HTTP process");
}
