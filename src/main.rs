pub extern crate tracing;
use crate::{
    pkg::{
        establish_connection, router::create_router, server::serve_forever, state::TodoState,
        todo_migration,
    },
    preludes::{TodoError, TodoResult},
};
use pkg::conf::Config;
use tracing_subscriber::{fmt, EnvFilter};
mod macros;
mod pkg;
mod preludes;

#[allow(dead_code)]
const APPLICATION_NAME: &str = "todo";

#[tokio::main]
async fn main() -> TodoResult<()> {
    init_logger()?;
    log!(info, "Initializing environment variables");
    let config =
        Config::from_env(std::env::vars()).map_err(|e| TodoError::EnvironmentVariable {
            message: e.to_string(),
        })?;
    let pool = establish_connection(config.get_database_url()).await?;
    log!(info, "connected to database");
    todo_migration(&pool).await?;
    log!(info, "migration run against database successfully");
    let state = TodoState::new(pool);
    let router = create_router(state);
    serve_forever(router, config.get_port()).await
}

fn init_logger() -> TodoResult<()> {
    let subscribers = fmt()
        .compact()
        .with_thread_ids(false)
        .with_target(true)
        .with_level(true)
        .with_line_number(true)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscribers)?;
    Ok(())
}
