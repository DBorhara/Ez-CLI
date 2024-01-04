mod git;
use crate::git::git_command;
mod orchestrator;
use crate::orchestrator::orchestrator::orchestrator;
use std::error::Error;
use std::fs;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::open("src/intro.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", &contents);

    let mut input_number = String::new();
    println!("Select a valid number: ");
    io::stdin()
        .read_line(&mut input_number)
        .expect("Failed to read line.");
    println!("Selected Number in Main:{}", input_number);
    let input_number: usize = input_number.trim().parse().unwrap();
    orchestrator(input_number)?;
    Ok(())
}
