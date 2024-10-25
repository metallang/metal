use crate::Ident;

// TODO: references/pointers, arrays, etc...

/// A "type expression".
pub enum Ty<'src> {
    /// An identifier, like `i32` in `pub items: Vec<i32> := Vec.new()`.
    Ident(Ident<'src>),
    Primitive(Primitives),
}

// TODO: maybe split this up?
pub enum Primitives<'src> {
    // Integers
    I8,
    I16,
    I32,
    I64,
    I128,

    // Floats
    F16,
    F32,
    F64,
    F128,

    //
    String,

    // Etc...
    Void
}
