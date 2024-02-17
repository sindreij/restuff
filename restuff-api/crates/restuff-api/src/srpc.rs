use axum::{
    extract::Path,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub async fn handle_get(Path(call): Path<String>) -> Response {
    match call.as_str() {
        "user_list" => {
            let users = Router::user_list();
            axum::Json(users).into_response()
        }
        _ => (
            axum::http::StatusCode::NOT_FOUND,
            axum::Json(SrpcError::from("No such call")),
        )
            .into_response(),
    }
}

#[derive(Serialize)]
struct SrpcError {
    message: String,
}

impl From<&str> for SrpcError {
    fn from(message: &str) -> Self {
        SrpcError {
            message: message.to_string(),
        }
    }
}

// So this is like t.router
struct Router;

impl Router {
    fn user_list() -> Vec<User> {
        vec![
            User {
                id: 1,
                name: "John".to_string(),
            },
            User {
                id: 2,
                name: "Doe".to_string(),
            },
        ]
    }
}

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}
