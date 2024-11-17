// SPDX-License-Identifier: MIT

#[derive(Debug, thiserror::Error, Default, PartialEq, Clone)]
pub enum Error {
    #[error("Unknown character sequence.")]
    #[default]
    UnknownCharacterSequence,
}
