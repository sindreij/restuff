use prettier::prettier;
use serde::Serialize;
use srpc::SrpcRouter;
use srpc_derive::{srpc_router, ZodGen};

use pretty_assertions::assert_eq;

struct Router;

#[allow(unused)]
#[derive(Serialize, ZodGen)]
struct User {
    id: i32,
    name: String,
    foo: Vec<String>,
}

#[srpc_router]
#[allow(unused)]
impl Router {
    pub fn get_first_user(&self) -> User {
        User {
            id: 1,
            name: "Alice".to_string(),
            foo: vec![],
        }
    }

    pub fn get_second_user(&self) -> User {
        User {
            id: 1,
            name: "Bob".to_string(),
            foo: vec![],
        }
    }

    pub fn get_users(&self) -> Vec<User> {
        vec![]
    }
}

#[test]
fn test_generate_ts() {
    let expected = include_str!("./expected.ts");

    let actual = prettier(&Router::generate_ts());

    assert_eq!(expected, actual);
}
