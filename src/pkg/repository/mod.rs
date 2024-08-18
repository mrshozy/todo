use crate::preludes::{
    log,
    TodoError,
    TodoResult,
};
use sqlx::{
    migrate,
    sqlite::{
        SqliteConnectOptions,
        SqlitePoolOptions,
    },
    SqlitePool,
};
use std::str::FromStr;

pub mod crud;
pub mod error;
pub mod manager;
pub mod models;

pub async fn todo_migration(pool: &SqlitePool) -> TodoResult<()> {
    log!(info, "running database migration");
    migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| TodoError::DatabaseMigration { message: e.to_string() })
}

pub async fn establish_connection(database_url: &str) -> TodoResult<SqlitePool> {
    log!(info, "connecting to database");

    let options = SqliteConnectOptions::from_str(database_url)
        .map_err(|e| TodoError::DatabaseConnection { message: e.to_string() })?
        .create_if_missing(true)
        .foreign_keys(true);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|e| TodoError::DatabaseConnection { message: e.to_string() })
}
