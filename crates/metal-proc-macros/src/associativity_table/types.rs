// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;

#[derive(Debug)]
pub struct Table {
    pub entries: Vec<TableEntry>,
}

#[derive(Debug)]
pub struct TableEntry {
    pub tokens: Vec<TokenStream>,
    pub flavor: TableEntryFlavor,
    pub assoc: TableEntryAssociativity,
}

#[derive(Debug)]
pub enum TableEntryAssociativity {
    Left,
    Right,
    Other,
}

#[derive(Debug)]
pub enum TableEntryFlavor {
    Prefix,
    Infix,
    Postfix,
}
