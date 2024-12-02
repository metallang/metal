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
        fs::create_dir_all("./target/.llvm-cache")?;

        if !fs::exists("./target/CACHEDIR.TAG")? {
            fs::write("./target/CACHEDIR.TAG", CACHEDIR_CONTENTS)?;
        }
        if !fs::exists("./target/.gitignore")? {
            fs::write("./target/.gitignore", GITIGNORE_CONTENTS)?;
        }
        Ok(())
    }

    pub fn reset_llvm() -> TargetResult<()> {
        fs::remove_dir_all("./target/.llvm-cache")?;
        fs::create_dir_all("./target/.llvm-cache")?;
        Ok(())
    }

    pub fn write_llvm_ir(module: &str, contents: &[u8]) -> TargetResult<()> {
        fs::write(
            "./target/.llvm-cache/".to_string() + module + ".ll",
            contents,
        )?;
        Ok(())
    }
}
