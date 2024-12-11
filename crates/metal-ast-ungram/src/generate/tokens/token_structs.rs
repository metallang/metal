use proc_macro2::TokenStream;

use crate::{engram::Engram, generate::utils::generate_grammar_item_struct};

/// Generates token structs.
pub fn generate_token_structs(grammar: &Engram) -> impl Iterator<Item = TokenStream> + use<'_> {
    grammar.tokens().map(generate_grammar_item_struct)
}
