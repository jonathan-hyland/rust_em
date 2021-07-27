// Contains the definition for the Employee struct and related functions.
pub mod emp {
    
    use core::fmt;
    use core::fmt::Display;
    use serde::{Serialize, Deserialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Employee {
        id: Uuid,
        pub name: String,
        pub department: String,
    }
    
    impl Display for Employee {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {}", self.name, self.department)
        }
    }
    
    impl Employee {
        pub fn new() -> Employee {
            let new_employee = Employee {
                id : Uuid::new_v4(),
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