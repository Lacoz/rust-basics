//! HTTP server bez Leptosu, WASM ani JavaScriptu: odpoveď je čisto to, čo pošle Rust handler.
//! Vyskúšaj: `curl -s http://127.0.0.1:3090/text` alebo `curl -s http://127.0.0.1:3090/`

use axum::{
    http::header,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::time::SystemTime;

const ADDR: &str = "127.0.0.1:3090";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(html_info))
        .route("/text", get(plain_info));

    let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();
    println!("Počúvam http://{ADDR}");
    println!("  curl -s http://{ADDR}/text");
    println!("  curl -s http://{ADDR}/");
    axum::serve(listener, app).await.unwrap();
}

fn server_facts() -> String {
    let now = format!("{:?}", SystemTime::now());
    format!(
        "čas na serveri (SystemTime): {now}\n\
         OS: {} / arch: {}\n\
         Rust {} ({})",
        std::env::consts::OS,
        std::env::consts::ARCH,
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_NAME"),
    )
}

async fn plain_info() -> impl IntoResponse {
    let body = server_facts();
    ([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], body)
}

async fn html_info() -> Html<String> {
    let facts = server_facts();
    let escaped = facts
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="sk">
<head>
  <meta charset="utf-8">
  <title>00 — Axum HTML (bez JS)</title>
</head>
<body>
  <h1>Serverom generovaná stránka</h1>
  <p>Žiadny JavaScript, žiadny WASM — len HTTP odpoveď z Axum.</p>
  <pre>{escaped}</pre>
  <p>Plain text: <code>curl -s http://{ADDR}/text</code></p>
</body>
</html>"#,
    );
    Html(html)
}
