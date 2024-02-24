// TODO: Use ThingId instead of i32 (need support in srpc)

use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{DateTime, Utc},
    SqlitePool,
};
use srpc_derive::ZodGen;

use crate::events::{self, get_all_events};

#[derive(Serialize, Debug, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum ThingEvent {
    SetName(String),
    SetDescription(String),
    Move { move_to: i64 },
}

pub async fn get_thing(db: &SqlitePool, id: i64) -> Result<Option<Thing>> {
    let events = get_all_events::<ThingEvent>(&db, id).await?;

    if events.is_empty() {
        return Ok(None);
    }

    let mut thing = Thing {
        id: events[0].stream_id,
        created_at: events[0].created_at,
        name: String::default(),
        description: String::default(),
        is_in_thing: 0,
    };

    for event in events {
        match event.data {
            ThingEvent::SetName(name) => thing.name = name,
            ThingEvent::SetDescription(description) => thing.description = description,
            ThingEvent::Move { move_to } => thing.is_in_thing = move_to,
        }
    }

    Ok(Some(thing))
}

#[derive(Serialize, ZodGen)]
pub struct Thing {
    id: i64,
    created_at: DateTime<Utc>,
    name: String,
    description: String,
    is_in_thing: i64,
}

pub async fn add_event(db: &SqlitePool, thing_id: i64, event: ThingEvent) -> Result<()> {
    events::insert_event(db, thing_id, event).await?;

    Ok(())
}

// async fn foo(db: SqlitePool) {
//     let things = query_as!(
//         Thing,
//         r#"SELECT id, created_at as "created_at: DateTime<Utc>" from things"#
//     )
//     .fetch_all(&db)
//     .await
//     .unwrap();

//     for thing in things {
//         println!("Thing: {:?}", thing.created_at);
//     }
// }
