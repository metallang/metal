// SPDX-License-Identifier: MIT

#![feature(decl_macro)]

mod block;
mod common;
mod expr;
mod item;
mod parser;
mod type_;
mod utils;

pub use block::parse_block_items as parse_root;
pub use parser::Parser;
