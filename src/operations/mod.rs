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

mod creatable;
pub use creatable::*;

mod deletable;
pub use deletable::*;

mod queryable;
pub use queryable::*;

mod updatable;
pub use updatable::*;

use crate::Statement;

/// You can use the `raw()` method for edge cases where existing mechanisms
/// or interfaces don't meet your needs, and it will still provide you with
/// object mapping support.
pub fn raw<T>(_sql: String) -> Statement<T> {
    todo!()
}
