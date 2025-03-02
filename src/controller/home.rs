#[tracing::instrument]
pub async fn index() -> &'static str {
    "Hello, World!"
}
