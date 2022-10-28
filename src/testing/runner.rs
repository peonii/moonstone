use crate::testing::test_result::TestResult;
use crate::Error;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::Command;
use std::time::Instant;

#[derive(Serialize, Deserialize, Clone)]
pub struct Test {
    pub input: String,
    pub output: String,
}

impl Test {
    pub fn generate_testcase() -> Result<Self, Error> {
        let mut gen = if cfg!(windows) {
            Command::new(".\\gen.exe")
        } else {
            Command::new("./gen")
        };

        let gen = gen
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .output()?;

        let input = String::from_utf8(gen.stdout)?;

        let mut brute = if cfg!(windows) {
            Command::new(".\\brute.exe")
        } else {
            Command::new("./brute")
        };

        let mut brute = brute
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let brute_stdin = brute.stdin.take();

        if let Some(mut brute_stdin) = brute_stdin {
            brute_stdin.write_all(input.as_bytes())?;
        } else {
            return Err("Failed to write to brute stdin".into());
        }

        let brute_output = brute.wait_with_output()?;

        let output = String::from_utf8(brute_output.stdout)?;

        Ok(Self { input, output })
    }

    pub fn test(&self, tl: u128) -> Result<TestResult, Error> {
        let clock = Instant::now();

        let mut main = if cfg!(windows) {
            Command::new(".\\main.exe")
        } else {
            Command::new("./main")
        };

        let mut main = main
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let main_stdin = main.stdin.take();

        if let Some(mut main_stdin) = main_stdin {
            main_stdin.write_all(self.input.as_bytes())?;
        } else {
            return Err("Failed to write to main stdin".into());
        }

        let main_output = main.wait_with_output()?;

        let output = String::from_utf8(main_output.stdout)?;

        let time = clock.elapsed().as_millis();

        if output.trim() != self.output.trim() {
            return Ok(TestResult::WrongAnswer(output, self.output.clone()));
        }

        if time > tl {
            return Ok(TestResult::Timeout);
        }

        Ok(TestResult::Accepted)
    }
}
