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

use super::Driver;

pub struct SqliteDriver {
    // FIXME: Real Connection
    // connection: SqliteConnection,
}

impl Driver for SqliteDriver {
    fn connect(&mut self) {
        todo!()
    }

    fn disconnect(&mut self) {
        todo!()
    }

    fn execute(&self, _query: String) -> Result<(), crate::OmiError> {
        todo!()
    }
}
