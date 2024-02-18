use std::convert::Infallible;

use axum::{
    body::Body,
    http::{HeaderName, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum HttpError {
    #[error("Unauthorized: {0}")]
    Unauthorized(&'static str),
    #[error("Bad Request: {0}")]
    BadRequest(&'static str),
    #[error("Bad Request: {0}")]
    BadRequestString(String),
    #[error("Not Found")]
    NotFound,

    #[error("Internal Server Error: {0}")]
    InternalServerError(&'static str),

    #[error("Status code: {0}")]
    StatusCodeError(StatusCode),

    #[error("HTTP: {0}")]
    Http(#[from] axum::http::Error),
    #[error("Json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Extension: {0}")]
    Extension(#[from] axum::extract::rejection::ExtensionRejection),
    // #[error("Sqlx: {0}")]
    // Sqlx(#[from] sqlx::Error),
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
    // #[error("ErrReport: {0}")]
    // ErrReport(#[from] eyre::Report),
    #[error("Invalid Header Value")]
    InvalidHeaderValue(#[from] axum::http::header::InvalidHeaderValue),
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
    #[error("JoinError: {0}")]
    Join(#[from] tokio::task::JoinError),
    #[error("Converting header to string: {0}")]
    ToStrError(#[from] axum::http::header::ToStrError),
    #[error("Format error: {0}")]
    Fmt(#[from] std::fmt::Error),

    #[error("Infallible")]
    Infallible(#[from] Infallible),
}

pub type Result<T, E = HttpError> = std::result::Result<T, E>;

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_json) = match self {
            HttpError::NotFound => (StatusCode::NOT_FOUND, json!("Not Found")),
            HttpError::BadRequest(msg) => (StatusCode::BAD_REQUEST, json!(msg)),
            HttpError::BadRequestString(msg) => (StatusCode::BAD_REQUEST, json!(msg)),
            // HttpError::Forbidden(msg) => (StatusCode::FORBIDDEN, json!(msg)),
            HttpError::Unauthorized(val) => (StatusCode::UNAUTHORIZED, json!(val)),
            // HttpError::Access(val) => (StatusCode::UNAUTHORIZED, json!(val.to_string())),
            HttpError::InternalServerError(val) => {
                (StatusCode::INTERNAL_SERVER_ERROR, json!(val.to_string()))
            }
            HttpError::StatusCodeError(code) => (code, json!(code.to_string())),

            other => {
                eprintln!("Internal Server Error: {:?}", other);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!("Internal Server Error"),
                )
            }
        };

        let mut response = Json(json!({
            "error": error_json,
        }))
        .into_response();

        *response.status_mut() = status;

        response
    }
}

pub fn response_from_reqwest_body(
    res: reqwest::Response,
    // cache: Cache,
) -> Result<Response, HttpError> {
    let mut response_builder = Response::builder()
        .status(res.status().as_u16())
        .header("Cache-Control", "no-cache");

    let headers = response_builder
        .headers_mut()
        .ok_or(HttpError::InternalServerError("Headers not found"))?;

    for (key, value) in res.headers() {
        if key != "connection" && key != "cache-control" {
            headers.append(
                HeaderName::try_from(key.to_string()).unwrap(),
                value.to_str().unwrap().parse().unwrap(),
            );
        }
    }

    // let cache_header = match cache {
    //     Cache::KeepUpstream => res
    //         .headers()
    //         .get("cache-control")
    //         .cloned()
    //         .unwrap_or_else(|| "".parse().unwrap()),
    //     Cache::No => "no-cache".parse().unwrap(),
    // };

    // headers.append(
    //     "cache-control",
    //     cache_header.to_str().unwrap().parse().unwrap(),
    // );

    Ok(response_builder.body(Body::from_stream(res.bytes_stream()))?)
}

// #[derive(Debug, Clone, Copy)]
// pub enum Cache {
//     KeepUpstream,
//     No,
// }
