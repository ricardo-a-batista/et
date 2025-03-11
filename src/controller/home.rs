use axum::response::Response;

use crate::{template::Template, Result};

#[tracing::instrument]
pub async fn index() -> Result<Response<String>> {
    Template::default()
        .with_template(String::from("home/index.html"))
        .with_context(vec![(
            String::from("hello_world"),
            String::from("Hello, World!"),
        )])
        .build()
}
