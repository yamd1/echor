use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;
fn run(args: &[&str], expected: &str) -> TestResult {
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected.to_owned());
    Ok(())
}

#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello World!", "-n"], "Hello World!")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello World!"], "Hello World!\n")
}

#[test]
fn hello3() -> TestResult {
    run(&["Hello", "World", "!"], "Hello World !\n")
}

#[test]
fn hello4() -> TestResult {
    run(&["Hello", "World", "!", "-n"], "Hello World !")
}
