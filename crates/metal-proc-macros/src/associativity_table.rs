// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;

mod parse;
mod paste;
mod types;

// associativity_table! {
//     [.] as left
//     [**] as right
//     [*, /, %] as left
//     [+, -] as left
//     [<<, >>] as left
//     [&] as left
//     [^] as left
//     [|] as left
// }

pub fn associativity_table(input: TokenStream) -> TokenStream {
    let table = parse::parse_associativity_table(input);

    paste::paste_table(table)
}
