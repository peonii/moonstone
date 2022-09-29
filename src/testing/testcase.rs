use crate::testing::config::Test;
use std::io::Write;
use std::process::{Child, Command, Output};

pub async fn generate_testcase() -> Test {
    // if the operating system is windows, run different commands
    let gen: Output;
    let mut brute: Child;
    if cfg!(windows) {
        // Run gen.cpp
        gen = Command::new(".\\gen.exe")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .output()
            .expect("Failed to run gen.cpp");

        brute = Command::new(".\\brute.exe")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to run brute.cpp");
    } else {
        gen = Command::new("./gen")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .output()
            .expect("Failed to run gen.cpp");

        brute = Command::new("./brute")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to run brute.cpp");
    };
    let input = String::from_utf8(gen.stdout.clone()).unwrap();
    brute
        .stdin
        .take()
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let brute_output = brute.wait_with_output().unwrap();

    // Save gen.cpp to the input of the testcase, and brute.cpp to the output of the testcase
    let test = Test {
        input: String::from_utf8(gen.stdout).unwrap(),
        output: String::from_utf8(brute_output.stdout).unwrap(),
    };

    return test;
}
