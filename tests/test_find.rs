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

use omi::prelude::*;
use omi::{Database, OmiError};

#[derive(Debug, Entity, Queryable, PartialEq, Clone)]
#[entity(table = "products")]
struct Product {
    #[column(length = 255, default = "")]
    title: String,
}

impl Default for Product {
    fn default() -> Self {
        Self {
            title: "test".into(),
        }
    }
}

#[test]
fn test_find_one() {
    let db = Database::new("mysql://root:123456@omi".into());
    let result = Product::find().one(&db);

    match result {
        Ok(product) => assert_eq!(product, Product::default()),
        Err(error) => assert_eq!(error, OmiError::NotFoundError),
    }
}

#[test]
fn test_find_all() {
    let db = Database::new("mysql://root:123456@omi".into());
    let result = Product::find().all(&db);

    match result {
        Ok(products) => assert!(products.is_empty()),
        Err(error) => assert_eq!(error, OmiError::NotFoundError),
    }
}
