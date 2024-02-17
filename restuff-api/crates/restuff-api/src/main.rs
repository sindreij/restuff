use app_state::AppState;
use axum::{routing::get, Router};

mod app_state;
mod rpc;

#[tokio::main]
async fn main() {
    // Pancakes::hello_macro();

    let app_state = AppState {
        rpc_router: rpc::create_router(),
    };

    let app = Router::new()
        .route("/api/srpc/:call", get(rpc::handle_get))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

// #[derive(HelloMacro)]
// struct Pancakes;
