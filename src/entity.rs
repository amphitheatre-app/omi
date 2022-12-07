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

pub trait Entity {
    fn meta() -> Meta;
}

#[derive(Debug, Default)]
pub struct Meta {
    pub table: String,
}

#[cfg(test)]
mod test {

    use omi::prelude::*;

    use crate as omi;

    #[derive(Debug, Default, Clone, Entity)]
    #[entity(table = "products")]
    struct Product {}

    #[test]
    fn test_entity_meta() {
        assert_eq!(Product::meta().table, "products");
    }
}
