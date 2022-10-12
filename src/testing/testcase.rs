use indicatif::{ProgressBar, ProgressStyle};
use serde::{Serialize, Deserialize};

use std::{
    process::{Command, Output, Child},
    io::Write, fs, time::Instant, sync::{Mutex, Arc}
};

use crate::Error;
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
    pub time_limit: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Test {
    pub input: String,
    pub output: String
}

impl TestPackage {
    pub fn new(name: String, time_limit: u32) -> Self {
        Self {
            name,
            tests: Vec::new(),
            time_limit,
        }
    }

    pub fn add_test(&mut self, input: String, output: String) {
        self.tests.push(Test {
            input,
            output
        });
    }

    pub async fn generate_tests(&mut self, amount: u32) -> Result<(), Error> {
        println!(
            "{} 📃 Compiling test generators...",
            "[1/2]".dimmed()
        );

        // Compile the testcase generators
        let generator = Command::new("g++")
            .arg("-o")
            .arg("gen")
            .arg("gen.cpp")
            .output();

        if let Err(e) = generator {
            return Err(e.into());
        }

        let brute = Command::new("g++")
            .arg("-o")
            .arg("brute")
            .arg("brute.cpp")
            .output();
        
        if let Err(e) = brute {
            return Err(e.into());
        }

        println!(
            "{} 🧪 Generating testcases...",
            "[2/2]".dimmed()
        );

        let pb_style = match ProgressStyle::with_template("[{bar:20}] {pos}/{len} ({percent}%) - {eta} remaining...") {
            Ok(style) => style,
            Err(e) => return Err(e.into())
        };

        let mut handles = vec![];
        let bar = Arc::new(Mutex::new(
               ProgressBar::new(amount.into())
        ));

        let pb = bar.lock();
        if let Ok(pb) = pb {
            pb.set_style(pb_style);
        }

        for _ in 0..amount {
            let b_local = Arc::clone(&bar);
            handles.push(
                tokio::spawn(async move {
                    let t = Test::generate_testcase();
                    let prog = b_local.lock();
                    if let Ok(prog) = prog {
                        prog.inc(1);
                    }
                    return t;
                })
            );
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
    
    pub fn save(&self) -> Result<(), Error> {
        let current_dir = std::env::current_dir()?;
        let path = current_dir.join("tests");
        let file_name = format!("{}.json", self.name);

        let file_json = serde_json::to_string_pretty(self)?;
        fs::write(path.join(file_name), file_json)?;
        Ok(())
    }

    pub fn load(name: String) -> Result<Self, Error> {
        let current_dir = std::env::current_dir()?;
        let path = current_dir.join("tests");
        let file_name = format!("{}.json", name);
        let file_json = fs::read_to_string(path.join(file_name))?;
        let test_package: Self = serde_json::from_str(&file_json)?;
        Ok(test_package)
    }

    pub async fn test<'a>(&'a self) -> Result<(), Error> {
        println!(
            "{} 📃 Compiling program...",
            "[1/3]".dimmed()
        );
        let mut handles = vec![];

        let main_c = Command::new("g++")
            .arg("-o")
            .arg("main")
            .arg("main.cpp")
            .output();

        if let Err(e) = main_c {
            return Err(e.into());
        }

        println!(
            "{} 🧪 Testing...",
            "[2/3]".dimmed()
        );

        let amount = self.tests.len() as u64;

        let pb_style = match ProgressStyle::with_template("[{bar:20}] {pos}/{len} ({percent}%) - {eta} remaining...") {
            Ok(style) => style,
            Err(e) => return Err(e.into())
        };

        let bar = Arc::new(Mutex::new(
               ProgressBar::new(amount)
        ));

        let pb = bar.lock();
        if let Ok(pb) = pb {
            pb.set_style(pb_style);
        }

        for test in &self.tests {
            let local_test = test.clone();
            let tl = self.time_limit;
            let b_local = Arc::clone(&bar);
            handles.push(
                tokio::spawn(async move {
                    let t = local_test.test(tl);
                    let prog = b_local.lock();
                    if let Ok(prog) = prog {
                        prog.inc(1);
                    }
                    return t;
                })
            );
        }

        let res = futures::future::join_all(handles).await;

        let bar = bar.lock();
        if let Ok(bar) = bar {
            bar.finish();
        }

        println!(
            "{} 🔑 Verifying testcases...",
            "[3/3]".dimmed()
        );
        
        let mut i = 1;
        let mut correct = 0;
        for test in res {
            let test_unwrapped = test??; // LMAO

            match test_unwrapped {
                TestResult::Accepted => {
                    correct += 1;
                }
                TestResult::WrongAnswer(output, expected) => {
                    println!("❌ Testcase #{i} {}: ", "failed".red());
                    println!("\tWrong answer");
                    println!("\tExpected: {expected}",);
                    println!("\tGot: {output}");
                }
                TestResult::Timeout => {
                    println!("❌ Testcase #{i} {}: ", "failed".red());
                    println!("\tProgram timed out");
                }
            } 
            i += 1;
        }

        if correct != self.tests.len() {
            println!("{} {}/{} testcases {}", "✅".green(), correct, self.tests.len(), "passed".green());
        } else {
            println!("{} All testcases {}", "🎉".green(), "passed!".green());
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
            Err(e) => return Err(e.into())
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

        Ok(Self {
            input,
            output
        })
    }

    // TODO: Add memory limit
    pub fn test(&self, tl: u32) -> Result<TestResult, Error> {
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
        let time = time as u32;

        if output != self.output {
            return Ok(TestResult::WrongAnswer(output.clone(), self.output.clone()));
        }

        if time > tl {
            return Ok(TestResult::Timeout);
        }

        Ok(TestResult::Accepted)
    }
}