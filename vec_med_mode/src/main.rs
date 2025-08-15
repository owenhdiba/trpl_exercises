//! Command-line program to read a comma-separated list of integers from stdin
//! and print the median and mode(s)
use std::io;

use vec_med_mode::{median, mode};

fn compute_stats_and_print(num_vec: Vec<i32>) {
    let median_result = median(&num_vec);
    match median_result {
        Some(v) => println!("The median is {v}"),
        None => println!("The median does not exist."),
    }
    let mode_result = mode(&num_vec);
    match mode_result {
        Some(set) if (set.len() == 1) => println!("The mode is {}", set[0]),
        Some(set) => println!("The modes are {set:?}"),
        None => println!("The mode does not exist."),
    }
}
fn main() {
    // Loop to accept user commands
    loop {
        println!("Please input a list of comma separated integers.");

        let mut data = String::new();

        io::stdin()
            .read_line(&mut data)
            .expect("Failed to read line");

        let parsed_data: Result<Vec<i32>, std::num::ParseIntError> = data
            .trim()
            .split(',')
            .map(|num| num.parse::<i32>())
            .collect();

        match parsed_data {
            Ok(num_vec) => {
                compute_stats_and_print(num_vec);
                break;
            }
            Err(e) => println!("Error reading input: {e}"),
        }
    }
}
