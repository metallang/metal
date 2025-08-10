// SPDX-License-Identifier: MIT

use crate::cli::dev::format_dom::DevFormatDomCommand;
use crate::cli::dev::lex::DevLexCommand;
use crate::cli::dev::parse::DevParseCommand;
use crate::error::Error;

mod format_dom;
mod lex;
mod parse;

pub enum DevCommand {
    /// Debug-print the result of lexing the provided file.
    Lex(DevLexCommand),
    /// Parse a Metal source file and debug-print its AST.
    Parse(DevParseCommand),
    /// Parse a Metal source file and debug-print its formatter DOM.
    FormatDom(DevFormatDomCommand),
}

impl tapcli::Command for DevCommand {
    type Error = Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let arg = parser.next().ok_or(Error::InsufficientArguments)?;

        match arg.as_ref() {
            tapcli::ArgRef::Value("lex") => Ok(Self::Lex(DevLexCommand::parse(parser)?)),
            tapcli::ArgRef::Value("parse") => Ok(Self::Parse(DevParseCommand::parse(parser)?)),
            tapcli::ArgRef::Value("format-dom") => {
                Ok(Self::FormatDom(DevFormatDomCommand::parse(parser)?))
            }
            _ => Err(Error::UnrecognizedArgument(arg)),
        }
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        match self {
            DevCommand::Lex(cmd) => cmd.run(),
            DevCommand::Parse(cmd) => cmd.run(),
            DevCommand::FormatDom(cmd) => cmd.run(),
        }
    }
}
