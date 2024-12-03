use std::str::FromStr;

use ungrammar::Grammar;

use crate::{
    generate::{
        nodes::generate_node_items, syntax_kind::generate_syntax_kind, tokens::generate_token_items,
    },
    utils::save_generated,
};

const GRAMMAR: &str = include_str!("../../metal-ast-ng/metal.ungram");
const SYNTAX_KIND: &str = "./src/syntax_kind.rs";
const TOKENS: &str = "./src/tokens.rs";
const NODES: &str = "./src/nodes.rs";

mod debug;
mod engram;
mod error;
mod generate;
mod utils;

pub use crate::error::Error;

pub fn run() -> Result<(), Error> {
    if !std::fs::exists("./src/")? {
        return Err(Error::InvalidInvocation);
    }

    let grammar = Grammar::from_str(GRAMMAR)?.into();

    save_generated(generate_syntax_kind(&grammar), SYNTAX_KIND)?;
    save_generated(generate_token_items(&grammar), TOKENS)?;
    save_generated(generate_node_items(&grammar), NODES)?;

    Ok(())
}
