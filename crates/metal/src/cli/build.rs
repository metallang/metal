use crate::error::Error;

// If the command needs not to store any data, it is recommended to add a private
// unit field to prevent accidentally constructing it without going through ::parse()
// aka encapsulation
pub struct BuildCommand(());

impl tapcli::Command for BuildCommand {
    type Error = Error;

    fn parse(_: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        // this command has no arguments (yet)

        Ok(Self(()))
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        eprintln!(
            "The time will come, and this command will do something useful. Until then, bye."
        );

        // In the future, this will implement an interface on top of `metallic`, which will have
        // it's own CLI. Thanks to `tapcli`, we will be able to simply re-use command definitions
        // from future `metallic` here.

        // (^ definitely not me praising my own work)

        Ok(())
    }
}
