use metal_lexer::{spanned, Spanned};

use crate::misc::Ident;

/// A struct definition statement, like `struct Vec { ... }`.
#[spanned]
#[derive(Spanned)]
pub struct StructDefStmt<'src> {
    /// See [Ident].
    pub ident: Ident<'src>,
    /// See [Block].
    pub items: Items<'src>,
}
