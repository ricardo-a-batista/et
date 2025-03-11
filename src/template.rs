use axum::{
    http::{Response, StatusCode},
    response::IntoResponse,
};
use lazy_static::lazy_static;
use tera::{Context, Tera};

use crate::{Error, Result};

lazy_static! {
    pub static ref TEMPLATES: Tera = Tera::new("templates/**/*").unwrap();
}

#[derive(Default, Debug)]
pub struct Template {
    body: String,
    template: Option<String>,
    status_code: Option<StatusCode>,
    headers: Vec<(String, String)>,
    context: Vec<(String, String)>,
}

impl Template {
    pub fn with_body(self, body: String) -> Self {
        Self { body, ..self }
    }

    pub fn with_template(self, template: String) -> Self {
        Self {
            template: Some(template),
            ..self
        }
    }

    pub fn with_headers(self, headers: Vec<(String, String)>) -> Self {
        Self { headers, ..self }
    }

    pub fn with_status_code(self, status_code: StatusCode) -> Self {
        Self {
            status_code: Some(status_code),
            ..self
        }
    }

    pub fn with_context(self, context: Vec<(String, String)>) -> Self {
        Self { context, ..self }
    }

    pub fn build(self) -> Result<Response<String>> {
        let status_code = match self.status_code {
            Some(status_code) => status_code,
            None => StatusCode::OK,
        };

        let body = match self.template {
            Some(template_name) => {
                let mut context = Context::new();
                for (key, val) in &self.context {
                    context.insert(key, val);
                }

                TEMPLATES.render(&template_name, &context)?
            }
            None => self.body,
        };

        let mut response_builder = Response::builder().status(status_code);
        for (key, value) in self.headers {
            response_builder = response_builder.header(key, value);
        }

        Ok(response_builder.body(body)?)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
