use metal_lexer::{spanned, Spanned};

use crate::item::Item;

/// An identifier, such as `i_am_a_variable`.
#[spanned]
#[derive(Spanned)]
pub struct Ident<'src> {
    pub inner: &'src str,
}

/// A block of code.
#[spanned]
#[derive(Spanned)]
pub struct Block<'src> {
    /// See [Item].
    pub items: Vec<Item<'src>>,
}

/// See [VisibilityKind].
#[spanned]
#[derive(Spanned)]
pub struct Visibility {
    /// See [VisibilityKind].
    pub vis: VisibilityKind,
}

/// An item's visibility.
pub enum VisibilityKind {
    /// Public visibility.
    Public,
    /// Inherited visibility, default, usually private.
    Inherited,
}
