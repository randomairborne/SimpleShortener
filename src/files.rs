// basic handler that responds with a static string
crate::static_file!(doc, "resources/doc.html", "text/html");

// basic handler that responds with a static icon file
crate::static_file!(favicon, "resources/favicon.ico", "image/x-icon");

// basic handler that responds with a static png file
crate::static_file!(logo, "resources/logo.png", "image/png");

#[allow(clippy::unused_async)]
pub async fn panel(
    is_init: crate::IsInit,
) -> (axum::http::StatusCode, axum::http::HeaderMap, &'static str) {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("text/html"),
    );
    let file = if is_init.load(std::sync::atomic::Ordering::Relaxed) {
        trace!("Serving standard panel");
        include_str!("resources/panel.html")
    } else {
        trace!("Serving un-init panel");
        include_str!("resources/newuser.html")
    };
    (axum::http::StatusCode::OK, headers, file)
}

/// Usage: `static_file!(root, "index.html", "text/html")`
#[macro_export]
macro_rules! static_file {
    ($name:ident, $path:literal, $ctype:expr) => {
        #[allow(clippy::unused_async)]
        pub async fn $name() -> (axum::http::StatusCode, axum::http::HeaderMap, Vec<u8>) {
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static($ctype),
            );
            let file = include_bytes!($path).to_vec();
            (axum::http::StatusCode::OK, headers, file)
        }
    };
}
