use tracing::info;

mod app;
mod auth;
mod config;
mod feature;
mod openapi;
mod repos;

// tokio multithreaded runtime needs to be enabled, use full features for simplicity
#[tokio::main]
async fn main() {
    // Load environment variables from .env
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        // From default env uses RUST_LOG env variable to set log level
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = app::build();

    let config = config::Config::from_env();
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind to address");

    info!(%addr, "server is starting");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}