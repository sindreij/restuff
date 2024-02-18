use prettier::prettier;
use serde::Serialize;
use srpc::SrpcRouter;
use srpc_derive::srpc_router;

use pretty_assertions::assert_eq;

struct Router;

#[allow(unused)]
#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

#[srpc_router]
#[allow(unused)]
impl Router {
    pub fn get_first_user(&self) -> User {
        User {
            id: 1,
            name: "Alice".to_string(),
        }
    }

    pub fn get_second_user(&self) -> User {
        User {
            id: 1,
            name: "Bob".to_string(),
        }
    }
}

#[test]
fn test_generate_ts() {
    let expected = include_str!("./expected.ts");

    let actual = prettier(&Router::generate_ts());

    assert_eq!(expected, actual);
}
