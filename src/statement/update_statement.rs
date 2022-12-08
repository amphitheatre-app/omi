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

use super::{Filters, Statement};
use crate::builder::*;
use crate::entity::Entity;
use crate::{Database, OmiError, Result};

// Represents a database UPDATE operation statement.
#[derive(Clone)]
pub struct UpdateStatement<T> {
    pub(crate) entity: Option<T>,

    /// Used to store filter conditions
    pub(crate) filters: Vec<Filters>,

    /// Used to store specified fields will be update.
    pub(crate) changes: HashMap<String, String>,
}

impl<T> UpdateStatement<T>
where
    T: Entity + Default + From<T> + Clone,
{
    pub fn new(entity: Option<T>) -> Self {
        Self {
            entity,
            filters: vec![],
            changes: HashMap::new(),
        }
    }

    /// Implement the filter() method for the Statement type
    pub fn filter(&mut self, filters: Filters) -> &mut Self {
        // Add the filter conditions to the filters attribute
        self.filters.push(filters);
        self
    }

    /// Set the specified fields to update
    pub fn set(&mut self, fields: impl Into<HashMap<String, String>>) -> &mut Self {
        self.changes = fields.into();
        self
    }

    /// Implement the execute() method for the Statement type
    pub fn execute(&self, db: &Database) -> Result<Vec<T>> {
        let sql = Builder::build(Statement::Update(self.clone()));

        let result = db.query::<T>(sql);

        match result {
            Ok(entities) => Ok(entities),
            Err(_) => Err(OmiError::DatabaseError),
        }
    }
}
