// SPDX-License-Identifier: MIT

#![feature(new_range_api)]

mod lexer;
mod span;
mod token;

pub use crate::lexer::Lexer;
pub use crate::span::Span;
pub use crate::token::Token;
