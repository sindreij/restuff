use axum::{routing::get, Router};

mod srpc;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/srpc/:call", get(srpc::handle_get));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
