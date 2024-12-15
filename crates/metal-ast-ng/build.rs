// SPDX-License-Identifier: MIT

fn main() -> Result<(), metal_ast_ungram::Error> {
    println!("cargo::rerun-if-changed=./metal.ungram");

    metal_ast_ungram::run()?;

    Ok(())
}
