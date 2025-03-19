// SPDX-License-Identifier: MIT

use lex::DevLexCommand;
use parse::DevParseCommand;

use crate::error::Error;

mod lex;
mod parse;

pub enum DevCommand {
    /// Debug-print the result of lexing the provided file.
    Lex(DevLexCommand),
    /// Parse a Metal source file and debug-print its AST.
    Parse(DevParseCommand),
}

impl tapcli::Command for DevCommand {
    type Error = Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let arg = parser.next().ok_or(Error::InsufficientArguments)?;

        match arg.as_ref() {
            tapcli::ArgRef::Value("lex") => Ok(Self::Lex(DevLexCommand::parse(parser)?)),
            tapcli::ArgRef::Value("parse") => Ok(Self::Parse(DevParseCommand::parse(parser)?)),
            _ => Err(Error::UnrecognizedArgument(arg)),
        }
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        match self {
            DevCommand::Lex(cmd) => cmd.run(),
            DevCommand::Parse(cmd) => cmd.run(),
        }
    }
}
