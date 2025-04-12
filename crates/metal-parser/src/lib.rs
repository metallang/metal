// SPDX-License-Identifier: MIT

#![allow(clippy::wildcard_enum_match_arm)]
#![feature(decl_macro, let_chains, if_let_guard)]

mod block;
mod common;
mod expr;
mod generics;
mod item;
mod parser;
mod restrictions;
mod stmt;
mod type_;

pub use crate::parser::Parser;
pub use crate::restrictions::*;

pub fn parse_root(parser: &mut crate::parser::Parser) {
    parser.start_node(metal_ast::N![Root]);

    crate::block::parse_block_stmts(parser);

    parser.end_node();
}
