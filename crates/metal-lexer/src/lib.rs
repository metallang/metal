#![feature(decl_macro, new_range_api)]

mod callbacks;
mod error;
mod span;
#[cfg(test)]
mod tests;
mod token;

use logos::Logos;
pub use metal_proc_macros::{span, spanned, Spanned};

pub use crate::error::Error;
pub use crate::span::{MaybeSpanned, Span, Spanned};
pub use crate::token::{Token, TokenKind};

pub fn lex(input: &str) -> impl Iterator<Item = Result<Token<'_>, Error>> {
    TokenKind::lexer(input).spanned().map(|(result, span)| {
        result.map(|kind| Token {
            span: span.into(),
            kind,
        })
    })
}
