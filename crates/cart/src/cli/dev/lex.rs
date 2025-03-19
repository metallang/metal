// SPDX-License-Identifier: MIT

use crate::error::Error;

pub struct DevLexCommand {
    path: String,
}

impl tapcli::Command for DevLexCommand {
    type Error = Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let Some(tapcli::Arg::Value(path)) = parser.next() else {
            panic!("expected a file path");
        };

        Ok(Self { path })
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        let contents = std::fs::read_to_string(self.path).unwrap();

        for token in metal_lexer::Lexer::new(&contents) {
            eprintln!("{token:?}");
        }

        Ok(())
    }
}
