// SPDX-License-Identifier: MIT

#![feature(let_chains)]

use debug_to_display::ForwardDebugToDisplay;
use tapcli::Command;

use crate::cli::Cli;
use crate::error::Error;

pub mod cli;
pub mod config;
pub mod error;

fn main() -> Result<(), ForwardDebugToDisplay<Error>> {
    let cli = Cli::from_env()?;

    cli.run()?;

    Ok(())
}
