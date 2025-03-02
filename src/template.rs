use axum::response::IntoResponse;
use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = Tera::new("templates/**/*").unwrap();
}

pub struct Builder {}

impl Builder {
    pub fn build(self) -> Template {
        todo!()
    }
}

pub struct Template {}

impl IntoResponse for Template {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
