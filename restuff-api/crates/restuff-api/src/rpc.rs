use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use srpc::SrpcRouter;
use srpc_derive::SrpcRouter;

pub fn create_router() -> RpcRouter {
    RpcRouter
}

pub async fn handle_get(Path(call): Path<String>, router: State<RpcRouter>) -> Response {
    router.call(&call)
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
// #[derive(SrpcRouter)]
#[derive(SrpcRouter, Clone, Debug)]
pub struct RpcRouter;

// impl SrpcRouter for RpcRouter {
//     fn call(&self, call: &str) -> Response {
//         match call {
//             "user_list" => {
//                 let users = RpcRouter::user_list();
//                 axum::Json(users).into_response()
//             }
//             "get_first_user" => {
//                 let user = RpcRouter::get_first_user();
//                 axum::Json(user).into_response()
//             }
//             _ => (
//                 axum::http::StatusCode::NOT_FOUND,
//                 axum::Json(SrpcError::from("No such call")),
//             )
//                 .into_response(),
//         }
//     }
// }

impl RpcRouter {
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

    fn get_first_user() -> User {
        User {
            id: 1,
            name: "John".to_string(),
        }
    }
}

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}
