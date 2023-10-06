
use std::{process::{Command, Stdio}, io::{Write, Read}, mem::take, time::Duration, thread::sleep, sync::{Mutex, Arc}};
use std::io::{prelude::*, Result};

use interactive_process::InteractiveProcess;

use crate::agda_interaction::load_name_in_file;

mod agda_interaction;

fn main() {
    let x = agda_interaction::AGDA;
    println!("agda: {x}");


    // let out = Command::new(x).output().expect("could not run");

    // let res = std::str::from_utf8(&out.stdout).unwrap();

    // println!("result: \n{}", res);

    load_name_in_file("main", "./data/Test.agda");
}





