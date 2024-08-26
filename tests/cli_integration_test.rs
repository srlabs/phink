mod shared;

use assert_cmd::Command;
use predicates::prelude::*;
use std::{
    fs,
    path::PathBuf,
};

#[test]
fn test_instrument_command() {

    let output_instrumented_contract: PathBuf = "".parse().unwrap();
    fs::remove_dir(&output_instrumented_contract).unwrap();
    let contract_path = "sample/dummy";

    let mut cmd = Command::cargo_bin("phink").unwrap();
    cmd.arg("instrument")
        .arg(contract_path)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "has been instrumented and compiled",
        ));

    assert!(fs::exists(output_instrumented_contract).unwrap());
}
