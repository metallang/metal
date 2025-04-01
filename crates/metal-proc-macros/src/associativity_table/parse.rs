use proc_macro2::{Delimiter, TokenStream, TokenTree};

use super::types::{Table, TableEntry, TableEntryAssociativity, TableEntryFlavor};

type Input = std::iter::Peekable<proc_macro2::token_stream::IntoIter>;

pub fn parse_associativity_table(input: TokenStream) -> Table {
    let mut input = input.into_iter().peekable();
    let mut table = Table { entries: vec![] };

    while input.peek().is_some() {
        table.entries.push(parse_table_entry(&mut input));
    }

    table
}

pub fn parse_table_entry(input: &mut Input) -> TableEntry {
    let tokens = parse_table_entry_tokens(input);

    let flavor = parse_table_entry_flavor(input);

    let assoc = parse_table_entry_assoc(input);

    TableEntry {
        tokens,
        flavor,
        assoc,
    }
}

pub fn parse_table_entry_flavor(input: &mut Input) -> TableEntryFlavor {
    match input.next().unwrap() {
        TokenTree::Ident(ident) if ident == "prefix" => TableEntryFlavor::Prefix,
        TokenTree::Ident(ident) if ident == "infix" => TableEntryFlavor::Infix,
        TokenTree::Ident(ident) if ident == "postfix" => TableEntryFlavor::Postfix,
        other => panic!("expected a flavor, got {other:#?}"),
    }
}

pub fn parse_table_entry_tokens(input: &mut Input) -> Vec<TokenStream> {
    let mut tokens = vec![];

    loop {
        tokens.push(parse_table_entry_token(input));

        match input.peek() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '|' => input.next(),
            _ => break,
        };
    }

    tokens
}

pub fn parse_table_entry_token(input: &mut Input) -> TokenStream {
    match input.next().unwrap() {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => group.stream(),
        other => panic!("expected a bracketed token, got {other:#?}"),
    }
}

pub fn parse_table_entry_assoc(input: &mut Input) -> TableEntryAssociativity {
    match input.next().unwrap() {
        TokenTree::Ident(ident) if ident == "left" => TableEntryAssociativity::Left,
        TokenTree::Ident(ident) if ident == "right" => TableEntryAssociativity::Right,
        TokenTree::Ident(ident) if ident == "other" => TableEntryAssociativity::Other,
        other => panic!("expected either `left`, `right` or `other`, got {other:#?}"),
    }
}
