use derive_more::Display;

pub type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(Debug, thiserror::Error, Display)]
#[display("{self:?}")]
pub enum DatabaseError {
    Duplicate { message: String },
    NotFound { message: String },
    Binding(#[from] sea_query::error::Error),
    Sql(#[from] sqlx::Error),
}
