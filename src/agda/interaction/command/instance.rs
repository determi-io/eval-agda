use regex::Regex;

use super::IAgdaCommand;


pub struct AgdaCommandLoad
{
    command: String,
    regex: Regex,
    pub result: Option<String>
}

impl AgdaCommandLoad
{
    pub fn new(name: &str, file: &str) -> AgdaCommandLoad
    {
        let command = format!("IOTCM \"{file}\" NonInteractive Indirect ( Cmd_compute_toplevel DefaultCompute \"{name}\" )\n");
        let regex = Regex::new(r#"^\(agda2-info-action "\*Normal Form\*" "(?s)(.*)" nil\)$"#).unwrap();

        AgdaCommandLoad { command, regex, result: None }
    }
}

impl IAgdaCommand for AgdaCommandLoad
{
    fn get_command(&self) -> String {
        self.command.clone()
    }

    fn parse(&mut self, line: &str) -> Option<()> {
        if let Some(capture) = self.regex.captures(line)
        {
            let ex = capture.extract();
            println!("extracted is: {:?}", ex);
            let (_, [value]) = ex;
            self.result = Some(value.to_owned());
            Some(())
        }
        else
        {
            None
        }
    }
}






