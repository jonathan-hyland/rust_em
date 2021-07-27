pub mod employee;

pub mod employee_funcs {

    use crate::employee_administration::employee_functions::employee::emp::Employee;
    use std::io;
    use std::fs::File;

    pub fn add_employees() {
        // Initialize vector, entry variables, and loop control
        let mut employees: Vec<Employee> = Vec::new();
        let mut emp_name_entry = String::new();
        let mut dept_name_entry = String::new();
        let mut cmd_loop = true;
    
        /* Begin 'employee loop'. User will be asked to enter employee's name and department.
        A new instance of Employee is created and the 'name' and 'department' values are mapped to 'emp_name_entry'
        and 'dept_name_entry', respectively. Employee is then pushed to a Vector that will hold all employees
        added during the current session. */
    
        while cmd_loop == true {
            println!("Please enter the employee's name.");
            io::stdin()
                .read_line(&mut emp_name_entry)
                .expect("Entry failed, try again.");
    
            println!("Please enter the employee's department.");
            io::stdin()
                .read_line(&mut dept_name_entry)
                .expect("Entry failed, try again.");
            
            let mut new_employee = Employee::new();
            new_employee.name = emp_name_entry.clone();
            new_employee.department = dept_name_entry.clone();
            employees.push(new_employee);
    
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
    
                    if go_on.trim().to_string() == yes {
                        go_on_loop = false;
                        cmd_loop = true;
                        emp_name_entry.clear();
                        dept_name_entry.clear();
                        go_on.clear();
                    } else if go_on.trim().to_string() == no {
                        go_on_loop = false;
                        cmd_loop = false;
                        let mut directory = File::create("employee.txt").expect("unable to make file");
                        for e in &employees {
                            bincode::serialize_into(&mut directory, e).unwrap();
                        }
                        println!("{:#?}", employees);
                    } else {
                        go_on_loop = true;
                        println!("Please enter y or n.");
                        go_on.clear();
                    }
                }
        }
    }

    pub fn _edit_employees() {
        // placeholder for editing employees
    }

    pub fn _delete_employees() {
        // placeholder for deleting employees
    }
}