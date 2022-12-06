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

use omi::{Database, Entity, Queryable};

#[derive(Debug, Entity)]
struct Product {
    title: String,
}

impl Default for Product {
    fn default() -> Self {
        Self {
            title: "test".into(),
        }
    }
}

// impl Entity for Product {
//     fn meta(&self) -> omi::Meta {
//         omi::Meta::default()
//     }
// }

#[test]
fn test_find_one() {
    let db = Database::new("mysql://root:123456@omi".into());
    let product = omi::find::<Product>().execute(&db);

    assert_eq!(product, Ok(Product::default()));
}

#[test]
fn test_find_all() {
    let db = Database::new("mysql://root:123456@omi".into());
    let products = omi::find::<Product>().execute(&db);
    assert_eq!(products, Ok(vec![]))
}

#[test]
fn test_entity_get_one() {
    let db = Database::new("mysql://root:123456@omi".into());
    let ret = Product::get().execute(&db);
}
