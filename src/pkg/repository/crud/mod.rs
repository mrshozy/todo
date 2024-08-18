pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use modql::SIden;
use sea_query::{DynIden, Iden, IntoIden, TableRef};

#[derive(Iden)]
pub enum CommonIden {
    Id,
}

pub trait DbBmc {
    const TABLE: &'static str;
    fn table_ref() -> TableRef {
        TableRef::Table(Self::table_iden())
    }
    fn table_iden() -> DynIden {
        SIden(Self::TABLE).into_iden()
    }
}
