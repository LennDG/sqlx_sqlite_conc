use std::{str::FromStr, time::Duration};

use anyhow::{Error, Result};
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Sqlite,
};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    let db_url = "sqlite://test.db";

    if Sqlite::database_exists(db_url).await? {
        Sqlite::drop_database(db_url).await?;
    }

    let options = SqliteConnectOptions::from_str(db_url)?
        .create_if_missing(true)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .read_only(false);

    let db = SqlitePoolOptions::new().connect_with(options).await?;
    sqlx::migrate!("./migrations").run(&db).await?;

    for i in 0..2000 {
        let (id,) = sqlx::query_as::<_, (i64,)>("INSERT INTO user (name) VALUES ($1) RETURNING id")
            .bind(format!("user-{}", i))
            .fetch_one(&db)
            .await?;

        sqlx::query_as("SELECT (name) FROM user WHERE id = $1")
            .bind(id)
            .fetch_optional(&db)
            .await?
            .ok_or({
                let error = format!("user-{} was not found", i);
                Error::msg(error)
            })?;
    }

    Ok(())
}

//&format!("user-{} was not found", i
