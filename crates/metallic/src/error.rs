// SPDX-License-Identifier: MIT

use std::io;

use ron::de::SpannedError;

use crate::target::TargetError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unrecognized argument: {0}")]
    UnrecognizedArgument(tapcli::Arg),

    #[error("Insufficient arguments. Run with `--help` to see usage.")]
    InsufficientArguments,

    #[error("\"{0}\" is not a supported input type")]
    UnsupportedInputType(String),

    #[error("System IO Error: {0}")]
    IOError(io::Error),

    #[error("Failed to deserialize MIR: {0}")]
    DeserializationError(SpannedError),

    #[error("Feature not yet implemented: {0}")]
    Unimplemented(String),

    #[error("{0}")]
    TargetError(TargetError),
}
