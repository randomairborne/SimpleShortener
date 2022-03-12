mod admin;
mod db;
mod files;
mod redirect_handler;
mod structs;
mod utils;

use crate::db::init_db_storage;
use once_cell::sync::OnceCell;
use std::net::SocketAddr;

/// Configuration oncecell, holds the Config struct and can easily be pulled from
static CONFIG: OnceCell<structs::Config> = OnceCell::new();
/// URL dashmap. This can be mutated, be careful not to do so
static URLS: OnceCell<dashmap::DashMap<String, String>> = OnceCell::new();
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
    let database_path = config
        .database
        .clone()
        .or_else(|| std::env::var("DATABASE_PATH").ok())
        .expect("Database URI not set!");
    let urls = utils::read_bincode(&database_path);
    URLS.set(urls).expect("Failed to set URLS OnceCell");
    init_db_storage();
    let app = utils::build_app();
    if config.socket.is_none() {
        // Checks for a PORT environment variable
        let port = utils::get_port(&config);

        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        if let Some(ref tls_config) = config.tls {
            let key = std::fs::read(&tls_config.keyfile).expect("IO error on key file");
            let cert = std::fs::read(&tls_config.certfile).expect("IO error on certificate file");
            let tls_app = app.clone();
            let tls_port = utils::get_port_tls(tls_config);
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
            tracing::log::info!("listening on https://{}", addr);
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
        tracing::log::info!("listening on http://{}", addr);
        server_http
            .await
            .expect("Failed to await main HTTP process");
    } else {
        let socket = config.socket.expect("Socket not set?");
        let listener = tokio::net::UnixListener::bind(&socket).expect("Failed to bind to socket");
        let stream = tokio_stream::wrappers::UnixListenerStream::new(listener);
        let acceptor = hyper::server::accept::from_stream(stream);
        let server_http = tokio::spawn(async move {
            axum::Server::builder(acceptor)
                .serve(app.into_make_service())
                .with_graceful_shutdown(async {
                    tokio::signal::ctrl_c()
                        .await
                        .expect("failed to listen for ctrl+c");
                })
                .await
                .expect("Failed to bind to address, is something else using the port?");
        });
        tracing::log::info!("listening on http://{}", &socket);
        server_http
            .await
            .expect("Failed to await main HTTP process");
    }
}
