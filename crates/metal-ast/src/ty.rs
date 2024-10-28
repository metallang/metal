use crate::Ident;

/// A "type expression".
pub enum Ty<'src> {
    /// An identifier, like `i32` in `items: Vec<i32>`.
    Ident(Box<Ident<'src>>),
}
