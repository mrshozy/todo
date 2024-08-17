use crate::pkg::repository::{
    crud::{
        CommonIden,
        DbBmc,
    },
    error::{
        DatabaseError,
        DatabaseResult,
    },
    manager::ModelManager,
};
use modql::field::HasSeaFields;
use sea_query::{
    Expr,
    Query,
    SqliteQueryBuilder,
};
use sea_query_binder::SqlxBinder;
use sqlx::{
    sqlite::SqliteRow,
    FromRow,
};

pub async fn get<MC, E>(mm: &ModelManager, id: String) -> DatabaseResult<E>
where
    MC: DbBmc,
    E: HasSeaFields,
    E: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
{
    let mut query = Query::select();
    query
        .from(MC::table_ref())
        .columns(E::sea_column_refs())
        .and_where(Expr::col(CommonIden::Id).eq(id.clone()));
    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
    let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
    match sqlx_query.fetch_optional(mm.get_pool()).await? {
        None => Err(DatabaseError::NotFound { message: format!("Id not found: {}", id) }),
        Some(data) => Ok(data),
    }
}
