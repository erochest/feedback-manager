use std::process::Command;

use assert_cmd::prelude::*;

#[test]
fn test_run() {
    Command::cargo_bin(env!["CARGO_PKG_NAME"])
        .unwrap()
        .arg("--help")
        .assert()
        .success();
}

/// A test that runs the binary to mark that I have given someone feedback.
#[test]
fn test_feedback() {
    Command::cargo_bin(env!["CARGO_PKG_NAME"])
        .unwrap()
        .args(&["feedback", "Zaphod"])
        .assert()
        .success();
}