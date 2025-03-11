use derive_more::derive::{Display, From};
use sqlx::migrate::MigrateError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From, Display)]
pub enum Error {
    #[from]
    Migrate(MigrateError),

    #[from]
    Sqlx(sqlx::Error),

    #[from]
    Io(std::io::Error),

    #[from]
    Http(axum::http::Error),

    #[from]
    Template(tera::Error),

    #[from]
    InvalidHeaderValue(axum::http::header::InvalidHeaderValue),
}
