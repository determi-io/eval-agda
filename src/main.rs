





use crate::agda_interaction::load_name_in_file;

mod agda_interaction;

fn main() {
    let x = agda_interaction::AGDA;
    println!("agda: {x}");

    load_name_in_file("main", "./data/Test.agda");
}





