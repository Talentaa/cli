use std::{
    error::Error,
    fs::{self, File},
    io::Read,
};

use assert_cmd::Command;
use predicates::prelude::*;

const PRG: &str = "headr";
const EMPTY: &str = "./tests/inputs/empty.txt";
const ONE: &str = "./tests/inputs/one.txt";
const TWO: &str = "./tests/inputs/two.txt";
const THREE: &str = "./tests/inputs/three.txt";
const TEN: &str = "./tests/inputs/ten.txt";

type TestResult = Result<(), Box<dyn Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::eq(&expected.as_bytes() as &[u8]));
    Ok(())
}

fn run_stdin(args: &[&str], input_file: &str, expected_file: &str) -> TestResult {
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);
    let input = fs::read_to_string(input_file)?;

    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::eq(&expected.as_bytes() as &[u8]));
    Ok(())
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn empty_c1() -> TestResult {
    run(&[EMPTY, "-c", "1"], "tests/expected/empty.txt.c1.out")
}

#[test]
fn empty_c2() -> TestResult {
    run(&[EMPTY, "-c", "2"], "tests/expected/empty.txt.c2.out")
}

#[test]
fn empty_c4() -> TestResult {
    run(&[EMPTY, "-c", "4"], "tests/expected/empty.txt.c4.out")
}

#[test]
fn empty_n2() -> TestResult {
    run(&[EMPTY, "-n", "2"], "tests/expected/empty.txt.n2.out")
}

#[test]
fn empty_n4() -> TestResult {
    run(&[EMPTY, "-n", "4"], "tests/expected/empty.txt.n4.out")
}

#[test]
fn one() -> TestResult {
    run(&[ONE], "tests/expected/one.txt.out")
}

#[test]
fn one_c1() -> TestResult {
    run(&[ONE, "-c", "1"], "tests/expected/one.txt.c1.out")
}

#[test]
fn one_c2() -> TestResult {
    run(&[ONE, "-c", "2"], "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4() -> TestResult {
    run(&[ONE, "-c", "4"], "tests/expected/one.txt.c4.out")
}

#[test]
fn one_n2() -> TestResult {
    run(&[ONE, "-n", "2"], "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4() -> TestResult {
    run(&[ONE, "-n", "4"], "tests/expected/one.txt.n4.out")
}

#[test]
fn one_stdin() -> TestResult {
    run_stdin(&[], ONE, "tests/expected/one.txt.out")
}

#[test]
fn one_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], ONE, "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], ONE, "tests/expected/one.txt.n4.out")
}

#[test]
fn one_c1_stdin() -> TestResult {
    run_stdin(&["-c", "1"], ONE, "tests/expected/one.txt.c1.out")
}

