// SPDX-License-Identifier: MIT

#![allow(clippy::wildcard_enum_match_arm)]
#![feature(decl_macro, let_chains, if_let_guard)]

pub mod block;
pub mod common;
pub mod expr;
pub mod generics;
pub mod item;
pub mod parser;
pub mod restrictions;
pub mod stmt;
pub mod type_;

pub use crate::parser::Parser;
pub use crate::restrictions::*;

pub fn parse_root(parser: &mut crate::parser::Parser) {
    parser.start_node(metal_ast::N![Root]);

    crate::block::parse_block_stmts(parser);

    // collect trivia at the end of the file
    while let Some(ws_token) = parser.next_raw() {
        parser.token(ws_token);
    }

    parser.end_node();
}
