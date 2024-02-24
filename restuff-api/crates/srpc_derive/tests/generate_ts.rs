use axum::Json;
use prettier::prettier;
use serde::Serialize;
use srpc::SrpcRouter;
use srpc_derive::{srpc_router, SrpcOutput};

use pretty_assertions::assert_eq;

struct Router;

#[allow(unused)]
#[derive(Serialize, SrpcOutput)]
struct User {
    id: i32,
    name: String,
    foo: Vec<String>,
}

#[srpc_router]
#[allow(unused)]
impl Router {
    pub fn get_first_user(&self) -> Json<User> {
        Json(User {
            id: 1,
            name: "Alice".to_string(),
            foo: vec![],
        })
    }

    pub fn get_second_user(&self) -> Json<User> {
        Json(User {
            id: 1,
            name: "Bob".to_string(),
            foo: vec![],
        })
    }

    pub fn get_user(&self, id: i32) -> Json<User> {
        Json(User {
            id,
            name: "Bob".to_string(),
            foo: vec![],
        })
    }

    pub fn create_user(&self, name: String, age: i32) -> Json<User> {
        Json(User {
            id: 1,
            name,
            foo: vec![],
        })
    }

    pub fn get_users(&self) -> Json<Vec<User>> {
        Json(vec![])
    }
}

#[test]
fn test_generate_ts() {
    let expected = include_str!("./expected.ts");

    let actual = prettier(&Router::generate_ts());

    assert_eq!(expected, actual);
}
