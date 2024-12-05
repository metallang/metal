// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::types::{visibility::Visibility, Type};

/// Represents the fields of a struct, i.e. `a: B`.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct StructField {
    /// The name of the struct field, i.e. `a`.
    pub name: String,
    /// The type of this struct field, i.e. `String`.
    pub ty: Type,
}

/// Represents a struct in Metal, i.e.
///
/// ```metal
/// pub struct A {
///     b: String,
///     pub c: i32
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Struct {
    /// The name of this struct.
    pub name: String,
    /// The fields of this struct.
    pub fields: Vec<StructField>,
    /// The visibility of this struct.
    pub vis: Visibility,
}
