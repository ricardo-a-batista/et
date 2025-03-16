use sqlx::{migrate::Migrator, Database, Pool, Sqlite};
use tokio::signal;

use crate::Result;

static MIGRATE: Migrator = sqlx::migrate!("./migrations");

pub async fn migrate<DB: Database>(pool: &Pool<DB>) -> Result<()>
where
    <DB as sqlx::Database>::Connection: sqlx::migrate::Migrate,
{
    MIGRATE.run(pool).await?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: Pool<Sqlite>,
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
