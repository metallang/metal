// SPDX-License-Identifier: MIT

use lex_ng::DevLexNgCommand;
use parse::DevParseCommand;
use parse_ng::DevParseNgCommand;

use crate::error::Error;

mod lex_ng;
mod parse;
mod parse_ng;

pub enum DevCommand {
    /// Parse a Metal source file and debug-print it's AST.
    Parse(DevParseCommand),
    /// Debug-print the result of lexing the provided file.
    LexNg(DevLexNgCommand),
    /// Parse a Metal source file using the new parser and debug-print it's AST.
    ParseNg(DevParseNgCommand),
}

impl tapcli::Command for DevCommand {
    type Error = Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let arg = parser.next().ok_or(Error::InsufficientArguments)?;

        match arg.as_ref() {
            tapcli::ArgRef::Value("parse") => Ok(Self::Parse(DevParseCommand::parse(parser)?)),
            tapcli::ArgRef::Value("lex-ng") => Ok(Self::LexNg(DevLexNgCommand::parse(parser)?)),
            tapcli::ArgRef::Value("parse-ng") => {
                Ok(Self::ParseNg(DevParseNgCommand::parse(parser)?))
            }
            _ => Err(Error::UnrecognizedArgument(arg)),
        }
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        match self {
            DevCommand::Parse(cmd) => cmd.run(),
            DevCommand::LexNg(cmd) => cmd.run(),
            DevCommand::ParseNg(cmd) => cmd.run(),
        }
    }
}
