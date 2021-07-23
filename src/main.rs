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
        2 - Access employee directory"
    );

    let mut cmd = String::new();
    let mut start_loop = true;

    while start_loop == true {
        io::stdin()
            .read_line(&mut cmd)
            .expect("Please enter a valid command.");
    
        if cmd.trim() == "1".to_string() {
            println!("Accessing employee administration...");
            start_loop = false;
            employee_admin::start_administration();
        } else if cmd.trim() == "2".to_string() {
            println!("Accessing employee directory...");
            start_loop = false;
        } else {
            println!("Please enter a valid command.");
            cmd.clear();
            start_loop = true;
        }
    }
}