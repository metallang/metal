// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unrecognized argument: {0}")]
    UnrecognizedArgument(tapcli::Arg),

    #[error("Insufficient arguments. Run with `--help` to see usage.")]
    InsufficientArguments,
}
