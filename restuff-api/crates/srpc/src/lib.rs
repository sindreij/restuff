use axum::response::Response;

pub trait HelloMacro {
    fn hello_macro();
}

pub trait SrpcRouter {
    fn call(&self, call: &str) -> Response;
}
