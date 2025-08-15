A small CLI program that reads a comma-separated list of integers from
standard input, then calculates and prints the median and mode(s).

This is one of the exercises left to the reader at the end of Chapter 8 in *The
Rust Programming Language*.
# Overview
The program repeatedly prompts the user to enter a list of integers until
valid input is provided. It then:
- Parses the input string into a `Vec<i32>`
- Computes the median using `vec_med_mode::median`
- Computes the mode(s) using `vec_med_mode::mode`
- Prints the results in a human-readable format

The program terminates after printing the statistics for the first valid input.

## Example usage
```text
Please input a list of comma separated integers.
1,1,2,2,3,4
The median is 2
The modes are 1, 2
```