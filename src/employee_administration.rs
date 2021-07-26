pub mod employee_functions;
pub use crate::employee_administration::employee_functions::employee_funcs;

pub mod employee_admin {

    use crate::employee_administration::employee_funcs::add_employees;
    use std::io;

    pub fn start_administration() {
        println!("Welcome to employee administration!");
        println!(
            "Please select a command:\n
            1 - Add a new employee\n
            2 - Edit an existing employee\n
            3 - Delete an existing employee"
        );
        
        let mut cmd = String::new();
        let mut start_loop = true;

        while start_loop == true {
            io::stdin()
                .read_line(&mut cmd)
                .expect("Please enter a valid command.");
            
            let cmd_trim = cmd.trim();

            match cmd_trim.parse::<i32>() {
                Ok(1) => {
                    println!("Adding new employees...");
                    start_loop = false;
                    add_employees();
                }
                Ok(2) => {
                    println!("Accessing employee information...");
                    start_loop = false;
                }
                Ok(3) => {
                    println!("Delete an employee. WARNING: This action is not reversable!");
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
}