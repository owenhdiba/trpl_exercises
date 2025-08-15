A simple implementation of the CLI tool `grep` written in Rust, as walked through
in Chapter 12 of *The Rust Programming Language*. 
## Usage 
To use the tool run
```bash
cargo run -- <QUERY> <INPUT_FILE> [CASE_INSENSITIVE]
```
- `<QUERY>` – The search string to look for in the file. This is required.
- `<INPUT_FILE>` – Path to the file to search. This is required.
- `[CASE_INSENSITIVE]` – Optional flag (`true` to enable) to make the search case-insensitive. If omitted or set to `false`, the search will be case-sensitive.
## Example usage

In this example, `poem.txt` is a sample text file used as a benchmark in the
chapter, and provided in this project's root directory.

```bash
cargo run -- who poem.txt true
```

**Output:**
```
I'm nobody! Who are you?
```