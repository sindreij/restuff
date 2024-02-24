use std::path::Path;

use anyhow::Result;
use app_state::AppState;
use axum::{routing::get, Router};
use rpc::RpcRouter;
use srpc::SrpcRouter;

mod app_state;
mod db;
mod events;
mod frontend;
mod http_error;
mod http_utils;
mod rpc;
mod thing;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let db_pool = db::create_pool().await?;
    db::run_migrations(&db_pool).await?;

    let app_state = AppState {
        rpc_router: rpc::create_router(&db_pool),
        db_pool,
    };

    let ts_file = prettier::prettier(&RpcRouter::generate_ts());

    let ts_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../restuff-www/src/rpc.ts");
    std::fs::write(ts_path, ts_file)?;

    let app = Router::new()
        .route("/api/srpc/:call", get(rpc::handle_get))
        .fallback(frontend::catchall)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

// #[derive(HelloMacro)]
// struct Pancakes;
