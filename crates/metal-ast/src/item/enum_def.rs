use metal_lexer::{spanned, Spanned};

use crate::{FnDef, Ident, Ty, Visibility};

/// A enum definition, such as `enum IpAddress { ... }`.
#[spanned]
#[derive(Spanned)]
pub struct EnumDef<'src> {
    /// See [Visibility].
    pub vis: Visibility,
    /// See [Ident].
    pub ident: Ident<'src>,
    /// See [EnumItems].
    pub items: EnumItems<'src>,
}

/// Items of an [EnumDef], such as variants and associated functions.
#[spanned]
#[derive(Spanned)]
pub struct EnumItems<'src> {
    /// See [EnumItem].
    pub items: Vec<EnumItem<'src>>,
}

/// A "enum item", such as a variant or an associated function.
#[derive(Spanned)]
pub enum EnumItem<'src> {
    /// See [EnumVariant].
    Variant(EnumVariant<'src>),
    /// A [FnDef] associated with this enum.
    FnDef(FnDef<'src>),
}

/// A [EnumDef]'s variant, such as `Some(T)`.
#[spanned]
#[derive(Spanned)]
pub struct EnumVariant<'src> {
    /// See [Ident].
    pub ident: Ident<'src>,
    /// The variant's associated data's type, if any.
    pub datatype: Option<Ty<'src>>,
}
