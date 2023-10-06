
pub mod instance;

pub type AgdaCommand = dyn IAgdaCommand;
pub trait IAgdaCommand
{
    /// command to give to agda
    fn get_command(&self) -> String;

    /// if parsing is successful, has to return `()`
    fn parse(&mut self, line: &str) -> Option<()>;
}

