# *The Rust Programming Language*: Exercise Solutions
This repository contains solutions to some of the coding exercises suggested in
[*The Rust Programming Language*](https://doc.rust-lang.org/book/). 

## Included Crates

1. `vec_med_mode`

    Solution to an exercise at end of chapter 8:

    *'Given a list of integers, use a vector and return the median (when sorted, the
    value in the middle position) and mode (the value that occurs most often; a hash
    map will be helpful here) of the list.'*
    
2. `pig_latin`
    
    Solution to an exercise at end of chapter 8:

    *'Convert strings to pig latin. The first consonant of each word is moved to the
    end of the word and ay is added, so first becomes irst-fay. Words that start
    with a vowel have hay added to the end instead (apple becomes apple-hay). Keep
    in mind the details about UTF-8 encoding!'*

3. `add_employee`
    
    Solution to an exercise at end of chapter 8:

    *'Using a hash map and vectors, create a text interface to allow a user to add
    employee names to a department in a company; for example, “Add Sally to
    Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of
    all people in a department or all people in the company by department,
    sorted alphabetically.'*

4. `minigrep`
    
    This is the crate that Chapter 12 of TRPL develops - *'a version of the
    classic command line search tool grep.'*

    It is mostly the same as the code given in the book other than how case
    insensitive search is selected - I use a CLI argument rather than an
    environment variable.
## Usage

Before running any of the CLI tools in this repository, make sure you have [Rust installed](https://www.rust-lang.org/tools/install).  

### Running a crate
From the repository root, you can build and run a specific crate with:

```bash
cargo run -p <CRATE_NAME> [ARGS...]
```

Replace `<CRATE_NAME>` with the name of the crate you want to run, and provide any arguments after it.  

### Viewing documentation
To generate and open the documentation for a crate’s libraries, run:

```bash
cargo doc -p <CRATE_NAME> --open
```

This will build the docs and open them in your default browser.  

Documentation for the CLI tools themselves (rather than their libraries) can be
found in the `README.md` file within each crate’s folder.  

### Running Unit Tests

To execute unit tests on all crates run:

```
cargo test
```
from the root directory. To run unit tests on a particular crate run: 
```
cargo test -p <CRATE_NAME>
```
from the root directoy or run `cargo test` from within the crate's own directory.

### Pre-commit Hook
This repository includes a pre-commit hook that runs `cargo fmt` (to format the
code according to Rust’s style guidelines), `cargo clippy` (to warn of going
against best practices)  and `cargo test`
(to ensure all tests pass) before allowing a commit.

Make sure you have [pre-commit](https://pre-commit.com) installed and the hooks set up by running:
```
pre-commit install
```
This helps keep the codebase consistent and prevents commits that would break tests.