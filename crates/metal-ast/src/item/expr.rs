use metal_lexer::{spanned, Spanned};

use crate::Expr;

/// A standalone expression, like `print("Hello, World!")`.
#[spanned]
#[derive(Spanned)]
pub struct ExprStmt<'src> {
    /// See [Expr].
    pub expr: Expr<'src>,
}
