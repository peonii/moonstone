use std::io;
use std::io::Write;
use std::sync::{Arc, Mutex};

use crate::testing::config::TestConfig;
use colored::Colorize;
use std::fs::File;
use std::process::{Child, Command};

pub async fn test_package() {
    println!("{}", "Input the name of the testcase package:".yellow());
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let cwd = std::env::current_dir().expect("Failed to get current directory");
    let path = cwd.join("tests").join(format!("{}.test", name.trim()));
    let test_config_file = File::open(path).expect("Failed to open test config file");

    let test_config: TestConfig = serde_json::from_reader(test_config_file)
        .unwrap_or_else(|err| {
            panic!("Failed to parse test file: {}", err);
        });

    // Compile the main.cpp algorithm
    let _compile = Command::new("g++")
        .arg("main.cpp")
        .arg("-o")
        .arg("main")
        .output()
        .expect("Failed to compile main.cpp");

    let mut handles = Vec::new();
    let testcases = Arc::new(Mutex::new(vec![false; test_config.count as usize]));
    let tested = Arc::new(Mutex::new(0));
    print!("\n\n");

    for i in 0..test_config.count {
        let ind = i;
        let test = test_config.tests[ind as usize].clone();
        let tcs = Arc::clone(&testcases);
        let tested = Arc::clone(&tested);
        handles.push(tokio::spawn(async move {
            let mut main: Child;

            if cfg!(windows) {
                main = Command::new("main")
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::piped())
                    .spawn()
                    .expect("Failed to run main");
            } else {
                main = Command::new("./main")
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::piped())
                    .spawn()
                    .expect("Failed to run main");
            }

            main.stdin
                .as_mut()
                .unwrap()
                .write_all(test.input.as_bytes())
                .unwrap();
            let output = main.wait_with_output().expect("Failed to read output");

            let output = String::from_utf8(output.stdout).unwrap();

            if output == test.output {
                let mut local_tcs = tcs.lock().unwrap();

                local_tcs[ind as usize] = true;
            }

            let mut num = tested.lock().unwrap();
            *num += 1;

            let progress_bar =
                "=".repeat(((*num as f32 / test_config.count as f32 * 20.0) - 1.0) as usize);
            let progress_bar_empty = "∙".repeat(19 - progress_bar.len());
            print!(
                "\r{} [{}>{}] ({}/{})",
                "⏳ Testing".blue().bold(),
                progress_bar,
                progress_bar_empty,
                *num,
                test_config.count
            );
        }))
    }

    futures::future::join_all(handles).await;

    let testcases = testcases.lock().unwrap();

    let mut passed = 0;
    for i in 0..test_config.count {
        if testcases[i as usize] {
            passed += 1;
        }
    }

    print!("\r{}", " ".repeat(100));

    print!(
        "\r{} {}/{} testcases",
        "Passed".green().bold(),
        passed,
        test_config.count
    );

    if passed == test_config.count {
        print!("\r{}", " ".repeat(100));
        print!("\r✅ All testcases passed!");
    }

    for i in 0..test_config.count {
        if !testcases[i as usize] {
            print!("\n❌ Testcase {} {}", i, "failed".red());
        }
    }

    if cfg!(windows) {
        std::fs::remove_file("main.exe").unwrap();
    } else {
        std::fs::remove_file("main").unwrap();
    }

    print!("\n");
}
