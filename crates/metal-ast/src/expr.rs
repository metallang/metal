// SPDX-License-Identifier: MIT

pub mod call;
pub mod lit;

use metal_lexer::Spanned;

use crate::{CallExpr, Ident, LitExpr};

/// An expression, also called a value, such as `1 + 1` or `fib(20)`.
#[derive(Debug, Spanned)]
pub enum Expr<'src> {
    /// An identifier in place of an expression, like `arg` in `print(arg)`.
    Ident(Box<Ident<'src>>),
    /// See [LitExpr].
    Lit(Box<LitExpr<'src>>),
    /// See [CallExpr]. Note: In the future, this will be represented as a variant of a binary expression.
    Call(Box<CallExpr<'src>>),
}
