// SPDX-License-Identifier: MIT

use parse::DevParseCommand;

use crate::error::Error;

mod parse;

pub enum DevCommand {
    /// Parse a Metal source file and debug-print it's AST.
    Parse(DevParseCommand),
}

impl tapcli::Command for DevCommand {
    type Error = Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let arg = parser.next().ok_or(Error::InsufficientArguments)?;

        match arg.as_ref() {
            tapcli::ArgRef::Value("parse") => Ok(Self::Parse(DevParseCommand::parse(parser)?)),
            _ => Err(Error::UnrecognizedArgument(arg)),
        }
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        match self {
            DevCommand::Parse(cmd) => cmd.run(),
        }
    }
}
