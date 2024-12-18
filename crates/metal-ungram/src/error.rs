// SPDX-License-Identifier: MIT

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Syn error: {0}")]
    Syn(#[from] syn::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Ungrammar error: {0}")]
    Ungram(#[from] ungrammar::Error),
    #[error("`metal-ast-ungram` is expected to be run from build scripts only.")]
    InvalidInvocation,
}
