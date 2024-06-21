// See <https://github.com/rust-lang/rust-clippy/issues/11024>
#![allow(clippy::tests_outside_test_module)]

use std::{
    io::{Read, Write},
    process::{Command, Stdio},
};

use assert_cmd::cargo::CommandCargoExt;

/// Simple assertion wrapper around calling the program
fn check(input: &[u8], expected: &[u8]) {
    let mut child = Command::cargo_bin("test-rs")
        .expect("should be able to run binary")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("should be able to spawn process");

    let mut stdin = child.stdin.take().expect("child should have stdin");
    let mut stdout = child.stdout.take().expect("child should have stdin");

    stdin.write_all(input).expect("should be able to write to child stdin");

    // Close stdin so the child stops trying to read from it
    drop(stdin);

    let output = {
        let mut buf = Vec::new();
        stdout
            .read_to_end(&mut buf)
            .expect("should be able to read child stdout");
        buf
    };

    child.wait().expect("child should exit");

    assert_eq!(output, expected, "output should match the expected value");
}

#[test]
fn no_replacements() {
    let input = b"no changes needed";
    let expected = input;

    check(input, expected);
}

#[test]
fn oops_all_replacements() {
    let input = b";;;;;;;;;;;;;;;;;;;;";
    let expected = b"::::::::::::::::::::";

    check(input, expected);
}

#[test]
fn middle_replacement() {
    let input = b"wrong character 2; electric boogaloo";
    let expected = b"wrong character 2: electric boogaloo";

    check(input, expected);
}

#[test]
fn outer_replacements() {
    let cry = b";_;";
    let cropped_spider_face = b":_:";

    check(cry, cropped_spider_face);
}
