use crate::struct_;

pub mod array;
pub mod function;
pub mod primitives;
pub mod tuple;
pub mod visibility;

/// Represents a Metal type.
#[derive(Debug, Clone)]
pub enum Type {
    Composite(Box<Composite>),
    Primitive(Box<primitives::Primitive>),
    Function(Box<function::FunctionSignature>),
    Struct(Box<struct_::Struct>),
}

/// Represents an array type or group of types.
#[derive(Debug, Clone)]
pub enum Composite {
    Array(array::Array),
    Tuple(tuple::Tuple),
}
