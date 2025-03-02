#![deny(clippy::all, clippy::pedantic)]
use axum::{routing::get, Router};
use expenses_tracker::{
    controller::home,
    infra::{migrate, AppState},
    Result,
};
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let pool = SqlitePoolOptions::new()
        .connect("file:./et.db?mode=rwc")
        .await?;

    migrate(&pool).await?;

    let app = Router::new()
        .route("/", get(home::index))
        .with_state(AppState { pool });

    let bind = "0.0.0.0:3000";
    tracing::info!("Server starting: [{}]", bind);
    let listener = tokio::net::TcpListener::bind(bind).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
