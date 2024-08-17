pub use crate::log;
use derive_more::Display;
use tracing::dispatcher::SetGlobalDefaultError;

pub type TodoResult<T> = Result<T, TodoError>;

#[derive(Debug, thiserror::Error, Display)]
#[display("{self:?}")]
pub enum TodoError {
    EnvironmentVariable { message: String },
    IO(#[from] std::io::Error),
    DatabaseConnection { message: String },
    DatabaseMigration { message: String },
    Logger(#[from] SetGlobalDefaultError),
}
