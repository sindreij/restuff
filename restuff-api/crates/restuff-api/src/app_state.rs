use axum::extract::FromRef;

use crate::rpc::RpcRouter;

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub rpc_router: RpcRouter,
}
