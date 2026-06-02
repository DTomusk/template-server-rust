use std::env;
use tracing::info;

mod app;

// tokio multithreaded runtime needs to be enabled, use full features for simplicity
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // From default env uses RUST_LOG env variable to set log level
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = app::build();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind to address");

    info!(%addr, "server is starting");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}