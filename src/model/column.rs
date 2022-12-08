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

#[derive(Debug, Default)]
pub struct Column {
    /// The name of the column.
    pub name: String,
    /// The data kind of the column.
    pub kind: DataKind,
    /// Whether the column allows NULL values.
    pub null: bool,
    /// Whether the column is a primary key.
    pub primary: bool,
    /// The default value for the column.
    pub default: Option<String>,
    /// Whether the column is set to AUTO_INCREMENT.
    pub auto: bool,
    /// Whether the column is unique.
    pub unique: bool,
    /// Whether the column is set to indexed.
    pub index: bool,
}

#[derive(Debug)]
pub enum DataKind {
    /// An integer data type.
    Integer,
    /// A floating-point number data type.
    Float,
    /// A text data type.
    Text,
    /// A binary data type.
    Blob,
    /// A timestamp data type.
    Timestamp,
    /// A date data type.
    Date,
    /// A time data type.
    Time,
    /// A datetime data type.
    Datetime,
    /// A year data type.
    Year,
    /// A boolean data type
    Boolean,
}

impl Default for DataKind {
    fn default() -> Self {
        unreachable!()
    }
}
