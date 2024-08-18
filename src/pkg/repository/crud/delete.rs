use crate::pkg::repository::{
    crud::{CommonIden, DbBmc},
    error::{DatabaseError, DatabaseResult},
    manager::ModelManager,
};
use sea_query::{Expr, Query, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;
pub async fn delete<MC>(mm: &ModelManager, id: String) -> DatabaseResult<()>
where
    MC: DbBmc,
{
    let mut query = Query::delete();
    query
        .from_table(MC::table_ref())
        .and_where(Expr::col(CommonIden::Id).eq(id.clone()));
    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
    let sqlx_query = sqlx::query_with(&sql, values);
    let count = sqlx_query.execute(mm.get_pool()).await?.rows_affected();
    if count == 0 {
        return Err(DatabaseError::NotFound {
            message: format!("Id not found: {}", id),
        });
    }
    Ok(())
}
