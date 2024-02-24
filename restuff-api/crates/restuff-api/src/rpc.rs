use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::Uri,
    response::Response,
    Json,
};
use serde::Serialize;
use sqlx::SqlitePool;
use srpc::SrpcRouter;
use srpc_derive::{srpc_router, SrpcOutput};

use crate::{
    http_error::HttpError,
    thing::{self, Thing, ThingEvent},
};

pub fn create_router(db: &SqlitePool) -> RpcRouter {
    RpcRouter { db: db.clone() }
}

#[axum::debug_handler]
pub async fn handle_get(Path(call): Path<String>, uri: Uri, router: State<RpcRouter>) -> Response {
    router.call(&call, uri).await
}

#[derive(Clone, Debug)]
pub struct RpcRouter {
    db: SqlitePool,
}

#[srpc_router]
impl RpcRouter {
    pub async fn get_thing(&self, id: i64) -> Result<Json<Option<Thing>>, HttpError> {
        Ok(Json(thing::get_thing(&self.db, id).await?))
    }

    pub async fn set_thing_name(&self) -> Result<Json<()>, HttpError> {
        thing::add_event(&self.db, 1, ThingEvent::SetName("foobar".to_string())).await?;

        Ok(Json(()))
    }

    pub fn user_list(&self) -> Json<Vec<User>> {
        Json(vec![
            User {
                id: 1,
                name: "Joh".to_string(),
            },
            User {
                id: 2,
                name: "Doe".to_string(),
            },
        ])
    }

    pub fn get_first_user(&self) -> Json<User> {
        Json(User {
            id: 1,
            name: "John".to_string(),
        })
    }

    pub fn foobar(&self) -> Json<i32> {
        Json(42)
    }
}

#[derive(Serialize, SrpcOutput)]
pub struct User {
    id: i32,
    name: String,
}
