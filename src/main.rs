
// use crate::agda_interaction::load_name_in_file;

use crate::agda::interaction::{AgdaInteraction, command::{instance::AgdaCommandLoad, AgdaCommand}};

pub mod agda;


fn main() {
    let x = agda::AGDA;
    println!("agda: {x}");

    // load_name_in_file("main", "./data/Test.agda");
    let mut interaction = AgdaInteraction::new();
    let mut cmd = AgdaCommandLoad::new("main", "./data/Test.agda");
    match interaction.run_command(&mut cmd)
    {
        Ok(()) =>
        {
            println!("success!");
            println!("{}", cmd.result.unwrap())
        },
        Err(e) =>
        {
            println!("error! {e}");
        }
    }
}





