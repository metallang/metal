// SPDX-License-Identifier: MIT

use std::{env, fs, process::Command};

fn prefix_dir(s: &str) -> String {
    if cfg!(target_os = "windows") {
        "\\".to_string() + s
    } else {
        "/".to_string() + s
    }
}

pub fn get_lld_dir() -> String {
    let mut output = if let Ok(p) = env::var("LLVM_SYS_191_PREFIX") {
        p + &prefix_dir("bin")
    } else if let Ok(p) = env::var("METAL_LLD_DIR") {
        p
    } else {
        let output = Command::new("llvm-config").arg("--bindir").output();

        if let Ok(output) = output {
            String::from_utf8_lossy_owned(output.stdout).replace("\n", "")
        } else {
            panic!("Could not acquire a path for a linker")
        }
    };

    if cfg!(target_os = "windows") {
        output += "\\llvm-link.exe";
    } else {
        output += "/ld.lld"
    }

    if let Ok(false) = fs::exists(&output) {
        if cfg!(target_os = "windows") {
            return "llvm-link".to_string();
        } else {
            return "ld.lld".to_string();
        }
    }

    output
}

pub fn link(lld_dir: String, objs: Vec<String>, output_dir: &str, custom_link_args: &Vec<String>) {
    let mut command = Command::new(&lld_dir);

    // NOTE: only supports windows. Need someone with a Linux computer to be able to
    // get the correct args for linux

    // if std isn't being used, this shouldn't be linked.
    // TODO: allow configuring the subsystem, i.e. for window-only applications.
    command.arg("/SUBSYSTEM:CONSOLE");
    command.arg(format!("/OUT:{}", output_dir));

    for arg in custom_link_args {
        command.arg(arg);
    }
    command.args(objs);
    command.arg("kernel32.lib");

    #[cfg(debug_assertions)]
    eprintln!(
        "link error result: {:?}",
        String::from_utf8_lossy_owned(command.output().unwrap().stderr)
    );
}
