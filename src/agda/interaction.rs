
pub mod command;

use std::{process::{Command}, time::Duration, thread::sleep, sync::{Mutex, Arc}};
use std::mem::ManuallyDrop;
use interactive_process::InteractiveProcess;

use super::AGDA;
use command::AgdaCommand;


pub struct AgdaInteraction
{
    process: ManuallyDrop<InteractiveProcess>,
    result_mutex: Arc<Mutex<Vec<String>>>
}

impl Drop for AgdaInteraction
{
    fn drop(&mut self)
    {
        unsafe
        {
            ManuallyDrop::<InteractiveProcess>::take(&mut self.process).close().kill().unwrap()
        }
    }
}

impl AgdaInteraction
{
    pub fn new() -> AgdaInteraction
    {
        let load_result = Vec::new();
        let load_result_mutex = Arc::new(Mutex::new(load_result));
        let result_mutex = load_result_mutex.clone();

        let process = ManuallyDrop::new(InteractiveProcess::new(
            Command::new(AGDA).arg("--interaction"),
            move |line|
            {
                let res = line.unwrap();
                println!("Got: {}", res.clone());
                let mut str = load_result_mutex.lock().unwrap();
                str.push(res);
            }
        ).unwrap());

        AgdaInteraction { process, result_mutex }
    }

    pub fn run_command(&mut self, cmd: &mut AgdaCommand) -> Option<()>
    {
        // reset mutex to be empty
        *(self.result_mutex.lock().unwrap()) = Vec::new();

        // call agda with command
        self.process.send(&cmd.get_command()).unwrap();

        // we wait for at most 30s for command to return
        for i in 0..60
        {
            println!("===== ITER {i} ====");
            sleep(Duration::from_millis(500));

            // check whether one of the output lines matches our pattern
            let mut result_lock = self.result_mutex.lock().unwrap();
            for line in result_lock.iter()
            {
                // if parsing is successful, return
                // (the parse call will have changed the cmd object to contain the result)
                match cmd.parse(&line)
                {
                    Some(()) =>
                    {
                        println!("{line} matched!");
                        return Some(())
                    },
                    None => println!("{line} didnt match"),
                }
            }

            // if we didn't find anything, empty-out current result lines
            // and try again in the next iteration
            *result_lock = Vec::new()
        }

        None
    }
}


