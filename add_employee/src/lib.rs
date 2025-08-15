//! A simple company directory system that allows adding employees to departments
//! and retrieving department lists using simple text commands.

use std::collections::HashMap;

/// Represents the result of parsing and executing a command.
#[derive(PartialEq, Debug)]
pub enum CommandResult {
    /// Indicates a successful addition of an employee.
    Add,
    /// Returns the list of employees in the specified department, if any.
    Show(Option<Vec<String>>),
    /// Returned when a command could not be parsed.
    ParseError(String),
}

/// Stores a mapping of department names to their employee lists.
#[derive(Debug)]
pub struct CompanyDirectory {
    departments: HashMap<String, Vec<String>>,
}

impl CompanyDirectory {
    /// Creates a new, empty `CompanyDirectory`.
    pub fn new() -> Self {
        Self {
            departments: HashMap::new(),
        }
    }

    // Adds an employee to a department, creating the department if needed.
    // Employee names are stored in sorted order.
    pub(crate) fn add_employee(&mut self, employee: &str, department: &str) {
        let names = self.departments.entry(department.to_string()).or_default();
        names.push(employee.to_string());
        names.sort();
        println!("Adding {employee} to {department}...");
    }

    // Retrieves a sorted list of employees in the specified department.
    // Returns `None` if the department doesn't exist.
    pub(crate) fn get_department(&mut self, department: &str) -> Option<Vec<String>> {
        self.departments
            .get_mut(department)
            .map(|names| names.clone())
    }

    /// Parses a user input string and performs the corresponding action.
    ///
    /// Supported commands:
    /// - `"Add <Name> to <Department>"`: Adds an employee.
    /// - `"Show <Department>"`: Retrieves a list of employees in a department.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use add_employee::{CompanyDirectory, CommandResult};
    /// let mut directory = CompanyDirectory::new();
    /// let result = directory.parse_command("Add Alice to Sales");
    /// assert_eq!(result, CommandResult::Add);
    ///
    /// let result = directory.parse_command("Show Sales");
    /// assert!(matches!(result, CommandResult::Show(Some(_))));
    /// ```
    pub fn parse_command(&mut self, input: &str) -> CommandResult {
        let command: Vec<&str> = input.split_whitespace().collect();
        match command.as_slice() {
            ["Add", name, "to", department] => {
                self.add_employee(name, department);
                CommandResult::Add
            }
            ["Show", department] => {
                let employees = self.get_department(department);
                CommandResult::Show(employees)
            }
            _ => CommandResult::ParseError("Could not interpret command.".to_string()),
        }
    }
}

impl Default for CompanyDirectory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_employees() {
        let mut directory = CompanyDirectory::new();
        directory.add_employee("Eve", "HR");
        directory.add_employee("Adam", "HR");
        directory.add_employee("Gerry", "HR");

        let employees = directory.get_department("HR").unwrap();
        assert_eq!(employees, vec!["Adam", "Eve", "Gerry"])
    }

    #[test]
    fn get_empty_deparment() {
        let mut directory = CompanyDirectory::new();
        let employees = directory.get_department("HR");
        assert_eq!(employees, None)
    }

    #[test]
    fn parse_error() {
        let mut directory = CompanyDirectory::new();
        let user_input = "List HR";
        let output = directory.parse_command(user_input);
        assert!(matches!(output, CommandResult::ParseError(_)));
    }

    #[test]
    fn parse_add() {
        let mut directory = CompanyDirectory::new();
        let user_input = "Add Eve to HR";
        let result = directory.parse_command(user_input);
        assert!(matches!(result, CommandResult::Add));
    }

    #[test]
    fn parse_show() {
        let mut directory = CompanyDirectory::new();
        directory.add_employee("Eve", "HR");
        let user_input = "Show HR";
        let result = directory.parse_command(user_input);
        if let CommandResult::Show(Some(names)) = result {
            assert_eq!(names, vec!["Eve"]);
        } else {
            panic!("Expected CommandResult::Show(Some(...)), got {result:?}");
        }
    }

    #[test]
    fn parse_show_empty() {
        let mut directory = CompanyDirectory::new();
        let user_input = "Show HR";
        let result = directory.parse_command(user_input);
        assert!(matches!(result, CommandResult::Show(None)));
    }
}
