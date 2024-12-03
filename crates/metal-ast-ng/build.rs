use std::process::Command;

fn main() -> Result<(), std::io::Error> {
    println!("cargo::rerun-if-changed=./metal.ungram");

    // Command::new("cargo")
    //     .args(["run", "-p", "scripts", "--bin", "ungram"])
    //     .spawn()?
    //     .wait()?;

    Ok(())
}
