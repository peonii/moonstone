use crate::config::file::Config;
use crate::replace::replacer::Replacer;
use crate::{cwd, Error};
use std::process::Command;

pub fn run_project() -> Result<(), Error> {
    let config = Config::load()?;

    //let mut current_path = cwd!()?;

    //current_path.push("main.alg");

    //let mut new_path = cwd!()?;

    //new_path.push("main.cpp");

    //Replacer::from_path(&current_path)?.replace_to_file(&new_path)?;

    let compile_command: Vec<&str> = config.main_compile_command.split(' ').collect();
    let main_c = Command::new(&compile_command[0])
        .args(&compile_command[1..])
        .output();

    if let Err(e) = main_c {
        return Err(e.into());
    }

    let mut main_runner = if cfg!(windows) {
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
