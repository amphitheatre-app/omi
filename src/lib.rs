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

mod statement;
pub use crate::statement::*;

pub mod entity;
pub mod operations;

mod errors;
pub use crate::errors::*;

mod database;
pub use crate::database::*;

mod builder;

pub mod prelude {
    pub use omi_macros::{Creatable, Deletable, Entity, Queryable, Updatable};

    pub use crate::operations::{Creatable, Deletable, Queryable, Updatable};
}

pub mod order {
    #[derive(Debug, PartialEq)]
    pub enum Direction {
        Asc,
        Desc,
    }
}
