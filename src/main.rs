mod employee_administration;

pub use crate::employee_administration::employee_admin;

// mod print_directory;

use std::io;

fn main() {
    start_app();
}

fn start_app() {
    println!("Welcome to Rust 'Em, an application built in Rust to manage employee rosters.");
    println!(
        "Please select from the following options:\n
        1 - Employee administration\n
        2 - Access employee directory\n
        3 - Exit"
    );

    let mut cmd = String::new();
    let mut start_loop = true;

    while start_loop == true {

        // Capture user input as a String, then trim the string.
        io::stdin()
            .read_line(&mut cmd)
            .expect("Please enter a valid command.");
        
        let cmd_trim = cmd.trim();

        // Match statement parses the str into an i32 and compares passed value to the arms.
        match cmd_trim.parse::<i32>() {
            Ok(1) => {
                println!("Accessing employee administration...");
                start_loop = false;
                employee_admin::start_administration();
            }
            Ok(2) => {
                println!("Accessing employee directory...");
                start_loop = false;
            }
            Ok(3) => {
                println!("Good bye!");
                start_loop = false;
            },
            _ => {
                println!("Please enter a valid command.");
                cmd.clear();
                start_loop = true;
            }
        }
    }
}