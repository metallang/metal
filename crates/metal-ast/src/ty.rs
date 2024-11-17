// SPDX-License-Identifier: MIT

use metal_lexer::Spanned;

use crate::Ident;

// TODO: references/pointers, arrays, etc...

/// A "type expression".
#[derive(Debug, Spanned)]
pub enum Ty<'src> {
    /// A type identifier, like `i32` in `items: Vec<i32>`.
    Ident(Box<Ident<'src>>),
}
