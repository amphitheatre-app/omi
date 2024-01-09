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

use std::marker::PhantomData;

use super::Statement;
use crate::builder::Builder;
use crate::model::Entity;
use crate::{Database, OmiError, Result};

// Represents a database raw operation statement.
#[derive(Clone)]
pub struct RawStatement<T> {
    pub(crate) entity: PhantomData<T>,
    pub(crate) sql: String,
}

impl<T> RawStatement<T>
where
    T: Entity + Default + From<T> + Clone,
{
    pub fn new(sql: String) -> Self {
        Self {
            entity: PhantomData,
            sql,
        }
    }

    /// Implement the execute() method for the Statement type
    pub fn execute(&self, db: &Database) -> Result<Vec<T>> {
        let sql = Builder::build(Statement::Raw(self.clone()));
        let result = db.execute::<T>(sql);

        match result {
            Ok(entities) => Ok(entities),
            Err(_) => Err(OmiError::DatabaseError),
        }
    }
}
