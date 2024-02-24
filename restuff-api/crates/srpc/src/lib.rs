use async_trait::async_trait;
use axum::response::Response;
use serde::Serialize;

mod zod;

pub use zod::ZodGen;

pub trait HelloMacro {
    fn hello_macro();
}

#[async_trait]
pub trait SrpcRouter {
    async fn call(&self, call: &str) -> Response;
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
