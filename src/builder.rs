use sql_query_builder as sql;

use crate::entity::Entity;
use crate::statement::*;

pub struct Builder {}

impl Builder {
    pub fn build<T>(statement: Statement<T>) -> String
    where
        T: Entity,
    {
        match statement {
            Statement::Delete(stmt) => Self::build_delete_sql(stmt),
            Statement::Insert(stmt) => Self::build_insert_sql(stmt),
            Statement::Raw(stmt) => Self::build_raw_sql(stmt),
            Statement::Select(stmt) => Self::build_select_sql(stmt),
            Statement::Update(stmt) => Self::build_update_sql(stmt),
        }
    }

    fn build_delete_sql<T>(_statement: DeleteStatement<T>) -> String
    where
        T: Entity,
    {
        sql::Delete::new().as_string()
    }

    fn build_raw_sql<T>(statement: RawStatement<T>) -> String
    where
        T: Entity,
    {
        sql::Select::new().raw(&statement.sql).as_string()
    }

    fn build_insert_sql<T>(_statement: InsertStatement<T>) -> String
    where
        T: Entity,
    {
        sql::Insert::new().as_string()
    }

    fn build_select_sql<T>(_statement: SelectStatement<T>) -> String
    where
        T: Entity,
    {
        let meta = T::meta();
        sql::Select::new().select("*").from(&meta.table).as_string()
    }

    fn build_update_sql<T>(_statement: UpdateStatement<T>) -> String
    where
        T: Entity,
    {
        sql::Update::new().as_string()
    }
}
