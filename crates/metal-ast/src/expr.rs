use crate::Ident;

/// An expression, also called a value.
pub enum Expr<'src> {
    /// An identifier in place of an expression, like `arg` in `print(arg)`.
    Ident(Ident<'src>),
    // TODO: represent this some other way
    /// A number value, like `1` or `2.6`.
    Number(Ident<'src>)
}
