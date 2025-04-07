// SPDX-License-Identifier: MIT

#![allow(clippy::wildcard_enum_match_arm)]

mod associativity_table;

/// Generates the `binding_power_for` function from an associativity table.
///
/// Each entry has a unique, odd raw binding power. Latter entries have higher precedence.
///
/// A table entry is a `|`-separated list of tokens, followed by the operator flavor, followed by the operator associativity.
///
/// This macro is ONLY meant to be used from `metal_parser::expr::bp`.
#[proc_macro]
pub fn associativity_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::associativity_table::associativity_table(input.into()).into()
}
