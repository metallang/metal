// SPDX-License-Identifier: MIT

use lex_ng::DevLexNGCommand;
use parse::DevParseCommand;

use crate::error::Error;

mod lex_ng;
mod parse;

pub enum DevCommand {
    /// Parse a Metal source file and debug-print it's AST.
    Parse(DevParseCommand),
    /// Debug-print the result of lexing the provided file.
    LexNG(DevLexNGCommand),
}

impl tapcli::Command for DevCommand {
    type Error = Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let arg = parser.next().ok_or(Error::InsufficientArguments)?;

        match arg.as_ref() {
            tapcli::ArgRef::Value("parse") => Ok(Self::Parse(DevParseCommand::parse(parser)?)),
            tapcli::ArgRef::Value("lex-ng") => Ok(Self::LexNG(DevLexNGCommand::parse(parser)?)),
            _ => Err(Error::UnrecognizedArgument(arg)),
        }
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        match self {
            DevCommand::Parse(cmd) => cmd.run(),
            DevCommand::LexNG(cmd) => cmd.run(),
        }
    }
}
