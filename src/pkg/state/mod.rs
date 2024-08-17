use crate::pkg::repository::manager::ModelManager;
use getset::Getters;
use sqlx::SqlitePool;

#[derive(Clone, Debug, Getters)]
pub struct TodoState {
    #[getset(get = "pub with_prefix")]
    manager: ModelManager,
}

impl TodoState {
    pub fn new(pool: SqlitePool) -> Self {
        Self { manager: ModelManager::new(pool) }
    }
}
