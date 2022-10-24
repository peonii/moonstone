use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};

use std::{
    fs,
    io::Write,
    process::{Child, Command, Output},
    sync::{Arc, Mutex},
    time::{self, Instant},
};

use crate::{config::file::Config, Error};
use colored::Colorize;

pub enum TestResult {
    Accepted,
    WrongAnswer(String, String),
    Timeout,
}

#[derive(Serialize, Deserialize)]
pub struct TestPackage {
    pub name: String,
    pub tests: Vec<Test>,
    pub time_limit: u128,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Test {
    pub input: String,
    pub output: String,
}

impl TestPackage {
    pub const fn new(name: String, time_limit: u128) -> Self {
        Self {
            name,
            tests: Vec::new(),
            time_limit,
        }
    }

    pub fn add_test(&mut self, input: String, output: String) {
        self.tests.push(Test { input, output });
    }

    /**
    This function generates `amount` number of tests.

    This requires the files `brute(.exe)` and `gen(.exe)` to be present.
     */
    pub async fn generate_tests(&mut self, amount: u32) -> Result<(), Error> {
        println!("{} 📃 Compiling test generators...", "[1/2]".dimmed());
        let config = Config::load()?;

        // Compile the testcase generators
        let gen_compile_args: Vec<&str> = config.gen_compile_command.split(' ').collect();
        let generator = Command::new(&gen_compile_args[0])
            .args(&gen_compile_args[1..])
            .output();

        if let Err(e) = generator {
            return Err(e.into());
        }

        let brute_compile_args: Vec<&str> = config.brute_compile_command.split(' ').collect();
        let brute = Command::new(&brute_compile_args[0])
            .args(&brute_compile_args[1..])
            .output();

        if let Err(e) = brute {
            return Err(e.into());
        }

        println!("{} 🧪 Generating testcases...", "[2/2]".dimmed());

        // Set the progress bar up
        let pb_style = match ProgressStyle::with_template(
            "[{bar:20}] {pos}/{len} ({percent}%) - {eta} remaining...",
        ) {
            Ok(style) => style,
            Err(e) => return Err(e.into()),
        };

        let mut handles = vec![];
        let bar = Arc::new(Mutex::new(ProgressBar::new(amount.into())));

        // This is in braces to unlock the mutex after we modify the style
        {
            let pb = bar.lock();
            if let Ok(pb) = pb {
                pb.set_style(pb_style);
            }
        }

        for _ in 0..amount {
            let b_local = Arc::clone(&bar);
            handles.push(tokio::spawn(async move {
                let t = Test::generate_testcase();
                let prog = b_local.lock();
                if let Ok(prog) = prog {
                    prog.inc(1);
                }
                t
            }));
        }

        let res = futures::future::join_all(handles).await;

        let bar = bar.lock();
        if let Ok(bar) = bar {
            bar.finish();
        }

        for test in res {
            let test_unwrapped = test??; // LMAO
            self.add_test(test_unwrapped.input, test_unwrapped.output);
        }

        if cfg!(windows) {
            std::fs::remove_file("gen.exe")?;
            std::fs::remove_file("brute.exe")?;
        } else {
            std::fs::remove_file("gen")?;
            std::fs::remove_file("brute")?;
        }

        println!("✅ Successfully generated {} testcases!", amount);

        Ok(())
    }

    /**
        Save this testcase to a file in the `tests/` directory.
        The filename is `<name>.json`.
     */
    pub fn save(&self) -> Result<(), Error> {
        let current_dir = std::env::current_dir()?;
        let path = current_dir.join("tests");
        let file_name = format!("{}.json", self.name);

        let file_json = serde_json::to_string_pretty(self)?;
        fs::write(path.join(file_name), file_json)?;
        Ok(())
    }

    /**
        Load a new testcase from a file.
        The file has to be encoded in JSON, and has to be deserializable.
    */
    pub fn load(name: &str) -> Result<Self, Error> {
        let current_dir = std::env::current_dir()?;
        let path = current_dir.join("tests");
        let file_name = format!("{}.json", name);

        let file_json = fs::read_to_string(path.join(file_name))?;
        let test_package: Self = serde_json::from_str(&file_json)?;
        Ok(test_package)
    }

