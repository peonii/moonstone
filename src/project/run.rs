use crate::Error;
use std::process::Command;
use crate::config::file::Config;

pub fn run_project() -> Result<(), Error> {
    let config = Config::load()?;

    let compile_command: Vec<&str> = config.main_compile_command.split(' ').collect();
    let main_c = Command::new(&compile_command[0])
        .args(&compile_command[1..])
        .output();
    
    if let Err(e) = main_c {
        return Err(e.into());
    }

    let mut main_runner =
        if cfg!(windows) {
            Command::new(".\\main.exe")
        } else {
            Command::new("./main")
        };
        
    let _main_r = main_runner
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .output()?;

    println!("\n\n");
    if cfg!(windows) {
        std::fs::remove_file("main.exe")?;
    } else {
        std::fs::remove_file("main")?;
    }
    
    Ok(())
}
