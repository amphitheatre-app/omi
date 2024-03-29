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

use crate::statement::DeleteStatement;

pub trait Deletable<T> {
    /// Begin a delete session, which accepts an entity and returns a Statement instance
    fn delete(entity: T) -> DeleteStatement<T>;
}

#[cfg(test)]
mod test {
    use omi::prelude::*;

    use crate as omi;

    #[derive(Debug, Default, Clone, Entity, Deletable, PartialEq)]
    #[entity(table = "products")]
    struct Product {}

    #[test]
    fn test_deletable_delete() {
        let statement = Product::delete(Product::default());
        assert_eq!(statement.entity, Product::default());
    }
}
