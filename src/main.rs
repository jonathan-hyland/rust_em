/* Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a
department or all people in the company by department, sorted alphabetically. */

mod add_employee;
mod print_directory;
use crate::print_directory::print_directory::company_directory;
use crate::add_employee::add_employee::add_employees;

fn main() {
    let emp_directory = add_employees();
    company_directory(emp_directory);
}