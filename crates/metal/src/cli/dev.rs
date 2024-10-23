use crate::error::Error;

pub enum DevCommand {
    /// A placeholder command that should be removed once at least one
    /// other subcommand is added (i.e. this command's only purpose is
    /// to make instantiating DevCommand possible).
    Placeholder,
}

impl tapcli::Command for DevCommand {
    type Error = Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let arg = parser.next().ok_or(Error::InsufficientArguments)?;

        match arg.as_ref() {
            tapcli::ArgRef::Value("placeholder") => Ok(Self::Placeholder),
            _ => Err(Error::UnrecognizedArgument(arg)),
        }
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        match self {
            DevCommand::Placeholder => {
                eprintln!("I am a gorgeous placeholder. Normally, one would instead do ::Placeholder(cmd) => cmd.run()
                but gorgeous placeholders are an exception, because i said so and because i am lazy. Also it's getting late.");

                Ok(())
            }
        }
    }
}
