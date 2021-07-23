
    #[derive(Debug)]

    struct Employee {
        name: String,
        department: String,
    }

    impl Employee {
        pub fn new() -> Employee {
            let new_employee = Employee {
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
        println!("{:?}", new_employee);
    }
}