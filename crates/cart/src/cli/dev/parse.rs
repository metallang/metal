// SPDX-License-Identifier: MIT

use crate::error::Error;

pub struct DevParseCommand {
    path: String,
}

impl tapcli::Command for DevParseCommand {
    type Error = Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let Some(tapcli::Arg::Value(path)) = parser.next() else {
            panic!("expected a file path");
        };

        Ok(Self { path })
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        let contents = std::fs::read_to_string(self.path).unwrap();

        let mut tokens = metal_lexer::lex(&contents)
            .filter_map(Result::ok)
            .peekable();

        let ast = metal_parser::parse_block_raw(&mut tokens);

        eprintln!("{:#?}", ast);

        Ok(())
    }
}
