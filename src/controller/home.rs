use axum::response::Response;

use crate::{template::Template, Result};

#[tracing::instrument]
pub async fn index() -> Result<Response<String>> {
    Template::default()
        .with_body(String::from("Hello, World!"))
        .build()
}
