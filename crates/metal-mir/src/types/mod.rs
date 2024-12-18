// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::struct_;

pub mod array;
pub mod function;
pub mod primitives;
pub mod tuple;
pub mod visibility;

/// Represents a Metal type.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Type {
    Composite(Box<Composite>),
    Primitive(Box<primitives::Primitive>),
    Function(Box<function::FunctionSignature>),
    Struct(Box<struct_::Struct>),
}

/// Represents an array type or a group of types.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Composite {
    Array(array::Array),
    Tuple(tuple::Tuple),
}
