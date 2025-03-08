// SPDX-License-Identifier: MIT

use std::{env, process::Command};

#[derive(Debug)]
pub enum LLCFormat {
    /// Compile to textual assembly (`.s`)
    TextualAssembly,
    /// Compile to a native object (`.o`)
    NativeObject,
}

#[derive(Debug)]
#[repr(u8)]
pub enum Optimization {
    O0 = 0,
    O1 = 1,
    O2 = 2,
    O3 = 3,
}

pub fn get_llc_dir() -> String {
    let mut output = if let Ok(p) = env::var("LLVM_SYS_191_PREFIX") {
        p + "\\bin"
    } else if let Ok(p) = env::var("METAL_LLD_DIR") {
        p
    } else {
        let output = Command::new("llvm-config")
            .arg("--bindir")
            .output()
            .unwrap();

        String::from_utf8_lossy_owned(output.stdout)
    };

    if cfg!(target_os = "windows") {
        output += "\\llc.exe";
    } else {
        output += "/llc";
    }

    output
}

/// Compiles LLVM IR to using LLC to a `.o` or `.asm`
pub fn ir_llc(
    llc_path: &str,
    input_file: &str,
    output_file_name: &str,
    optimize: Optimization,
    llcfmt: LLCFormat,
) {
    let filetype = match llcfmt {
        LLCFormat::TextualAssembly => "asm",
        LLCFormat::NativeObject => "obj",
    };
    let mut command = Command::new(llc_path);

    command
        .arg(format!("-o={}", output_file_name))
        .arg(format!("-O={}", optimize as u8))
        .arg(format!("-filetype={}", filetype))
        .arg(input_file);

    #[cfg(debug_assertions)]
    eprintln!("Running LLC command: `{:?}`", &command);

    #[cfg(debug_assertions)]
    eprintln!(
        "LLC error result: {:?}",
        String::from_utf8_lossy_owned(command.output().unwrap().stderr)
    );
}
