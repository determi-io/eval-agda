
use std::{process::{Command}, time::Duration, thread::sleep, sync::{Mutex, Arc}};
use interactive_process::InteractiveProcess;


////////////////////////////////////////////////////////////
// State


////////////////////////////
// Process


////////////////////////////
// Command




////////////////////////////////////////////////////////////
// Using state


pub fn load_name_in_file(name: &str, file: &str)
{


    // Send some data, waiting in between.
    // The result of this is "Got: echo: data1" being printed by our callback,
    // since our callback preprends "Got: " and the child process prepends
    // "echo: ".
    proc.send(&agda_load_command(name, file)).unwrap();

    // Sleep in this thread. Note that the process' `stdout` is processed in
    // another thread, so while this thread sleeps, that thread will pick
    // up the message printed by the child process and run the callback.
    sleep(Duration::from_secs(1));

    // Repeat that a few more times, for kicks.
    proc.send("data2").unwrap();
    sleep(Duration::from_secs(1));
    proc.send("data3").unwrap();

    // If we don't sleep here, the process won't have time to reply
    // before we kill it.
    sleep(Duration::from_millis(1));

    // We're done with the process, but it is not self-terminating,
    // so we can't use `proc.wait()`. Instead, we'll take the `Child` from
    // the `InteractiveProcess` and kill it ourselves.
    proc.close().kill().unwrap();

    let res = read_result_mutex.lock().unwrap();

    println!("other output was: {res:?}");
}




