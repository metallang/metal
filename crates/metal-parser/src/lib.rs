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

pub fn parse_root(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![Root]);

    // after construction, the parser will return None as the first token
    // for internal reasons. we therefore skip it here
    parser.next();

    parse_block_stmts(parser);

    parser.end_node();
}
