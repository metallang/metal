use metal_lexer::{spanned, Spanned};

use crate::{Expr, Ident, Ty, Visibility};

/// A constant definition, such as `const PREALLOC: u8 = 128;`.
#[spanned]
#[derive(Debug, Spanned)]
pub struct ConstItem<'src> {
    /// See [Visibility].
    pub vis: Visibility,
    /// See [Ident].
    pub ident: Ident<'src>,
    /// See [Ty].
    pub ty: Option<Ty<'src>>,
    /// See [Expr].
    pub value: Expr<'src>,
}