#[test]
fn one_c2_stdin() -> TestResult {
    run_stdin(&["-c", "2"], ONE, "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4_stdin() -> TestResult {
    run_stdin(&["-c", "4"], ONE, "tests/expected/one.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn two() -> TestResult {
    run(&[TWO], "tests/expected/two.txt.out")
}

#[test]
fn two_n2() -> TestResult {
    run(&[TWO, "-n", "2"], "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4() -> TestResult {
    run(&[TWO, "-n", "4"], "tests/expected/two.txt.n4.out")
}

#[test]
fn two_c2() -> TestResult {
    run(&[TWO, "-c", "2"], "tests/expected/two.txt.c2.out")
}

#[test]
fn two_c4() -> TestResult {
    run(&[TWO, "-c", "4"], "tests/expected/two.txt.c4.out")
}

#[test]
fn two_stdin() -> TestResult {
    run_stdin(&[], TWO, "tests/expected/two.txt.out")
}

#[test]
fn two_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], TWO, "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], TWO, "tests/expected/two.txt.n4.out")
}

#[test]
fn two_c2_stdin() -> TestResult {
    run_stdin(&["-c", "2"], TWO, "tests/expected/two.txt.c2.out")
}

#[test]
fn two_c4_stdin() -> TestResult {
    run_stdin(&["-c", "4"], TWO, "tests/expected/two.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn three() -> TestResult {
    run(&[THREE], "tests/expected/three.txt.out")
}

#[test]
fn three_n2() -> TestResult {
    run(&[THREE, "-n", "2"], "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4() -> TestResult {
    run(&[THREE, "-n", "4"], "tests/expected/three.txt.n4.out")
}

#[test]
fn three_c2() -> TestResult {
    run(&[THREE, "-c", "2"], "tests/expected/three.txt.c2.out")
}

#[test]
fn three_c4() -> TestResult {
    run(&[THREE, "-c", "4"], "tests/expected/three.txt.c4.out")
}

#[test]
fn three_stdin() -> TestResult {
    run_stdin(&[], THREE, "tests/expected/three.txt.out")
}

#[test]
fn three_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], THREE, "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], THREE, "tests/expected/three.txt.n4.out")
}

#[test]
fn three_c2_stdin() -> TestResult {
    run_stdin(&["-c", "2"], THREE, "tests/expected/three.txt.c2.out")
}

#[test]
fn three_c4_stdin() -> TestResult {
    run_stdin(&["-c", "4"], THREE, "tests/expected/three.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn ten() -> TestResult {
    run(&[TEN], "tests/expected/ten.txt.out")
}

#[test]
fn ten_n2() -> TestResult {
    run(&[TEN, "-n", "2"], "tests/expected/ten.txt.n2.out")
}

#[test]
fn ten_n4() -> TestResult {
    run(&[TEN, "-n", "4"], "tests/expected/ten.txt.n4.out")
}

#[test]
fn ten_c2() -> TestResult {
    run(&[TEN, "-c", "2"], "tests/expected/ten.txt.c2.out")
}

#[test]
fn ten_c4() -> TestResult {
    run(&[TEN, "-c", "4"], "tests/expected/ten.txt.c4.out")
}

#[test]
fn ten_stdin() -> TestResult {
    run_stdin(&[], TEN, "tests/expected/ten.txt.out")
}

#[test]
fn ten_n2_stdin() -> TestResult {
    run_stdin(&["-n", "2"], TEN, "tests/expected/ten.txt.n2.out")
}

#[test]
fn ten_n4_stdin() -> TestResult {
    run_stdin(&["-n", "4"], TEN, "tests/expected/ten.txt.n4.out")
}

#[test]
fn ten_c2_stdin() -> TestResult {
    run_stdin(&["-c", "2"], TEN, "tests/expected/ten.txt.c2.out")
}

#[test]
fn ten_c4_stdin() -> TestResult {
    run_stdin(&["-c", "4"], TEN, "tests/expected/ten.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn multiple_files() -> TestResult {
    run(&[EMPTY, ONE, TWO, THREE, TEN], "tests/expected/all.out")
}

#[test]
fn multiple_files_n2() -> TestResult {
    run(
        &[EMPTY, ONE, TWO, THREE, TEN, "-n", "2"],
        "tests/expected/all.n2.out",
    )
}

#[test]
fn multiple_files_n4() -> TestResult {
    run(
        &["-n", "4", EMPTY, ONE, TWO, THREE, TEN],
        "tests/expected/all.n4.out",
    )
}

#[test]
fn multiple_files_c1() -> TestResult {
    run(
        &[EMPTY, ONE, TWO, THREE, TEN, "-c", "1"],
        "tests/expected/all.c1.out",
    )
}

#[test]
fn multiple_files_c2() -> TestResult {
    run(
        &[EMPTY, ONE, TWO, THREE, TEN, "-c", "2"],
        "tests/expected/all.c2.out",
    )
}

#[test]
fn multiple_files_c4() -> TestResult {
    run(
        &["-c", "4", EMPTY, ONE, TWO, THREE, TEN],
        "tests/expected/all.c4.out",
    )
}
