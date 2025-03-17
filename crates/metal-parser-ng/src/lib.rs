// SPDX-License-Identifier: MIT

#![allow(clippy::wildcard_enum_match_arm)]
#![feature(decl_macro, let_chains, if_let_guard)]

mod block;
mod common;
mod expr;
mod item;
mod parser;
mod type_;

pub use block::parse_block_items as parse_root;
pub use parser::Parser;
