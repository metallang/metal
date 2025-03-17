// SPDX-License-Identifier: MIT

#![allow(clippy::wildcard_enum_match_arm)]
#![feature(decl_macro)]

use std::str::FromStr;

use ungrammar::Grammar;

use crate::{
    generate::{
        nodes::generate_nodes_file, syntax_kind::generate_syntax_kind_file,
        tokens::generate_tokens_file,
    },
    utils::save_generated,
};

const GRAMMAR: &str = include_str!("../metal.ungram");
const SYNTAX_KIND: &str = "../metal-ast-ng/src/syntax_kind.rs";
const TOKENS: &str = "../metal-ast-ng/src/tokens.rs";
const NODES: &str = "../metal-ast-ng/src/nodes.rs";

mod debug;
mod engram;
mod error;
mod generate;
mod grammar_item;
mod utils;

pub use crate::error::Error;

/// Runs the AST generator.
pub fn generate_ast() -> Result<(), Error> {
    if !std::fs::exists("./src/")? {
        return Err(Error::InvalidInvocation);
    }

    let grammar = Grammar::from_str(GRAMMAR)?.into();

    save_generated(generate_syntax_kind_file(&grammar), SYNTAX_KIND)?;
    save_generated(generate_tokens_file(&grammar), TOKENS)?;
    save_generated(generate_nodes_file(&grammar), NODES)?;

    Ok(())
}
