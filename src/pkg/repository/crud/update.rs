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

pub async fn update<MC, E>(mm: &ModelManager, id: String, data: E) -> DatabaseResult<()>
where
    MC: DbBmc,
    E: HasSeaFields,
{
    let fields = data.not_none_sea_fields().for_sea_update();
    let mut query = Query::update();
    query.table(MC::table_ref()).values(fields).and_where(Expr::col(CommonIden::Id).eq(id.clone()));
    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
    let sqlx_query = sqlx::query_with(&sql, values);
    let count = sqlx_query.execute(mm.get_pool()).await?.rows_affected();
    if count == 0 {
        return Err(DatabaseError::NotFound { message: format!("Id not found: {}", id) });
    }
    Ok(())
}
