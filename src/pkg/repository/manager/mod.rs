use getset::Getters;
use sqlx::SqlitePool;

#[derive(Clone, Debug, Getters)]
pub struct ModelManager {
    #[getset(get = "pub with_prefix")]
    pool: SqlitePool,
}

impl ModelManager {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
