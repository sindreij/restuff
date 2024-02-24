use async_trait::async_trait;
use axum::{http::Uri, response::Response};
use serde::Serialize;

pub use query::SrpcQueryParams;

mod query;
mod zod;

pub use zod::ZodSchema;

#[async_trait]
pub trait SrpcRouter {
    async fn call(&self, call: &str, uri: Uri) -> Response;
    fn generate_ts() -> String;
}

#[derive(Serialize)]
pub struct SrpcError {
    message: String,
}

impl From<&str> for SrpcError {
    fn from(message: &str) -> Self {
        SrpcError {
            message: message.to_string(),
        }
    }
}

impl From<String> for SrpcError {
    fn from(message: String) -> Self {
        SrpcError { message: message }
    }
}
