use crate::misc::Ident;

/// An expression, also called a value.
pub enum Expr<'src> {
    /// An identifier in place of an expression, like `arg` in `print(arg)`.
    Ident(Ident<'src>),
}
