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

use crate::{Ops, Statement};

pub trait Entity {
    fn meta(&self) -> Meta;
}

#[derive(Debug, Default)]
pub struct Meta {
    pub table: String,
}

pub trait Queryable<T> {
    fn get() -> Statement<T>;
    fn find() -> Statement<T>;
}

impl <T> Queryable<T> for T 
where T : Entity + Default
{
    fn get() -> Statement<T> {
        Statement::new(Ops::Select)
    }

    fn find() -> Statement<T> {
        Statement::new(Ops::Select)
    }
}

#[cfg(test)]
mod test {
    #[derive(Default)]
    struct Product {
        title: String,
    }

    #[test]
    fn test_entity_trait_meta() {
        let product = Product::default();
        assert!(Product::meta());
    }
}
