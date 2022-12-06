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

//! Provides functions for database operations

use crate::{Entity, Ops, Statement};

/// Begin a query session and returns a Statement instance.
pub fn find<T>() -> Statement<T>
where
    T: Entity + Default,
{
    // Create a new Statement instance and return it
    Statement::new(Ops::Select)
}

/// Begin a insert session, which accepts an entity and returns a Statement
/// instance
pub fn create<T>(entity: T) -> Statement<T>
where
    T: Entity + Default,
{
    // Create a new Statement instance and return it
    Statement::new(Ops::Insert)
}

/// Begin a update session, which accepts an optional entity and returns a
/// Statement instance. There are two usages:
///
/// 1. Call with entity instance:
/// Omi will automatically recognize the model of this instance and compare the
/// differences, saving only the changed part of the fields to the database.
///
/// 2. Call without entity instance:
/// It's mean you want to update the specified fields raw, this way, you need to
/// call it like with T: `update::<T>()`, for tell Omi that what entity you specified.
pub fn update<T>(entity: Option<T>) -> Statement<T>
where
    T: Entity + Default,
{
    // Create a new Statement instance and return it
    Statement::new(Ops::Update)
}

/// Begin a delete session, which accepts an entity and returns a Statement instance
pub fn delete<T>(entity: T) -> Statement<T>
where
    T: Entity + Default,
{
    // Create a new Statement instance and return it
    Statement::new(Ops::Delete)
}

#[cfg(test)]
mod test {
    use crate::{create, delete, find, update, Entity, Ops};

    #[derive(Default)]
    struct Product {}

    impl Entity for Product {
        fn meta(&self) -> crate::Meta {
            todo!()
        }
    }

    #[test]
    fn test_find() {
        let session = find::<Product>();
        assert_eq!(session.ops(), &Ops::Select);
    }

    #[test]
    fn test_create() {
        let session = create(Product::default());
        assert_eq!(session.ops(), &Ops::Insert);
    }

    #[test]
    fn test_update() {
        let session = update(Some(Product::default()));
        assert_eq!(session.ops(), &Ops::Update);
    }

    #[test]
    fn test_delete() {
        let session = delete(Product::default());
        assert_eq!(session.ops(), &Ops::Delete);
    }
}
