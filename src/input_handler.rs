// src/input_handler.rs

use std::io::{self, Write};

pub fn input_string(prompt: &str) -> String {
    let mut input = String::new();

    loop {
        input.clear();

        print!("{}: ", prompt);
        io::stdout().flush().expect("Failed to flush!");

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("\nProgram interrupted by user! Goodbye");
                std::process::exit(0);
            }

            Ok(_) => {
                let trimmed = input.trim().to_string();

                if trimmed.is_empty() {
                    println!("Input can't be empty! Please try again\n");
                    continue;
                }

                return trimmed;
            }

            Err(err) => {
                println!("Something went wrong! {}", err);
                std::process::exit(0);
            }
        }
    }
}

pub fn input_number(prompt: &str) -> i32 {
    let mut input = String::new();

    loop {
        input.clear();

        print!("{}: ", prompt);
        io::stdout().flush().expect("Failed to flush");

        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("\nProgram interrupted by user! Goodbye");
                std::process::exit(0);
            }

            Ok(_) => match input.trim().parse::<i32>() {
                Ok(num) => {
                    return num;
                }

                Err(_) => {
                    println!("Invalid input! Please try again\n");
                    continue;
                }
            },

            Err(err) => {
                println!("Something went wrong! {}", err);
                std::process::exit(0);
            }
        }
    }
}
