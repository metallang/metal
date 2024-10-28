use metal_lexer::{spanned, Spanned};

use crate::{
    item::FnDef,
    misc::{Ident, Visibility},
    Ty,
};

/// A struct definition, such as `struct Vec { ... }`.
#[spanned]
#[derive(Spanned)]
pub struct StructDef<'src> {
    /// See [Visibility].
    pub vis: Visibility,
    /// See [Ident].
    pub ident: Ident<'src>,
    /// See [StructItems].
    pub items: StructItems<'src>,
}

/// Items of a [StructDef], such as fields and associated functions.
#[spanned]
#[derive(Spanned)]
pub struct StructItems<'src> {
    /// See [StructItem].
    pub items: Vec<StructItem<'src>>,
}

/// A "struct item", such as a field or an associated function.
#[derive(Spanned)]
pub enum StructItem<'src> {
    /// See [StructField].
    Field(StructField<'src>),
    /// A [FnDef] associated with this struct.
    FnDef(FnDef<'src>),
}

/// A [StructDef]'s field, such as `pub enable_logging: bool`.
#[spanned]
#[derive(Spanned)]
pub struct StructField<'src> {
    /// See [Visibility].
    pub vis: Visibility,
    /// See [Ident].
    pub ident: Ident<'src>,
    /// See [Ty].
    pub ty: Ty<'src>,
}
