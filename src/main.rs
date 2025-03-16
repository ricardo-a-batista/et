#![deny(clippy::all, clippy::pedantic)]
use axum::{routing::get, Router};
use expenses_tracker::{
    controller::home,
    infra::{migrate, AppState},
    Result,
};
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, decompression::RequestDecompressionLayer, trace::TraceLayer,
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
        .layer(
            ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(AppState { pool });

    let bind = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(bind).await?;
    tracing::info!("Server starting: [{}]", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
