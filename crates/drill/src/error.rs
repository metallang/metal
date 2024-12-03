#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No arguments were given! Run with `--help` to see usage.")]
    NoArgs,
    #[error("The argument '{0}' is unrecognized! Check `--help` for the list of args.")]
    UnrecognizedArgument(tapcli::Arg),
}