use metal_lexer::{spanned, Spanned};

use crate::{FnItem, Ident, Ty, Visibility};

/// A struct definition, such as `struct Vec { ... }`.
#[spanned]
#[derive(Spanned)]
pub struct StructItem<'src> {
    /// See [Visibility].
    pub vis: Visibility,
    /// See [Ident].
    pub ident: Ident<'src>,
    /// See [StructBody].
    pub body: StructBody<'src>,
}

/// Insides of a [StructItem], such as fields and associated functions.
#[spanned]
#[derive(Spanned)]
pub struct StructBody<'src> {
    /// See [StructBodyItem].
    pub items: Vec<StructBodyItem<'src>>,
}

/// A "struct body item", such as a field or an associated function.
#[derive(Spanned)]
pub enum StructBodyItem<'src> {
    /// See [StructField].
    Field(StructField<'src>),
    /// A [FnItem] associated with this struct.
    FnItem(FnItem<'src>),
}

/// A [StructItem]'s field, such as `pub enable_logging: bool`.
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
