use srpc::SrpcOutput;
use srpc_derive::SrpcOutput;

use prettier::prettier;

#[derive(SrpcOutput)]
#[allow(unused)]
struct User {
    name: String,
    age: i32,
}

#[test]
fn test_generates() {
    let schema = prettier(&User::generate_zod_schema());
    let expected = include_str!("expected_zod.ts");

    assert_eq!(schema, expected);
}
