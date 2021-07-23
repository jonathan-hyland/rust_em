A more sophisticated version of the Chapter 8 exercise from The Rust Programming Language (https://doc.rust-lang.org/book/):

*Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.*

Additions will include:

- reading and writing employee data to a file
- a static list of departments with checks to ensure employees can only be added to existing departments
- functionality to add, edit, and delete departments and employees
- functionality to print search results to a file