use axum::{
    routing::get,
    Router,
};

use std::env;

// tokio multithreaded runtime needs to be enabled, use full features for simplicity
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler));

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    println!("Server running on http://{}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, World!"
}