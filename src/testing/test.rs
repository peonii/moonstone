use std::io;
use std::io::Write;

use crate::testing::config::{TestConfig};
use std::fs;
use std::process::Command;

pub async fn test_package() {
    println!("Input the name of the testcase package:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    let cwd = std::env::current_dir().expect("Failed to get current directory");
    let path = cwd.join("tests").join(format!("{}.test", name.trim()));

    let test_config_content = fs::read_to_string(path).unwrap_or_else(|err| {
        panic!("Failed to read test file: {}", err);
    });

    let test_config: TestConfig = serde_json::from_str(test_config_content.as_str()).unwrap_or_else(|err| {
        panic!("Failed to parse test file: {}", err);
    });

    println!("Testing {}...", name.trim());

    // Compile the main.cpp algorithm
    let _compile = Command::new("g++")
        .arg("main.cpp")
        .arg("-o")
        .arg("main")
        .output()
        .expect("Failed to compile main.cpp");
    
    let mut handles = Vec::new();

    for i in 0..test_config.count {
        let ind = i;
        let test = test_config.tests[ind as usize].clone();
        handles.push(tokio::spawn(async move {
            let mut main = Command::new("main")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("Failed to run main");
            
            main.stdin.as_mut().unwrap().write_all(test.input.as_bytes()).unwrap();
            let output = main.wait_with_output().expect("Failed to read output");

            let output = String::from_utf8(output.stdout).unwrap();

            if output == test.output {
                println!("Testcase {} passed", ind);
            } else {
                println!("Testcase {} failed", ind);
                println!("Expected: {}", test.output);
                println!("Got: {}", output);
            }
        }))
    }

    futures::future::join_all(handles).await;
}