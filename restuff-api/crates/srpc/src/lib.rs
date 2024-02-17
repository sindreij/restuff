use axum::response::Response;
use serde::Serialize;

pub trait HelloMacro {
    fn hello_macro();
}

pub trait SrpcRouter {
    fn call(&self, call: &str) -> Response;
    fn generate_ts() -> &'static str;
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
