// SPDX-License-Identifier: MIT

use std::{fs, io};

#[derive(Debug, thiserror::Error)]
pub enum TargetError {
    #[error("Failed to write or read to target. IO Error: {0}")]
    TargetIgnoreFailure(#[from] io::Error),
}

type TargetResult<T> = Result<T, TargetError>;

pub struct Target;

const CACHEDIR_CONTENTS: &str = r#"Signature: 8a477f597d28d172789f06886806bc55
# This file is a cache directory tag created by metallic.
# For information about cache directory tags, see:
#	http://www.brynosaurus.com/cachedir/"#;

const GITIGNORE_CONTENTS: &str = "*";

impl Target {
    pub fn setup() -> TargetResult<()> {
        // ensure the directories all exist
        fs::create_dir_all("./target/.llvm-ir-cache")?;
        fs::create_dir_all("./target/.llvm-linking-cache")?;
        fs::create_dir_all("./target/build")?;

        if !fs::exists("./target/CACHEDIR.TAG")? {
            fs::write("./target/CACHEDIR.TAG", CACHEDIR_CONTENTS)?;
        }
        if !fs::exists("./target/.gitignore")? {
            fs::write("./target/.gitignore", GITIGNORE_CONTENTS)?;
        }
        Ok(())
    }

    pub fn reset_llvm() -> TargetResult<()> {
        fs::remove_dir_all("./target/.llvm-ir-cache")?;
        fs::create_dir_all("./target/.llvm-ir-cache")?;
        fs::remove_dir_all("./target/.llvm-linking-cache")?;
        fs::create_dir_all("./target/.llvm-linking-cache")?;
        Ok(())
    }

    pub fn reset_build() -> TargetResult<()> {
        fs::remove_dir_all("./target/build")?;
        fs::create_dir_all("./target/build")?;
        Ok(())
    }

    pub fn write_llvm_ir(module: &str, contents: &[u8]) -> TargetResult<String> {
        let path = "./target/.llvm-ir-cache/".to_string() + module + ".ll";
        fs::write(&path, contents)?;
        Ok(path)
    }

    pub fn write_build_output(module: &str, contents: &[u8], filetype: &str) -> TargetResult<()> {
        // TODO: separate between triples & environments
        fs::write("./target/build/".to_string() + module + filetype, contents)?;
        Ok(())
    }
}
