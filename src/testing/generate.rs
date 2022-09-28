use colored::Colorize;
use std::fs::File;
use std::io::Write;
use std::process::{Child, Command, Output};
use std::sync::{Arc, Mutex};
use std::{io, path};

use crate::testing::config::{Test, TestConfig};

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

pub async fn generate_testcases() {
    // Get the name of the testcase package
    println!("{}", "Input the name of the testcase package:".yellow());
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("{}", "Input the time limit for one testcase (ms):".yellow());
    // Get the time limit
    let mut time_limit = String::new();
    io::stdin()
        .read_line(&mut time_limit)
        .expect("Failed to read line");

    let time_limit = time_limit.trim().parse::<u32>().unwrap();

    println!(
        "{}",
        "Input the memory limit for one testcase (MB):".yellow()
    );
    // Get the memory limit
    let mut memory_limit = String::new();
    io::stdin()
        .read_line(&mut memory_limit)
        .expect("Failed to read line");

    let memory_limit = memory_limit.trim().parse::<u32>().unwrap();

    println!(
        "{}",
        "Input how many testcases you want to generate:".yellow()
    );
    // Get the memory limit
    let mut testcases_number = String::new();
    io::stdin()
        .read_line(&mut testcases_number)
        .expect("Failed to read line");

    let testcases_number = testcases_number.trim().parse::<u32>().unwrap();

    let mut test_config = TestConfig {
        count: testcases_number,
        time_limit,
        memory_limit,
        tests: Vec::new(),
    };

    // Get the current test
    let cwd = std::env::current_dir().expect("Failed to get current directory");
    let testcase_path = path::Path::new(&cwd).join("tests");

    print!("\n\n");

    // Compile gen.cpp and brute.cpp
    let _gen = Command::new("g++")
        .arg("gen.cpp")
        .arg("-o")
        .arg("gen")
        .output()
        .expect("Failed to compile gen.cpp");

    let _brute = Command::new("g++")
        .arg("brute.cpp")
        .arg("-o")
        .arg("brute")
        .output()
        .expect("Failed to compile brute.cpp");

    let mut handles = vec![];
    // this can't be vec because it's not Copy

    let count = Arc::new(Mutex::new(0));

    for _ in 0..testcases_number {
        let count = Arc::clone(&count);
        handles.push(tokio::spawn(async move {
            let t = generate_testcase().await;
            let mut num = count.lock().unwrap();
            *num += 1;
            let progress_bar =
                "=".repeat(((*num as f32 / test_config.count as f32 * 20.0) - 1.0) as usize);
            let progress_bar_empty = " ".repeat(19 - progress_bar.len());
            print!(
                "\r{} [{}>{}] ({}/{})",
                "⏳ Generating...".blue().bold(),
                progress_bar,
                progress_bar_empty,
                *num,
                test_config.count
            );
            return t;
        }))
    }

    let res = futures::future::join_all(handles).await;
    print!("\n");

    test_config.tests = res.into_iter().map(|x| x.unwrap()).collect();

    // Save the test config
    let test_config = serde_json::to_string_pretty(&test_config).unwrap();
    let mut file = File::create(testcase_path.join(format!("{}.test", name.trim()))).unwrap();
    file.write_all(test_config.as_bytes()).unwrap();

    // Remove gen and brute
    if cfg!(windows) {
        std::fs::remove_file("gen.exe").unwrap();
        std::fs::remove_file("brute.exe").unwrap();
    } else {
        std::fs::remove_file("gen").unwrap();
        std::fs::remove_file("brute").unwrap();
    }

    println!("Testcases generated!");
}
