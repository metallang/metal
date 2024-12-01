// SPDX-License-Identifier: MIT

use std::{fs, io};

#[derive(Debug, thiserror::Error)]
pub enum TargetError {
    #[error("Failed to create target directory. IO Error: {0}")]
    SetupFailure(io::Error),
    #[error("Failed to write target ignores. IO Error: {0}")]
    TargetIgnoreFailure(io::Error),
    #[error("Failed to reset LLVM cache. IO Error: {0}")]
    LLVMResetFailure(io::Error),
    #[error("Failed to write to LLVM cache. IO Error: {0}")]
    LLVMWriteFailure(io::Error),
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
        fs::create_dir_all("./target/.llvm-cache").map_err(TargetError::SetupFailure)?;

        if !fs::exists("./target/CACHEDIR.TAG").map_err(TargetError::TargetIgnoreFailure)? {
            fs::write("./target/CACHEDIR.TAG", CACHEDIR_CONTENTS)
                .map_err(TargetError::TargetIgnoreFailure)?;
        }
        if !fs::exists("./target/.gitignore").map_err(TargetError::TargetIgnoreFailure)? {
            fs::write("./target/.gitignore", GITIGNORE_CONTENTS)
                .map_err(TargetError::TargetIgnoreFailure)?;
        }
        Ok(())
    }

    pub fn reset_llvm() -> TargetResult<()> {
        fs::remove_dir_all("./target/.llvm-cache").map_err(TargetError::LLVMResetFailure)?;
        fs::create_dir_all("./target/.llvm-cache").map_err(TargetError::LLVMResetFailure)?;
        Ok(())
    }

    pub fn write_llvm_ir(module: &str, contents: &[u8]) -> TargetResult<()> {
        fs::write(
            "./target/.llvm-cache/".to_string() + module + ".ll",
            contents,
        )
        .map_err(TargetError::LLVMWriteFailure)?;
        Ok(())
    }
}
