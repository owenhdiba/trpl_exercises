//! # Company Directory CLI
//!
//! A simple command-line interface for adding employees to departments
//! and listing employees in a department using text commands.
//!
//! Supported commands:
//! - `Add <Name> to <Department>`
//! - `Show <Department>`
//!
//! Example:
//! ```text
//! > Add Alice to Sales
//! > Show Sales
//! Employees: ["Alice"]
//! ```

use add_employee::{CommandResult, CompanyDirectory};
use std::io;

/// Starts the command-line interface for interacting with the company directory.
///
/// Reads commands from stdin in a loop and prints the result.
fn main() {
    let mut directory = CompanyDirectory::new();

    // Loop to accept user commands
    loop {
        println!("Please input your command.");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let parsed_command: CommandResult = directory.parse_command(&command);

        match parsed_command {
            CommandResult::Add => println!("Successfully added person to department."),
            CommandResult::Show(Some(employees)) => println!("Employees: {employees:?}"),
            CommandResult::Show(None) => println!("Error: department doesn't exist."),
            CommandResult::ParseError(msg) => println!("Error: {msg}"),
        }
    }
}
