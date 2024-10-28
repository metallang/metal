use metal_lexer::{spanned, Spanned};

use super::fn_def::FnDefStmt;
use crate::{misc::Ident, Ty};

/// A struct definition, such as `struct Vec { ... }`.
#[spanned]
#[derive(Spanned)]
pub struct StructDef<'src> {
    /// See [Ident].
    pub ident: Ident<'src>,
    /// See [StructItems].
    pub items: StructItems<'src>,
}

#[spanned]
#[derive(Spanned)]
pub struct StructItems<'src> {
    pub items: Vec<StructItem<'src>>,
}

#[derive(Spanned)]
pub enum StructItem<'src> {
    Field(StructField<'src>),
    FnDef(FnDefStmt<'src>),
}

#[spanned]
#[derive(Spanned)]
pub struct StructField<'src> {
    pub ident: Ident<'src>,
    pub ty: Ty<'src>,
}
