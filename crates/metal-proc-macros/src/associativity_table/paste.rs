// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;
use quote::quote;

use super::types::{Table, TableEntry, TableEntryAssociativity, TableEntryFlavor};

pub fn paste_table(table: Table) -> TokenStream {
    let mut raw_binding_power = 1u16;
    let arms = table.entries.into_iter().map(|entry| {
        raw_binding_power += 2;

        paste_table_entry(entry, raw_binding_power)
    });

    quote! {
        pub fn binding_power_for(op: SyntaxKind, flavor: Flavor) -> Option<BindingPower> {
            let bp = match op {
                #(#arms),*,
                _ => return None,
            };

            Some(bp.into())
        }
    }
}

fn paste_table_entry(entry: TableEntry, raw_binding_power: u16) -> TokenStream {
    let pattern = entry
        .tokens
        .into_iter()
        .map(|token| quote! { | T![#token] });
    let flavor = paste_table_entry_flavor(entry.flavor);
    let assoc = paste_table_entry_assoc(entry.assoc);

    quote! {
        #(#pattern)* if matches!(flavor, #flavor) => (#raw_binding_power, #assoc)
    }
}

fn paste_table_entry_assoc(assoc: TableEntryAssociativity) -> TokenStream {
    match assoc {
        TableEntryAssociativity::Left => quote! { Assoc::Left },
        TableEntryAssociativity::Right => quote! { Assoc::Right },
        TableEntryAssociativity::Other => quote! { Assoc::Inapplicable },
    }
}

fn paste_table_entry_flavor(flavor: TableEntryFlavor) -> TokenStream {
    match flavor {
        TableEntryFlavor::Prefix => quote! { Flavor::Prefix },
        TableEntryFlavor::Infix => quote! { Flavor::Infix },
        TableEntryFlavor::Postfix => quote! { Flavor::Postfix },
    }
}
