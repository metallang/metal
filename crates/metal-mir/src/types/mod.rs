// SPDX-License-Identifier: MIT

use rkyv::{Archive, Deserialize, Serialize};

use crate::struct_;

pub mod array;
pub mod function;
pub mod primitives;
pub mod tuple;
pub mod visibility;

/// Represents a Metal type.
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[rkyv(compare(PartialEq), derive(Debug, Clone))]
pub enum Type {
    Composite(Box<Composite>),
    Primitive(Box<primitives::Primitive>),
    Function(Box<function::FunctionSignature>),
    Struct(Box<struct_::Struct>),
}

/// Represents an array type or a group of types.
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[rkyv(compare(PartialEq), derive(Debug, Clone))]
pub enum Composite {
    Array(array::Array),
    Tuple(tuple::Tuple),
}
