use axum::{
    routing::get,
    Router,
};

// tokio multithreaded runtime needs to be enabled, use full features for simplicity
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, World!"
}