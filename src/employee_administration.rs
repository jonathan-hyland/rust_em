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
        
            if cmd.trim() == "1".to_string() {
                println!("Adding new employee...");
                start_loop = false;
                add_employees();
            } else if cmd.trim() == "2".to_string() {
                println!("Accessing editing functions...");
                start_loop = false;
            } else if cmd.trim() == "3".to_string() {
                println!("Accessing deletion functions...");
                start_loop = false;
            } else {
                println!("Please enter a valid command.");
                cmd.clear();
                start_loop = true;
            }
        }
    }
}