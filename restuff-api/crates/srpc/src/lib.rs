use async_trait::async_trait;
use axum::{http::Uri, response::Response};
use serde::Serialize;

pub use query::SrpcQueryParams;
pub use srpc_input::SrpcInput;
pub use srpc_output::SrpcOutput;

mod query;
mod srpc_input;
mod srpc_output;

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
