use metal_lexer::{spanned, Spanned};

use crate::Expr;

/// A standalone expression, such as `print("Hello, World!")` inside a function.
#[spanned]
#[derive(Spanned)]
pub struct ExprItem<'src> {
    /// See [Expr].
    pub expr: Expr<'src>,
}
