use metal_lexer::{spanned, Spanned};

use crate::{Block, Expr, Ident, Ty, Visibility};

/// A function definition, such as `def main() {}`.
#[spanned]
#[derive(Spanned)]
pub struct FnDef<'src> {
    pub vis: Visibility,
    /// See [Ident].
    pub ident: Ident<'src>,
    /// See [FnInput].
    pub inputs: Vec<FnInput<'src>>,
    /// See [Ty].
    pub return_type: Option<Ty<'src>>,
    /// See [Block].
    pub body: Block<'src>,
}

/// A function input, also called a parameter, such as `yes: bool = True`.
#[spanned]
#[derive(Spanned)]
pub struct FnInput<'src> {
    /// See [Ident].
    pub ident: Ident<'src>,
    /// See [Ty].
    pub ty: Ty<'src>,
    /// See [Expr].
    pub default: Option<Expr<'src>>,
}
