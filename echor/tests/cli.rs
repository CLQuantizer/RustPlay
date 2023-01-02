use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    // init a mutable cmd from assert_cmd::Command::cargo_bin
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("SAGE"));
    Ok(())
}

#[test]
fn runs()->TestResult{
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
    cmd.arg("damn").assert().success();
    Ok(())
}

#[test]
fn compare()->TestResult{
    let outfile = "test/expected/hello1.n.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    //be careful there is no \n at the end of hello1.n.txt
    cmd
        .arg("-n")
        .arg("Hello  there").assert().success().stdout(expected);

    Ok(())
    }