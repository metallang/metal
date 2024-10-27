use crate::{Ident, Path, Ty};

/// An expression, also called a value.
pub enum Expr<'src> {
    /// An identifier in place of an expression, like `arg` in `print(arg)`.
    Ident(Ident<'src>),
    // TODO: represent this some other way
    /// A number value, like `1` or `2.6`.
    Number { ty: Ty<'src>, value: u64 },
    // A function call
    FnCall {
        fn_name: Ident<'src>,
        arguments: Vec<Expr<'src>>,
        module_name: Path<'src>,
    },
}