    /**
        Test the current package.

        This does 3 things:
        * compiles the `main` executable (has to be called main)
        * runs each test in the package
        * verifies each test's result

        This is meant as an interactive tester, that's why I included the println! macros.
    */
    pub async fn test(&self) -> Result<(), Error> {
        println!("{} 📃 Compiling program...", "[1/3]".dimmed());
        let mut handles = vec![];
        let config = Config::load()?;

        // Compile the main program, using the first argument as the executable name, and the rest as the args
        let main_compile_command: Vec<&str> = config.main_compile_command.split(' ').collect();
        let main_c = Command::new(&main_compile_command[0])
            .args(&main_compile_command[1..])
            .output();

        if let Err(e) = main_c {
            return Err(e.into());
        }

        println!("{} 🧪 Testing...", "[2/3]".dimmed());

        // Set the progress bar up

        // This is needed, because we need to convert usize to u64, and we can't just do .into()
        let amount = self.tests.len() as u64;

        let pb_style = match ProgressStyle::with_template(
            "[{bar:20}] {pos}/{len} ({percent}%) - {eta} remaining...",
        ) {
            Ok(style) => style,
            Err(e) => return Err(e.into()),
        };

        let bar = Arc::new(Mutex::new(ProgressBar::new(amount)));

        // This is in braces to make sure the lock is dropped just after setting the style
        {
            let pb = bar.lock();
            if let Ok(pb) = &pb {
                pb.set_style(pb_style);
            }
        }

        // Set the tests up
        for test in &self.tests {
            let local_test = test.clone();
            let tl = self.time_limit;
            let b_local = Arc::clone(&bar);

            // Unfortunately, this can't be cleaned up, sorry :p
            handles.push(tokio::spawn(async move {
                let t = local_test.test(tl);
                let prog = b_local.lock();
                if let Ok(prog) = prog {
                    prog.inc(1);
                }
                t
            }));
        }

        let res = futures::future::join_all(handles).await;

        // Finish the bar (required)
        let bar = bar.lock();
        if let Ok(bar) = bar {
            bar.finish();
        }

        println!("{} 🔑 Verifying testcases...", "[3/3]".dimmed());

        // Yes, these two variables are ugly, I know
        let mut i = 1;
        let mut correct = 0;

        let mut log_string = format!("### PACKAGE NAME: {}", self.name);
        let now_timestamp = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH)?;

        log_string += format!("\nTIMESTAMP: {}\n", now_timestamp.as_millis()).as_str();

        // Iterate through each test and verify its status
        for test in res {
            let test_unwrapped = test??; // LMAO

            match test_unwrapped {
                TestResult::Accepted => {
                    correct += 1;

                    log_string += format!("\nTestcase {i} OK").as_str();

                    println!("✅ Testcase #{i}... {}", "ok".green());
                }
                TestResult::WrongAnswer(output, expected) => {
                    println!("❌ Testcase #{i}... {}", "failed".red());

                    log_string += format!("\nTestcase {i} FAIL").as_str();
                    log_string += format!("\n\tExpected {expected}").as_str();
                    log_string += format!("\n\tReceived {output}").as_str();
                }
                TestResult::Timeout => {
                    println!("❌ Testcase #{i} {}... ", "failed".red());
                    println!("\tProgram timed out");

                    log_string += format!("\nTestcase {i} TIMEOUT").as_str();
                }
            }
            i += 1;
        }

        if correct == self.tests.len() {
            println!("{} All testcases {}", "🎉".green(), "passed!".green());
        } else {
            println!(
                "{} {}/{} testcases {}",
                "✅".green(),
                correct,
                self.tests.len(),
                "passed".green()
            );
        }

        println!("Log written to {}-log.txt.", now_timestamp.as_millis());

        std::fs::write(
            format!("{}-log.txt", now_timestamp.as_millis()),
            log_string
        )?;

        if cfg!(windows) {
            std::fs::remove_file("main.exe")?;
        } else {
            std::fs::remove_file("main")?;
        }

        Ok(())
    }
}

impl Test {
    pub fn generate_testcase() -> Result<Self, Error> {
        let gen: Result<Output, std::io::Error>;
        let mut brute: Child;

        if cfg!(windows) {
            gen = Command::new(".\\gen.exe")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .output();
        } else {
            gen = Command::new("./gen")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .output();
        }

        let gen = match gen {
            Ok(gen) => gen,
            Err(e) => return Err(e.into()),
        };

        let input = String::from_utf8(gen.stdout)?;

        if cfg!(windows) {
            brute = Command::new(".\\brute.exe")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()?;
        } else {
            brute = Command::new("./brute")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()?;
        }

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
        let mut main: Child;

        let clock = Instant::now();

        if cfg!(windows) {
            main = Command::new(".\\main.exe")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()?;
        } else {
            main = Command::new("./main")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()?;
        }

        let main_stdin = main.stdin.take();

        if let Some(mut main_stdin) = main_stdin {
            main_stdin.write_all(self.input.as_bytes())?;
        } else {
            return Err("Failed to write to main stdin".into());
        }

        let main_output = main.wait_with_output()?;

        let output = String::from_utf8(main_output.stdout)?;

        let time = clock.elapsed().as_millis();

        if output != self.output {
            return Ok(TestResult::WrongAnswer(output, self.output.clone()));
        }

        if time > tl {
            return Ok(TestResult::Timeout);
        }

        Ok(TestResult::Accepted)
    }
}
