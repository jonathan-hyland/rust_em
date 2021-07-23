pub mod print_directory {
    use std::collections::HashMap;
    use std::io;

    pub fn company_directory(dir: HashMap<String, String>) {

        println!("Employee entry finished. To print by department, type the department name.
        To print the entire company directory, type 'dir'.");

        let mut dept_name = String::new();

        io::stdin()
            .read_line(&mut dept_name)
            .expect("Entry failed, try again.");
        
        // Print all employees entered into the directory.
        if dept_name.trim() == "dir" {
            let mut dir_vec: Vec<(&String, &String)> = dir.iter().collect();
            dir_vec.sort_by(|a, b| a.cmp(&b));
            for (emp, dept) in dir_vec {
                println!("{}", dept); // if dept appears multiple times, print only once but print all employees
                println!("{}", emp);
            }
        } else {
            // Print all employees in user-entered department, alphabetize the names, and print.
            println!(" ");
            print!("{}", dept_name); // assumes user is typing in a valid dept name
            let mut dir_vec: Vec<(&String, &String)> = dir.iter().collect();
            dir_vec.sort_by(|a, b| a.cmp(&b));
            for (emp, dept) in dir_vec {
                if dept.trim() == dept_name.trim() {
                    println!("{}", emp);
                } else {

                }
            }
        }
    }
}