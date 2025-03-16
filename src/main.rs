#![deny(clippy::all, clippy::pedantic)]
use std::time::Duration;

use axum::{routing::get, Router};
use expenses_tracker::{
    controller::home,
    infra::{migrate, shutdown_signal, AppState},
    Result,
};
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, decompression::RequestDecompressionLayer, services::ServeDir,
    timeout::TimeoutLayer, trace::TraceLayer,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let pool = SqlitePoolOptions::new()
        .connect("file:./et.db?mode=rwc")
        .await?;

    migrate(&pool).await?;

    let app = Router::new()
        .route("/", get(home::index))
        .nest_service("/static", ServeDir::new("static"))
        .layer(
            ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        )
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ))
        .with_state(AppState { pool });

    let bind = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(bind).await?;
    tracing::info!("Server starting: [{}]", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
