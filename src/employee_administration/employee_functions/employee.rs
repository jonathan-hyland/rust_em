pub mod emp {
    
    use std::fmt;
    use std::fmt::*;
    use rand;
    
    #[derive(Debug)]
    pub struct Employee {
        id: i128,
        pub name: String,
        pub department: String,
    }
    
    impl Display for Employee {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {} {}", self.id, self.name, self.department)
        }
    }
    
    impl Employee {
        pub fn new() -> Employee {
            let new_employee = Employee {
                id : rand::random(),
                name : String::new(),
                department : String::new(),
            };
            new_employee
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        pub fn new_employee_test() {
            let mut new_employee = Employee::new();
            new_employee.name = "John".to_string();
            new_employee.department = "Sales".to_string();
            println!("{}", new_employee);
        }
    }
}