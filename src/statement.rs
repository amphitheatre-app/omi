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

use crate::{Database, Entity};

// Declare a type named Statement, which represents a database operation statement.
pub struct Statement<T> {
    ops: Ops,
    filters: Vec<Filters>, // Used to store filter conditions
    groups: Vec<String>,   // Used to store grouping fields
    orders: Vec<String>,   // Used to store sorting fields
    offset: i64,           // Used to store the offset value
    limit: i64,            // Used to store the limit value
    phantom: PhantomData<T>,
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
            filters: vec![],
            groups: vec![],
            orders: vec![],
            offset: 0,
            limit: 20,
            phantom: PhantomData,
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
        self.offset = value;
        self
    }

    /// Implement the limit() method for the Statement type
    pub fn limit(&mut self, value: i64) -> &mut Self {
        // Set the value of the limit attribute
        self.limit = value;
        self
    }

    /// Implement the execute() method for the Statement type
    pub fn execute<R>(&self, db: &Database) -> crate::Result<R>
    where
        R: From<T> + From<Vec<T>>,
    {
        let results = db.query::<T>();

        if type_name::<T>() == type_name::<R>() {
            return Ok(T::default().into());
        }

        return Ok(results.into());
    }

    pub(crate) fn ops(&self) -> &Ops {
        &self.ops
    }
}
