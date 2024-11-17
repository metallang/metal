// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

use crate::cli::build::BuildCommand;
#[cfg(any(debug_assertions, feature = "dev"))]
use crate::cli::dev::DevCommand;
use crate::error::Error;

pub mod build;
#[cfg(any(debug_assertions, feature = "dev"))]
pub mod dev;

pub enum Cli {
    /// `metal build`
    Build(BuildCommand),
    /// `metal dev`, a namespace for arbitrary command useful during development.
    #[cfg(any(debug_assertions, feature = "dev"))]
    Dev(DevCommand),
}

impl tapcli::Command for Cli {
    type Error = Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let arg = parser.next().ok_or(Error::InsufficientArguments)?;

        match arg.as_ref() {
            tapcli::ArgRef::Value("b" | "build") => Ok(Self::Build(BuildCommand::parse(parser)?)),
            #[cfg(any(debug_assertions, feature = "dev"))]
            tapcli::ArgRef::Value("dev") => Ok(Self::Dev(DevCommand::parse(parser)?)),
            _ => Err(Error::UnrecognizedArgument(arg)),
        }
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        match self {
            Cli::Build(cmd) => cmd.run(),
            #[cfg(any(debug_assertions, feature = "dev"))]
            Cli::Dev(cmd) => cmd.run(),
        }
    }
}
