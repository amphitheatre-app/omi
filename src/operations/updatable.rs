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

use crate::entity::Entity;
use crate::{Ops, Statement};

pub trait Updatable<T> {
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
    fn update(entity: Option<T>) -> Statement<T>;
}

impl<T> Updatable<T> for T
where
    T: Entity + Default,
{
    fn update(entity: Option<T>) -> Statement<T> {
        Statement::new(Ops::Update)
    }
}
