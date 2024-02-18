use std::path::Path;

use app_state::AppState;
use axum::{routing::get, Router};
use rpc::RpcRouter;
use srpc::SrpcRouter;

mod app_state;
mod frontend;
mod http_utils;
mod rpc;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app_state = AppState {
        rpc_router: rpc::create_router(),
    };

    let ts_file = RpcRouter::generate_ts();

    let ts_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../restuff-www/src/rpc.ts");
    std::fs::write(ts_path, ts_file).unwrap();

    let app = Router::new()
        .route("/api/srpc/:call", get(rpc::handle_get))
        .fallback(frontend::catchall)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

// #[derive(HelloMacro)]
// struct Pancakes;
