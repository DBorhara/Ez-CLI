use std::fs;
use std::io::{self, Read};

use crate::orchestrator::git_orchestrator::git_orchestrator;

pub fn orchestrator(selected_input: usize) -> io::Result<()> {
    println!("Selected Input: {}", selected_input);
    match selected_input {
        1 => {
            let mut file = fs::File::open("src/git_command_select.txt")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            println!("{}", &contents);

            let mut input_number = String::new();

            io::stdin()
                .read_line(&mut input_number)
                .expect("Failed to read line");
            println!("Selected Number:{}", input_number);
            let input_number: usize = input_number.trim().parse().unwrap();
            git_orchestrator(input_number);
            return Ok(());
        }
        _ => {
            // Temp error handling
            println!("Error: Number doesn't exit");
        }
    }
    Ok(())
}
