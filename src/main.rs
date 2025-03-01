#![deny(clippy::all, clippy::pedantic)]
use axum::{routing::get, Router};
use expenses_tracker::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(async || "Hello, World!"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
