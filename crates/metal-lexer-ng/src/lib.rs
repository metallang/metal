#![feature(new_range_api)]

mod lexer;
mod span;
mod token;
mod token_kind;

pub use crate::lexer::Lexer;
pub use crate::span::Span;
pub use crate::token_kind::TokenKind;
