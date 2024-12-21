// SPDX-License-Identifier: MIT

fn main() -> Result<(), metal_ungram::Error> {
    metal_ungram::generate_lexer()?;

    Ok(())
}
