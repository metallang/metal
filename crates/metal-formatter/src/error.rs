#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Expected the Option to be Some")]
    UnexpectedNone,
}

#[allow(clippy::disallowed_types)]
pub type Result<T = ()> = std::result::Result<T, Error>;
