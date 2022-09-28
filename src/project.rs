use colored::Colorize;
use std::io::Write;
use std::{fs, io, path};

use crate::dotfiles::config::GlobalConfig;
use crate::Error;

///
/// Creates a new project in path `path`.
///
async fn init_path(path: &path::Path) -> Result<(), Error> {
    if !path.exists() {
        fs::create_dir(path)?;
    }

    let test_path = path.join("tests");
    if !test_path.exists() {
        fs::create_dir(test_path)?;
    }

    let template_repo = GlobalConfig::load().template_repo;
    let template_branch = GlobalConfig::load().template_branch;
    let github_url = format!(
        "https://raw.githubusercontent.com/{}/{}/",
        template_repo, template_branch
    );

    // TODO: Create files
    let main_content = reqwest::get(github_url.to_owned() + "main.cpp")
        .await?
        .text()
        .await?;
    let brute_content = reqwest::get(github_url.to_owned() + "brute.cpp")
        .await?
        .text()
        .await?;
    let gen_content = reqwest::get(github_url.to_owned() + "gen.cpp")
        .await?
        .text()
        .await?;

    let mut main_file = fs::File::create(path.join("main.cpp"))?;
    let mut brute_file = fs::File::create(path.join("brute.cpp"))?;
    let mut gen_file = fs::File::create(path.join("gen.cpp"))?;

    main_file.write_all(main_content.as_bytes())?;
    brute_file.write_all(brute_content.as_bytes())?;
    gen_file.write_all(gen_content.as_bytes())?;

    Ok(())
}

pub async fn new_project() {
    println!("{}", "🖊️ Input the name of the project:".yellow());
    let cwd = std::env::current_dir().expect("Failed to get current directory");

    // Get the name of the project
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // Create the test directory
    let path = path::Path::new(&cwd).join(name.trim());
    init_path(&path)
        .await
        .expect("Failed to initialize project!");

    println!("{} project {}", "Generated".green().bold(), name.bold());
}

pub async fn init_project() {
    // Get the current working directory
    let cwd = std::env::current_dir().expect("Failed to get current directory");

    // Create the test directory
    let path = path::Path::new(&cwd);
    init_path(&path)
        .await
        .expect("Failed to initialize project!");

    println!(
        "{} project in {}",
        "Generated".green().bold(),
        &cwd.display()
    );
}
