// SPDX-License-Identifier: MIT

use std::{env, fs, process::Command};

pub fn get_lld_dir() -> String {
    let mut output = if let Ok(p) = env::var("LLVM_SYS_191_PREFIX") {
        p + "\\bin"
    } else {
        let output = Command::new("llvm-config")
            .arg("--bindir")
            .output()
            .unwrap();

        String::from_utf8_lossy_owned(output.stdout).replace("\n", "")
    };

    if cfg!(target_os = "windows") {
        output += "\\lld-link.exe";
    }

    if let Ok(false) = fs::exists(&output) {
        return "lld-link".to_string();
    }

    output
}

pub fn link(lld_dir: String, objs: Vec<String>, output_dir: &str) {
    let mut command = Command::new(lld_dir);

    command.args(objs);
    command.arg("libcmt.lib");
    command.arg("/SUBSYSTEM:WINDOWS");
    command.arg(format!("/OUT:{}", output_dir));

    #[cfg(debug_assertions)]
    eprintln!(
        "link error result: {:?}",
        String::from_utf8_lossy_owned(command.output().unwrap().stderr)
    );
}
