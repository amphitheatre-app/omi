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
use omi::Database;

#[derive(Debug, Entity)]
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
fn test_get_one() {
    let db = Database::new("mysql://root:123456@omi".into());
    let ret = Product::find().execute(&db);
}
