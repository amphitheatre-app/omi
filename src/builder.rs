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
    let builder = sql::Select::new();
    builder.select("*");
    // builder.from(&stmt.entity.meta().table);

    if let Some(offset) = stmt.offset {
        builder.offset(offset.to_string().as_str());
    }

    if let Some(limit) = stmt.limit {
        builder.limit(limit.to_string().as_str());
    }

    builder.as_string()
}

pub(crate) fn build_delete_sql<T: Entity>(stmt: &Statement<T>) -> String {
    sql::Delete::new().as_string()
}
