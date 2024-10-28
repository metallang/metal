use metal_lexer::{spanned, Spanned};

use crate::item::Item;

/// An identifier.
#[spanned]
#[derive(Spanned)]
pub struct Ident<'src> {
    pub inner: &'src str,
}

/// A block of code.
#[spanned]
#[derive(Spanned)]
pub struct Block<'src> {
    pub statements: Vec<Item<'src>>,
}
