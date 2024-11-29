// SPDX-License-Identifier: MIT

use build::BuildCommand;

use crate::error::Error;

pub mod build;

pub enum Cli {
    /// `metallic build` a command for building Metal code or IR
    Build(BuildCommand),
}

impl tapcli::Command for Cli {
    type Error = Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let arg = parser.next().ok_or(Error::InsufficientArguments)?;

        match arg.as_ref() {
            tapcli::ArgRef::Value("b" | "build") => Ok(Self::Build(BuildCommand::parse(parser)?)),
            _ => Err(Error::UnrecognizedArgument(arg)),
        }
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        match self {
            Cli::Build(cmd) => cmd.run(),
        }
    }
}
