// Copyright 2022 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::marker::PhantomData;

use crate::builder::*;
use crate::entity::Entity;
use crate::order::Direction;
use crate::{Database, OmiError, Result};

// Declare a type named Statement, which represents a database operation statement.
pub struct Statement<T> {
    pub(crate) ops: Ops,
    pub(crate) entity: PhantomData<T>,

    pub(crate) filters: Vec<Filters>, // Used to store filter conditions
    pub(crate) groups: Vec<String>,   // Used to store grouping fields
    pub(crate) orders: HashMap<String, Direction>, // Used to store sorting fields
    pub(crate) offset: Option<i64>,   // Used to store the offset value
    pub(crate) limit: Option<i64>,    // Used to store the limit value
}

#[derive(PartialEq, Eq, Debug)]
pub enum Ops {
    Insert,
    Update,
    Select,
    Delete,
}

pub struct Filters {}

impl<T> Statement<T>
where
    T: Entity + Default + From<T> + Clone,
{
    pub fn new(ops: Ops) -> Self {
        Self {
            ops,
            entity: PhantomData,
            filters: vec![],
            groups: vec![],
            orders: HashMap::new(),
            offset: None,
            limit: None,
        }
    }

    /// Implement the filter() method for the Statement type
    pub fn filter(&mut self, filters: Filters) -> &mut Self {
        // Add the filter conditions to the filters attribute
        self.filters.push(filters);
        self
    }

    /// Add the grouping columns to the groups attribute
    pub fn group_by(&mut self, cols: impl Into<Vec<String>>) -> &mut Self {
        self.groups = cols.into();
        self
    }

    /// Add the sorting field to the orders attribute
    pub fn order_by(&mut self, cols: impl Into<HashMap<String, Direction>>) -> &mut Self {
        self.orders = cols.into();
        self
    }

    /// Implement the offset() method for the Statement type
    pub fn offset(&mut self, value: i64) -> &mut Self {
        // Set the value of the offset attribute
        self.offset = Some(value);
        self
    }

    /// Implement the limit() method for the Statement type
    pub fn limit(&mut self, value: i64) -> &mut Self {
        // Set the value of the limit attribute
        self.limit = Some(value);
        self
    }

    fn build(&self) -> String {
        match self.ops {
            Ops::Insert => build_insert_sql(self),
            Ops::Update => build_update_sql(self),
            Ops::Select => build_select_sql(self),
            Ops::Delete => build_delete_sql(self),
        }
    }

    /// Implement the execute() method for the Statement type
    pub fn execute(&self, db: &Database) -> Result<Vec<T>> {
        let sql = self.build();
        let result = db.query::<T>(sql);

        match result {
            Ok(entities) => Ok(entities),
            Err(_) => Err(OmiError::DatabaseError),
        }
    }

    /// Retrieve a single record
    pub fn one(&mut self, db: &Database) -> Result<T> {
        self.limit = Some(1);

        let sql = self.build();
        let result = db.query::<T>(sql);

        match result {
            Ok(entities) => match entities.first() {
                Some(entity) => Ok(entity.clone()),
                None => Err(OmiError::NotFoundError),
            },
            Err(_) => Err(OmiError::DatabaseError),
        }
    }

    /// Fetch multiple rows
    pub fn all(&self, db: &Database) -> Result<Vec<T>> {
        let sql = self.build();
        let result = db.query::<T>(sql);

        match result {
            Ok(entities) => match !entities.is_empty() {
                true => Ok(entities),
                false => Err(OmiError::NotFoundError),
            },
            Err(_) => Err(OmiError::DatabaseError),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use omi::prelude::*;

    use crate as omi;
    use crate::order::Direction::Desc;
    use crate::{Ops, Statement};

    #[derive(Debug, Default, Clone, Entity)]
    #[entity(table = "products")]
    struct Product {}

    #[test]
    fn test_group_by() {
        let mut stmt: Statement<Product> = Statement::new(Ops::Select);
        stmt.group_by(["id".into(), "title".into()]);

        assert_eq!(stmt.groups, vec!["id", "title"])
    }

    #[test]
    fn test_order_by() {
        let mut stmt: Statement<Product> = Statement::new(Ops::Select);
        stmt.order_by([("id".into(), Desc)]);

        assert_eq!(stmt.orders, HashMap::from([("id".into(), Desc)]));
    }
}
