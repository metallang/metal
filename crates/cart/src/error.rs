// SPDX-License-Identifier: MIT

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unrecognized argument: {0}")]
    UnrecognizedArgument(tapcli::Arg),

    #[error("Insufficient arguments. Run with `--help` to see usage.")]
    InsufficientArguments,

    #[error("Invalid package version: {0}")]
    InvalidPackageVersion(String),

    #[error("Tag can't be included without the `git` field.")]
    TagIncluded,
}
