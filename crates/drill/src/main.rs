// SPDX-License-Identifier: MIT

use colored::Colorize;

mod cli;
mod error;

fn main() {
    let result = cli::main::app();

    if let Err(error) = result {
        eprintln!("{}: {}", "Error".red(), error)
    }
}