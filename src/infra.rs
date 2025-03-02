use sqlx::{migrate::Migrator, Database, Pool, Sqlite};

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
