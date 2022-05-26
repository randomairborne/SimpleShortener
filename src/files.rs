use axum_static_macro::static_file;

// basic handler that responds with a static string
static_file!(doc, "resources/doc.html", "text/html");

// basic handler that responds with a static string
static_file!(panel, "resources/panel.html", "text/html");

// basic handler that responds with a static icon file
static_file!(favicon, "resources/favicon.ico", "image/x-icon");

// basic handler that responds with a static png file
static_file!(logo, "resources/logo.png", "image/png");
