use crate::{Ident, Path, Ty};

/// An expression, also called a value.
pub enum Expr<'src> {
    /// An identifier in place of an expression, like `arg` in `print(arg)`.
    Ident(Ident<'src>),
    // TODO: represent this some other way
    /// A number value, like `1` or `2.6`.
    Number(NumberExpr<'src>),
    // A function call
    FnCall(FnCallExpr<'src>),
}

pub struct NumberExpr<'src> {
    pub ty: Ty<'src>,
    pub value: i64,
}

pub struct FnCallExpr<'src> {
    pub fn_name: Ident<'src>,
    pub arguments: Vec<Expr<'src>>,
    pub module_name: Path<'src>,
}
