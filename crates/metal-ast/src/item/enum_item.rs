use metal_lexer::{spanned, Spanned};

use crate::{FnItem, Ident, Ty, Visibility};

/// An enum definition, such as `enum IpAddress { ... }`.
#[spanned]
#[derive(Spanned)]
pub struct EnumItem<'src> {
    /// See [Visibility].
    pub vis: Visibility,
    /// See [Ident].
    pub ident: Ident<'src>,
    /// See [EnumBody].
    pub body: EnumBody<'src>,
}

/// Insides of an [EnumItem], such as variants and associated functions.
#[spanned]
#[derive(Spanned)]
pub struct EnumBody<'src> {
    /// See [EnumBodyItem].
    pub items: Vec<EnumBodyItem<'src>>,
}

/// An "enum body item", such as a variant or an associated function.
#[derive(Spanned)]
pub enum EnumBodyItem<'src> {
    /// See [EnumVariant].
    Variant(EnumVariant<'src>),
    /// A [FnItem] associated with this enum.
    FnItem(FnItem<'src>),
}

/// An [EnumItem]'s variant, such as `Some(T)`.
#[spanned]
#[derive(Spanned)]
pub struct EnumVariant<'src> {
    /// See [Ident].
    pub ident: Ident<'src>,
    /// The variant's associated data's type, if any.
    pub datatype: Option<Ty<'src>>,
}
