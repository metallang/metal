use crate::Ident;

/// A "type expression".
pub enum Ty<'src> {
    /// A type identifier, like `i32` in `items: Vec<i32>`.
    Ident(Box<Ident<'src>>),
}
