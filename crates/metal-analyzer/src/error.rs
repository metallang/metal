// SPDX-License-Identifier: MIT

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error with file IO system: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Bincode error: {0}")]
    BincodeError(#[from] bincode::Error),
}
