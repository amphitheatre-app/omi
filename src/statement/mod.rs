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

mod delete_statement;
pub use self::delete_statement::DeleteStatement;

mod insert_statement;
pub use self::insert_statement::InsertStatement;

mod raw_statement;
pub use self::raw_statement::RawStatement;

mod select_statement;
pub use self::select_statement::SelectStatement;

mod update_statement;
pub use self::update_statement::UpdateStatement;

// Declare a type named Statement, which represents a database operation statement.
pub enum Statement<T> {
    Delete(DeleteStatement<T>),
    Insert(InsertStatement<T>),
    Raw(RawStatement<T>),
    Select(SelectStatement<T>),
    Update(UpdateStatement<T>),
}

#[derive(Clone, Copy)]
pub struct Filters {}
