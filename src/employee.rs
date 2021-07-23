pub mod employee {
    struct Employee {
        name: String,
        department: String,
    }

    impl Employee {
        fn new() -> Employee {
            let new_employee = Employee {
                name = String::new(),
                department = String::new(),
            };
            new_employee
        }
    }
}