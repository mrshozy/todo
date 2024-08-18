use crate::pkg::repository::{
    crud::DbBmc,
    error::{
        DatabaseError::{
            Duplicate,
            Sql,
        },
        DatabaseResult,
    },
    manager::ModelManager,
};
use modql::field::HasSeaFields;
use sea_query::{
    Query,
    SqliteQueryBuilder,
};
use sea_query_binder::SqlxBinder;
use sqlx::{
    sqlite::SqliteRow,
    FromRow,
};

pub async fn create<MC, E, R>(mm: &ModelManager, data: E) -> DatabaseResult<R>
where
    MC: DbBmc,
    E: HasSeaFields,
    R: for<'r> FromRow<'r, SqliteRow> + Unpin + Send,
{
    let fields = data.not_none_sea_fields();
    let (columns, values) = fields.for_sea_insert();
    let mut query = Query::insert();
    query
        .into_table(MC::table_ref())
        .columns(columns)
        .values(values)?
        .returning(Query::returning().all());
    let (sql, values) = query.build_sqlx(SqliteQueryBuilder);
    let sqlx_query = sqlx::query_as_with::<_, R, _>(&sql, values);
    let data =
        sqlx_query.fetch_one(mm.get_pool()).await.map_err(|x| match x.as_database_error() {
            None => Sql(x),
            Some(e) => {
                if e.code()
                    .and_then(|x| x.parse::<i32>().ok())
                    .map(|value| value == 2067)
                    .unwrap_or(false)
                {
                    return Duplicate { message: e.message().to_string() };
                }
                Sql(x)
            }
        })?;
    Ok(data)
}
