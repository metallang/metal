use crate::Ident;

// TODO: references/pointers, arrays, etc...

/// A "type expression".
#[derive(Eq, PartialEq)]
pub enum Ty<'src> {
    /// An identifier, like `i32` in `pub items: Vec<i32> := Vec.new()`.
    Ident(Ident<'src>),
    Primitive(Primitives),
}

// TODO: maybe split this up?
#[derive(Eq, PartialEq)]
pub enum Primitives {
    // Integers
    I8,
    I16,
    I32,
    I64,
    I128,

    // Floats
    F32,
    F64,
    F128,

    // Etc...
    Void,
    String,
}
