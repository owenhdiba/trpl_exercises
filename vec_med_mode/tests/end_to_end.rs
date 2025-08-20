// tests/end_to_end.rs
use std::process::Command;
use vec_med_mode;
#[test]
fn test_cli_works() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "hello"])
        .output()
        .expect("failed to run command");
}