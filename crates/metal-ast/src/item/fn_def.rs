use metal_lexer::{spanned, Spanned};

use crate::{
    misc::{Block, Ident},
    Expr, Ty,
};

/// A function definition statement, like `def main() {}`.
#[spanned]
#[derive(Spanned)]
pub struct FnDefStmt<'src> {
    /// See [Ident].
    pub ident: Ident<'src>,
    /// See [FnInput].
    pub inputs: Vec<FnInput<'src>>,
    /// See [Ty].
    pub return_type: Option<Ty<'src>>,
    /// See [Block].
    pub body: Block<'src>,
}

/// A function input, also called a parameter, like `yes: bool = True`.
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
