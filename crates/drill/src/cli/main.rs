// SPDX-License-Identifier: MIT

use tapcli::*;

use crate::error::Error;

use super::help::HelpCommand;

enum MainCommand {
    Help(HelpCommand),
}

impl Command for MainCommand {
    type Error = Error;

    fn parse(parser: &mut Parser) -> Result<Self, Self::Error> {
        match parser.next() {
            Some(arg) => {
                #[allow(clippy::wildcard_enum_match_arm)]
                match arg.as_ref() {
                    ArgRef::Long("help") => Ok(Self::Help(HelpCommand::parse(parser)?)),
                    ArgRef::Value("help") => Ok(Self::Help(HelpCommand::parse(parser)?)),
                    _ => Err(Error::UnrecognizedArgument(arg)),
                }
            },
            None => Err(Error::NoArgs),
        }
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        match self {
            MainCommand::Help(cmd) => cmd.run(),
        }
    }
}

pub fn app() -> Result<(), Box<dyn std::error::Error>> {
    let cli = MainCommand::from_env()?;
    cli.run()?;
    Ok(())
}