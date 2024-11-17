// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

#[derive(Debug, thiserror::Error, Default, PartialEq, Clone)]
pub enum Error {
    #[error("Unknown character sequence.")]
    #[default]
    UnknownCharacterSequence,
}
