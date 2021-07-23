pub mod add_employee {

use std::collections::HashMap;
use std::io;

    pub fn add_employees() -> HashMap<String, String> {
        // Initialize vectors and hashmap
        let mut employees: Vec<(String, String)> = Vec::new();
        let mut directory: HashMap<String, String> = HashMap::new();
    
        println!("Welcome to employee administration!");
        
        // Initialize input variables and loop
        let mut emp_name_entry = String::new();
        let mut dept_name_entry = String::new();
        let mut cmd_loop = true;
    
        // Begin 'employee loop'. User will be asked to enter employee's name and department.
        // Name and department are pushed to a String Vector. Vector values are then mapped to variables
        // which are then inserted into the Hashmap.
    
        while cmd_loop == true {
            println!("Please enter the employee's name.");
            io::stdin()
                .read_line(&mut emp_name_entry)
                .expect("Entry failed, try again.");
    
            println!("Please enter the employee's department.");
            io::stdin()
                .read_line(&mut dept_name_entry)
                .expect("Entry failed, try again.");
            
            employees.push((emp_name_entry.to_string(), dept_name_entry.to_string()));
    
            // The 'go on' loop. This loops allows the user to choose to add another employee or stop.
            // Response values (y/n) are mapped to variables for easier reading. Else captures any other response
            // and continues the loop until a proper response is input.
    
            let mut go_on: String = String::new();
            let mut go_on_loop = true;
            let yes: String = "y".to_string();
            let no: String = "n".to_string(); 
    
            while go_on_loop == true {
                println!("Add another employee? y/n");
                io::stdin()
                    .read_line(&mut go_on)
                    .expect("Entry failed, try again.");
    
                    // Entry variables are re-initialized and Vectors are drained. Another way to do this may be to
                    // add a counter variable that's increased by 1 for each time the loop repeats instead of draining
                    // the vectors each time.
    
                    if go_on.trim().to_string() == yes {
                        go_on_loop = false;
                        cmd_loop = true;
                        emp_name_entry.clear();
                        dept_name_entry.clear();
                        go_on.clear();
                    } else if go_on.trim().to_string() == no {
                        go_on_loop = false;
                        cmd_loop = false;
                        for (key, val) in &employees {
                            directory.insert(key.trim().to_string(), val.trim().to_string());
                        }
                    } else {
                        go_on_loop = true;
                        println!("Please enter y or n.");
                        go_on.clear();
                    }
                }
        }
        directory
    }
}