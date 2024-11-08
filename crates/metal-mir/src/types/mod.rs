pub mod array;
pub mod function;
pub mod primitives;
pub mod tuple;
pub mod visibility;

/// Represents a Metal type.
///
/// Methods and other such functionality for
/// base types are provided by the corelib.
#[derive(Debug, Clone)]
pub enum Type {
    Composite(Box<Composite>),
    Primitive(Box<primitives::Primitive>),
    Function(Box<function::FunctionSignature>),
}

/// Represents an array type or group of types.
#[derive(Debug, Clone)]
pub enum Composite {
    Array(array::Array),
    Tuple(tuple::Tuple),
}
