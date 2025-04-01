// SPDX-License-Identifier: MIT

#![allow(clippy::wildcard_enum_match_arm)]

mod associativity_table;

#[proc_macro]
pub fn associativity_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::associativity_table::associativity_table(input.into()).into()
}
