// Copyright (c) The Amphitheatre Authors. All rights reserved.
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

use super::{Filters, Statement};
use crate::builder::*;
use crate::model::Entity;
use crate::order::Direction;
use crate::{Database, OmiError, Result};

// Represents a database SELECT operation statement.
#[derive(Clone)]
pub struct SelectStatement<T> {
    pub(crate) entity: PhantomData<T>,

    /// Used to store filter conditions
    pub(crate) filters: Vec<Filters>,

    /// Used to store included associated objects names
    pub(crate) includes: Vec<String>,
    // Used to store included associated objects names
    pub(crate) excludes: Vec<String>,

    /// Used to store grouping fields
    pub(crate) groups: Vec<String>,
    /// Used to store sorting fields and direction
    pub(crate) orders: HashMap<String, Direction>,

    /// Used to store the offset value
    pub(crate) offset: Option<i64>,
    /// Used to store the limit value
    pub(crate) limit: Option<i64>,
}

impl<T> SelectStatement<T>
where
    T: Entity + Default + From<T> + Clone,
{
    pub fn new() -> Self {
        Self {
            entity: PhantomData,
            filters: vec![],
            groups: vec![],
            includes: vec![],
            excludes: vec![],
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

    /// Include associated objects
    pub fn include(&mut self, cols: impl Into<Vec<String>>) -> &mut Self {
        self.includes = cols.into();
        self
    }

    /// Exclude associated objects
    pub fn exclude(&mut self, cols: impl Into<Vec<String>>) -> &mut Self {
        self.excludes = cols.into();
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

    /// Retrieve a single record
    pub fn one(&mut self, db: &Database) -> Result<T> {
        self.limit = Some(1);

        let sql = Builder::build(Statement::Select(self.clone()));
        let result = db.execute::<T>(sql);

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
        let sql = Builder::build(Statement::Select(self.clone()));
        let result = db.execute::<T>(sql);

        match result {
            Ok(entities) => match !entities.is_empty() {
                true => Ok(entities),
                false => Err(OmiError::NotFoundError),
            },
            Err(_) => Err(OmiError::DatabaseError),
        }
    }
}

impl<T> Default for SelectStatement<T>
where
    T: Entity + Default + From<T> + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use omi::prelude::*;

    use super::SelectStatement;
    use crate as omi;
    use crate::order::Direction::Desc;

    #[derive(Debug, Default, Clone, Entity)]
    #[entity(table = "products")]
    struct Product {}

    #[test]
    fn test_group_by() {
        let mut stmt: SelectStatement<Product> = SelectStatement::new();
        stmt.group_by(["id".into(), "title".into()]);

        assert_eq!(stmt.groups, vec!["id", "title"])
    }

    #[test]
    fn test_order_by() {
        let mut stmt: SelectStatement<Product> = SelectStatement::new();
        stmt.order_by([("id".into(), Desc)]);

        assert_eq!(stmt.orders, HashMap::from([("id".into(), Desc)]));
    }
}
