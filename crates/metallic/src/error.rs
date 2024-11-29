// SPDX-License-Identifier: MIT

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unrecognized argument: {0}")]
    UnrecognizedArgument(tapcli::Arg),

    #[error("Insufficient arguments. Run with `--help` to see usage.")]
    InsufficientArguments,
}
