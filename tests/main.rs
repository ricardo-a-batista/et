use cucumber::{given, then, World};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

#[derive(Debug, Default, World)]
struct EtWorld {
    pool: Option<SqlitePool>,
}

#[given(expr = "a connection pool")]
async fn database_pool(world: &mut EtWorld) -> expenses_tracker::Result<()> {
    let connect = SqlitePoolOptions::new().connect("sqlite::memory:").await;
    world.pool = Some(connect?);
    Ok(())
}

#[then(expr = "migrations run")]
async fn migrations_run(world: &mut EtWorld) -> expenses_tracker::Result<()> {
    if let Some(pool) = &world.pool {
        return expenses_tracker::infra::migrate(pool).await;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    EtWorld::cucumber()
        .fail_on_skipped()
        .run("tests/features/expenses_tracker.feature")
        .await;
}
