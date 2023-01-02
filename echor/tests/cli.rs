use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_no_args(){
    // init a mutable cmd from assert_cmd::Command::cargo_bin
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("SAGE"));
}

#[test]
fn runs(){
    //
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
    cmd.arg("damn").assert().success();
}