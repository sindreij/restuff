use std::env;

use anyhow::Result;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

pub async fn create_pool() -> Result<SqlitePool> {
    let options = SqliteConnectOptions::new()
        .filename(env::var("SQLITE_FILENAME").unwrap())
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .create_if_missing(true);

    Ok(SqlitePool::connect_with(options).await?)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;

    Ok(())
}
