use crate::Ident;

/// A "type expression".
pub enum Ty<'src> {
    /// An identifier, like `i32` in `pub items: Vec<i32> := Vec.new()`.
    Ident(Ident<'src>),
}
