use crate::{admin, files, redirect_handler};
use axum::routing::{delete, get, patch, post, put, Router};
use dashmap::DashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
// Build our app
pub fn build_app() -> axum::Router {
    Router::new()
        .route("/", get(redirect_handler::root))
        .route("/:path", get(redirect_handler::redirect))
        .route("/simpleshortener/api", get(files::doc))
        .route("/simpleshortener/api/", get(files::doc))
        .route("/simpleshortener/api/edit/:id", patch(admin::edit))
        .route("/simpleshortener/api/delete/:id", delete(admin::delete))
        .route("/simpleshortener/api/add", put(admin::add))
        .route("/simpleshortener/api/list", get(admin::list))
        .route("/simpleshortener/api/qr", post(admin::qr))
        .route("/simpleshortener", get(files::panel_html))
        .route("/simpleshortener/", get(files::panel_html))
        .route("/simpleshortener/static/link.png", get(files::logo))
        .route("/favicon.ico", get(files::favicon))
}

pub fn qrgen(link: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let qr_code = qr_code::QrCode::new(link.as_bytes())?;
    let bmp = qr_code.to_bmp();
    let mut bmp_vec: Vec<u8> = Vec::new();
    bmp.write(&mut bmp_vec)?;
    Ok(bmp_vec)
}

// Expect allowed here, as it is only called at the beginning of the program
pub fn read_bincode(filename: &String) -> DashMap<String, String> {
    tracing::debug!("Reading bincode database file!");

    let target_path = Path::new(filename);
    if !target_path.exists() {
        tracing::warn!("Creating new database...");
        let new_map: DashMap<String, String> = DashMap::new();
        let encoded: Vec<u8> = bincode::serialize(&new_map).unwrap();
        std::fs::write(target_path, encoded)
            .expect("failed to write to database file: check permissions");
        return DashMap::new();
    }

    let f = OpenOptions::new()
        .read(true)
        .open(target_path)
        .expect("Failed to open file");

    let deserialized: DashMap<String, String> =
        bincode::deserialize_from(f).expect("Failed to deserialize database");
    tracing::trace!("Got database: {:#?}", deserialized);
    deserialized
}

pub fn save_bincode<P: AsRef<Path>>(
    filename: P,
    map: &DashMap<String, String>,
) -> Result<(), bincode::Error> {
    let mut f = OpenOptions::new().write(true).read(false).open(filename)?;
    tracing::debug!("Saving bincode file...");
    tracing::trace!("with data {:#?}", map);
    let encoded: Vec<u8> = bincode::serialize(&map)?;
    f.write_all(&encoded)?;
    Ok(())
}

pub fn get_port(config: &crate::structs::Config) -> u16 {
    match std::env::var("PORT").map(|x| x.parse::<u16>()) {
        Ok(Ok(port)) => port,
        Err(_) => config.port.expect("Port not set!"),
        Ok(Err(e)) => panic!("port environment variable invalid: {}", e),
    }
}

#[cfg(feature = "tls")]
pub fn get_port_tls(tls_config: &crate::structs::TlsConfig) -> u16 {
    match std::env::var("TLS_PORT").map(|x| x.parse::<u16>()) {
        Ok(Ok(port)) => port,
        Err(_) => tls_config.port.expect("Port not set!"),
        Ok(Err(e)) => panic!("TLS port environment variable invalid: {}", e),
    }
}
