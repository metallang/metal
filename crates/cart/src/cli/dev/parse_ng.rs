// SPDX-License-Identifier: MIT

use crate::error::Error;

pub struct DevParseNgCommand {
    path: String,
}

impl tapcli::Command for DevParseNgCommand {
    type Error = Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let Some(tapcli::Arg::Value(path)) = parser.next() else {
            panic!("expected a file path");
        };

        Ok(Self { path })
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        let contents = std::fs::read_to_string(self.path).unwrap();

        let lexer = metal_lexer_ng::Lexer::new(&contents).peekable();
        let mut parser = metal_parser_ng::Parser::new(lexer, &contents);

        metal_parser_ng::parse_root(&mut parser);

        eprintln!("{:#?}", parser.finish());

        Ok(())
    }
}
