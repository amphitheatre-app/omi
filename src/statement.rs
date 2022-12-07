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

use std::any::type_name;
use std::marker::PhantomData;

use crate::builder::*;
use crate::entity::Entity;
use crate::{Database, OmiError, Result};

// Declare a type named Statement, which represents a database operation statement.
pub struct Statement<T> {
    pub(crate) ops: Ops,
    pub(crate) entity: PhantomData<T>,

    pub(crate) filters: Vec<Filters>, // Used to store filter conditions
    pub(crate) groups: Vec<String>,   // Used to store grouping fields
    pub(crate) orders: Vec<String>,   // Used to store sorting fields
    pub(crate) offset: Option<i64>,   // Used to store the offset value
    pub(crate) limit: Option<i64>,    // Used to store the limit value
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Ops {
    Insert,
    Update,
    Select,
    Delete,
}

pub struct Filters {}

impl<T> Statement<T>
where
    T: Entity + Default,
{
    pub(crate) fn new(ops: Ops) -> Self {
        Self {
            ops,
            entity: PhantomData,
            filters: vec![],
            groups: vec![],
            orders: vec![],
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

    /// Implement the group_by() method for the Statement type
    pub fn group_by(&mut self, field: &str) -> &mut Self {
        // Add the grouping field to the groups attribute
        self.groups.push(field.to_string());
        self
    }

    /// Implement the order_by() method for the Statement type
    pub fn order_by(&mut self, field: &str) -> &mut Self {
        // Add the sorting field to the orders attribute
        self.orders.push(field.to_string());
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
    pub fn one(&self, db: &Database) -> Result<T> {
        todo!()
    }

    /// Fetch multiple rows
    pub fn all(&self, db: &Database) -> Result<Vec<T>> {
        todo!()
    }
}
