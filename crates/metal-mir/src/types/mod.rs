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
}

#[derive(Debug, Clone)]
pub enum Composite {
    Array(array::Array),
    Tuple(tuple::Tuple),
}
