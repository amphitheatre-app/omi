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

use super::Statement;
use crate::builder::*;
use crate::model::Entity;
use crate::{Database, OmiError, Result};

/// Represents a database INSERT operation statement.
#[derive(Clone, Copy)]
pub struct InsertStatement<T> {
    pub(crate) entity: T,
}

impl<T> InsertStatement<T>
where
    T: Entity + Default + From<T> + Clone,
{
    pub fn new(entity: T) -> Self {
        Self { entity }
    }

    /// Implement the execute() method for the Statement type
    pub fn execute(&self, db: &Database) -> Result<Vec<T>> {
        let sql = Builder::build(Statement::Insert(self.clone()));
        let result = db.execute::<T>(sql);

        match result {
            Ok(entities) => Ok(entities),
            Err(_) => Err(OmiError::DatabaseError),
        }
    }
}
