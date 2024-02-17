use axum::{
    extract::{Path, State},
    response::Response,
};
use serde::Serialize;
use srpc::SrpcRouter;
use srpc_derive::srpc_router;

pub fn create_router() -> RpcRouter {
    RpcRouter
}

pub async fn handle_get(Path(call): Path<String>, router: State<RpcRouter>) -> Response {
    router.call(&call)
}

#[derive(Clone, Debug)]
pub struct RpcRouter;

#[srpc_router]
impl RpcRouter {
    pub fn user_list(&self) -> Vec<User> {
        vec![
            User {
                id: 1,
                name: "Joh".to_string(),
            },
            User {
                id: 2,
                name: "Doe".to_string(),
            },
        ]
    }

    pub fn get_first_user(&self) -> User {
        User {
            id: 1,
            name: "John".to_string(),
        }
    }

    pub fn foobar(&self) -> i32 {
        42
    }
}

#[derive(Serialize)]
pub struct User {
    id: i32,
    name: String,
}
