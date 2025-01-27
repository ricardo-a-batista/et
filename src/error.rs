use derive_more::derive::{Display, From};
use sqlx::migrate::MigrateError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From, Display)]
pub enum Error {
    #[from]
    Migrate(MigrateError),

    #[from]
    Sqlx(sqlx::Error),
}
