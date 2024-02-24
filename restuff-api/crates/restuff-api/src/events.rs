// trait StreamType {
//     type Output;

//     fn get_current() -> Self::Output;

// }

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use sqlx::{query, SqlitePool};

#[derive(Debug)]
pub struct Event<Data> {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub stream_id: i64,
    pub version: i64,
    pub data: Data,
}

pub async fn get_all_events<Data: DeserializeOwned>(
    pool: &SqlitePool,
    stream_id: i64,
) -> Result<Vec<Event<Data>>> {
    let events = query!(
        r#"SELECT id, created_at as "created_at: DateTime<Utc>", stream_id, version, data as "data!: serde_json::Value" from events where stream_id = ? ORDER BY version ASC"#,
        stream_id
    )
    .fetch_all(pool)
    .await?;

    Ok(events
        .into_iter()
        .map(|event| Event {
            id: event.id.unwrap(),
            created_at: event.created_at,
            stream_id: event.stream_id,
            version: event.version,
            data: serde_json::from_value(event.data).unwrap(),
        })
        .collect())
}

pub async fn insert_event<Data: serde::Serialize>(
    pool: &SqlitePool,
    stream_id: i64,
    data: Data,
) -> Result<()> {
    let mut transaction = pool.begin().await?;

    let version = query!(
        "SELECT COALESCE(MAX(version), 0) + 1 as version from events where stream_id = ?",
        stream_id
    )
    .fetch_one(&mut *transaction)
    .await?
    .version;

    let value = serde_json::to_value(&data)?;
    query!(
        "INSERT INTO events (stream_id, version, data) VALUES (?, ?, ?)",
        stream_id,
        version,
        value,
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use sqlx::SqlitePool;

    #[sqlx::test]
    async fn test_get_events(db: SqlitePool) {
        #[derive(Serialize, Debug, Deserialize)]
        #[serde(tag = "t", content = "c")]
        enum EventData {
            SetBlah(String),
            RemoveBlah,
        }

        insert_event(&db, 42, EventData::SetBlah("foo".to_string()))
            .await
            .unwrap();

        insert_event(&db, 42, EventData::RemoveBlah).await.unwrap();

        let events = get_all_events::<EventData>(&db, 42).await.unwrap();

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].stream_id, 42);
        assert_eq!(events[1].stream_id, 42);
        assert_eq!(events[0].version, 1);
        assert_eq!(events[1].version, 2);

        let data = events.iter().map(|e| &e.data).collect::<Vec<_>>();

        insta::assert_debug_snapshot!(data, @r###"
        [
            SetBlah(
                "foo",
            ),
            RemoveBlah,
        ]
        "###);

        let events_other_stream = get_all_events::<EventData>(&db, 43).await.unwrap();
        assert_eq!(events_other_stream.len(), 0);
    }
}
