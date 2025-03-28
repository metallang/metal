// SPDX-License-Identifier: MIT

#![allow(clippy::wildcard_enum_match_arm)]
#![feature(decl_macro, let_chains, if_let_guard)]

mod block;
mod common;
mod expr;
mod generics;
mod item;
mod parser;
mod stmt;
mod type_;

use metal_ast::N;

use crate::block::parse_block_stmts;
pub use crate::parser::Parser;

pub fn parse_root(parser: &mut crate::parser::Parser) {
    parser.start_node(N![Root]);

    parse_block_stmts(parser);

    parser.end_node();
}
