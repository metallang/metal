use tapcli::*;

use crate::error::Error;
// use colored::*;

// const VERSION: &str = env!("CARGO_PKG_VERSION");
// const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub struct HelpCommand {}

impl Command for HelpCommand {
    type Error = Error;

    fn parse(_: &mut Parser) -> Result<Self, Self::Error> {
        Ok(Self {})
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        println!("TODO!");

        Ok(())
    }
}