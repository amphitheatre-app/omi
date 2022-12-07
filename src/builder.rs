use sql_query_builder as sql;

use crate::entity::Entity;
use crate::Statement;

pub(crate) fn build_insert_sql<T: Entity>(stmt: &Statement<T>) -> String {
    sql::Insert::new().as_string()
}

pub(crate) fn build_update_sql<T: Entity>(stmt: &Statement<T>) -> String {
    sql::Update::new().as_string()
}

pub(crate) fn build_select_sql<T: Entity>(stmt: &Statement<T>) -> String {
    let meta = T::meta();
    sql::Select::new().select("*").from(&meta.table).as_string()
}

pub(crate) fn build_delete_sql<T: Entity>(stmt: &Statement<T>) -> String {
    sql::Delete::new().as_string()
}
