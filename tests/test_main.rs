use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::predicate;
use rand::distributions::{Alphanumeric, DistString};

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
    let data_file = assert_fs::NamedTempFile::new("feedback.json").unwrap();
    let name = Alphanumeric.sample_string(&mut rand::thread_rng(), 12);

    Command::cargo_bin(env!["CARGO_PKG_NAME"])
        .unwrap()
        .args(&["--data-file", data_file.path().to_str().unwrap()])
        .args(&["feedback", &name])
        .assert()
        .success();
    // Run Command to check that the feedback has been registered.
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(&["--data-file", data_file.path().to_str().unwrap()])
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&name));
}
