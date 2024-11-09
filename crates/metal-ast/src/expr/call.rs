use metal_lexer::{spanned, Spanned};

use crate::Expr;

/// A call expression, such as `print("Hello, World!")`.
#[spanned]
#[derive(Debug, Spanned)]
pub struct CallExpr<'src> {
    /// The function expression, such as `std.io.print` or `(self.callbacks.get("ready"))`.
    pub callee: Expr<'src>,
    /// The inputs to the function, also called arguments.
    pub inputs: Vec<Expr<'src>>,
}
