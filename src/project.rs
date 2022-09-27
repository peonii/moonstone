use std::{fs, io, path};

///
/// Creates a new project in path `path`.
///
fn init_path(path: &path::Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir(path)?;
    }

    let test_path = path.join("tests");
    if !test_path.exists() {
        fs::create_dir(test_path)?;
    }

    fs::create_dir(path.join("main.cpp"))?;
    fs::create_dir(path.join("gen.cpp"))?;
    fs::create_dir(path.join("brute.cpp"))?;

    Ok(())
}

pub fn new_project() {
    println!("Input the name of the project:");

    // Get the name of the project
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // Create the test directory
    let path = path::Path::new(&name);
    init_path(&path).expect("Failed to initialize project!");

    println!("Generated project {}", name);
}

pub fn init_project() {
    // Get the current working directory
    let cwd = std::env::current_dir().expect("Failed to get current directory");

    // Create the test directory
    let path = path::Path::new(&cwd);
    init_path(&path).expect("Failed to initialize project!");

    println!("Generated project in {}", &cwd.display());
}
