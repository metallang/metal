// SPDX-License-Identifier: MIT


#[derive(Debug, thiserror::Error)]
pub enum AnalyzerError {
    #[error("Error with file IO system: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Bincode error: {0}")]
    RkyvError(#[from] bincode::Error),
}
